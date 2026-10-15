import { z } from 'zod'
import { syntheticConfigurationSchema } from './syntheticConfiguration'
import { identifiedEstimatorSettingsSchema } from '@/domain/identifiedEstimatorSettings'
import { timeSeriesEstimatorSettingsSchema } from '@/domain/timeSeriesEstimatorSettings'
import { runCodeActLoop } from './octopus/rlm/codeactLoop'
import { asNodeId, type Citation } from './octopus/rlm/types'
import { isNonEmpty, assertNever } from './octopus/dop'
import { openRouterModel } from './model'
import {
  budgetWarning,
  type BudgetProgress,
  type ProviderUsage,
  type ProviderFailure,
} from './telemetry'
import { assistantPrompt } from './prompt'
import { assistantLimits, DEFAULT_LIMITS, QUESTION_DEADLINE_MS } from './limits'
import { priorTaskRecords, type PriorTask } from './octopus/rlm/priorTasks'
import { pythonSandbox } from './sandbox'
import type { SelectionProposal, SqlProposal, ProposalProblem } from './applicationTools'
import type { Result } from '@/domain/dop'
import type { AssistantRunOutcome } from './analysisRun'
import { runSubjectSchema, type RunSubject } from './analysisRun'
import { MEMORY_TOOLS, type MemoryAccess } from './memory/access'
import { pipelineSchema, type PipelineProposal } from './preparation'
import { graphProposalSchema, type GraphProposal } from './graphProposal'
import { studyProposalSchema, type StudyProposal } from './studyProposal'
import type { StudySelectionProposal } from './studySelection'
import type { StudyRead } from './studyRead'
import type { RunRead } from './runRead'
import { panelConfigurationSchema } from './panelConfiguration'

export const answerSchema = z.object({
  answer: z.string().min(1).max(12000),
  evidence: z
    .array(
      z.enum([
        'project-summary',
        'user-question',
        'approved-run',
        'conversation-memory',
        'data-summary',
        'study-record',
        'saved-run',
      ]),
    )
    .min(1),
})
const runSchema = z
  .object({
    key: z.string().min(1).max(100),
    reason: z.string().trim().min(1).max(2000),
    subject: runSubjectSchema.optional(),
  })
  .strict()
const tasksSchema = z.object({ tasks: z.array(z.string().min(1).max(2000)).min(1).max(2) })
export type Answer = z.infer<typeof answerSchema>
type TraceMessageKind =
  'task' | 'request' | 'writing' | 'python' | 'output' | 'tool' | 'answer' | 'failed'
export type Trace = {
  readonly agent: string
  readonly text: string
} & (
  | { readonly kind: TraceMessageKind }
  | { readonly kind: 'budget'; readonly requestId: number; readonly budget: BudgetProgress }
  | { readonly kind: 'usage'; readonly requestId: number; readonly usage: ProviderUsage }
  | {
      readonly kind: 'provider-failure'
      readonly requestId: number
      readonly failure: ProviderFailure
    }
)
interface Options {
  readonly key: string
  readonly model: string
  readonly question: string
  readonly context: string
  readonly readProject?: () => string
  readonly limits?: z.input<typeof assistantLimits>
  readonly history?: readonly PriorTask[]
  readonly memory?: MemoryAccess
  readonly readStudy?: (raw: unknown) => StudyRead
  readonly readRun?: (raw: unknown) => RunRead
  readonly profile?: (
    raw: unknown,
    signal: AbortSignal,
  ) => Promise<{ readonly kind: 'profiled' | 'refused' }>
  readonly proposePipeline?: (raw: unknown) => Result<PipelineProposal, string>
  readonly proposeGraph?: (raw: unknown) => Result<GraphProposal, string>
  readonly proposeStudy?: (raw: unknown) => Result<StudyProposal, string>
  readonly proposeStudySelection?: (raw: unknown) => Result<StudySelectionProposal, string>
  readonly workspaceSettings?: (raw: unknown) => unknown
  readonly proposeWorkspace?: (raw: unknown) => Result<unknown, string>
  readonly signal: AbortSignal
  readonly trace: (event: Trace) => void
  readonly propose: ((raw: unknown) => Result<SelectionProposal, ProposalProblem>) | null
  readonly proposeSql: ((raw: unknown) => Result<SqlProposal, ProposalProblem>) | null
  readonly requestRun: (
    key: string,
    reason: string,
    signal: AbortSignal,
    subject?: RunSubject,
    waiting?: (pending: boolean) => void,
  ) => Promise<AssistantRunOutcome>
}
export async function runAssistant(options: Options): Promise<Answer> {
  const parsedLimits = assistantLimits.safeParse(options.limits ?? DEFAULT_LIMITS)
  if (!parsedLimits.success) throw new Error('Choose valid assistant step and request limits.')
  const limits = parsedLimits.data
  const recalled = priorTaskRecords((options.history ?? []).slice(-6))
  const controller = new AbortController()
  const startedAt = Date.now()
  let waitingStarted: number | null = null
  let waitedMs = 0
  const deadline = activeTimer(QUESTION_DEADLINE_MS, () =>
    controller.abort(
      new Error(
        'The assistant reached the time limit for this question. Check the latest saved results before continuing.',
      ),
    ),
  )
  const signal = AbortSignal.any([options.signal, controller.signal])
  let requests = 0
  let children = 0
  let proposals = 0
  let requestedRun = false
  let latestAnalysisThisQuestion: AssistantRunOutcome | null = null
  const run = async (question: string, agent: string, child: boolean): Promise<Answer> => {
    const result: { answer: Answer | null } = { answer: null }
    let inspected = false
    let completedRun = false
    let readMemory = false
    let readProfile = false
    let readStudyRecord = false
    let readSavedRun = false
    let analysisFeedback: AssistantRunOutcome | null = null
    const trace = (kind: TraceMessageKind, text: string) => options.trace({ agent, kind, text })
    if (child) trace('task', question)
    const runtime = await pythonSandbox(signal, async (name, args) => {
      signal.throwIfAborted()
      trace('tool', name)
      if (MEMORY_TOOLS.some((tool) => tool === name) && options.memory) {
        const recalled = await options.memory.read(name, args, signal)
        readMemory = true
        return recalled
      }
      if (name === 'project') {
        inspected = true
        return {
          latestAnalysisThisQuestion,
          availableWorkspaceSettings: options.workspaceSettings?.({}) ?? [],
          ...JSON.parse(options.readProject?.() ?? options.context),
        }
      }
      if (name === 'learner_schema' && options.propose) return learnerConfigurationSchema()
      if (name === 'synthetic_control_schema' && options.propose)
        return z.toJSONSchema(syntheticConfigurationSchema, { io: 'input' })
      if (name === 'identified_estimator_schema' && options.propose)
        return z.toJSONSchema(identifiedEstimatorSettingsSchema, { io: 'input' })
      if (name === 'time_series_estimator_schema' && options.propose)
        return z.toJSONSchema(timeSeriesEstimatorSettingsSchema, { io: 'input' })
      if (name === 'panel_did_schema' && options.propose)
        return z.toJSONSchema(panelConfigurationSchema)
      if (name === 'study' && options.readStudy) {
        const record = options.readStudy(args)
        if (record.kind === 'study-record') readStudyRecord = true
        return record
      }
      if (name === 'run' && options.readRun) {
        const record = options.readRun(args)
        if (record.kind === 'saved-run' || record.kind === 'saved-calculation') readSavedRun = true
        return record
      }
      if (name === 'profile_table' && options.profile) {
        const summary = await options.profile(args, signal)
        if (summary.kind === 'profiled') readProfile = true
        return summary
      }
      if (name === 'pipeline_schema' && options.proposePipeline) {
        return pipelineSchema(args ?? {})
      }
      if (name === 'graph_schema' && options.proposeGraph)
        return z.toJSONSchema(graphProposalSchema)
      if (name === 'study_schema' && options.proposeStudy)
        return z.toJSONSchema(studyProposalSchema)
      if (name === 'workspace_settings' && options.workspaceSettings)
        return options.workspaceSettings(args)
      if (name === 'propose_workspace' && !child && options.proposeWorkspace) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const proposed = options.proposeWorkspace(args)
        return proposed.ok
          ? {
              kind: 'awaiting-user-review',
              message:
                'Settings proposed, not applied. After approval, inspect the workspace in a new question before requesting a run.',
            }
          : { kind: 'refused', reason: proposed.error }
      }
      if (name === 'propose_study_selection' && !child && options.proposeStudySelection) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const proposed = options.proposeStudySelection(args)
        return proposed.ok
          ? {
              kind: 'awaiting-user-review',
              message:
                'Study selection proposed, not applied. After approval, inspect the selected study and method in a new question before running.',
            }
          : { kind: 'refused', reason: proposed.error }
      }
      if (name === 'propose_study' && !child && options.proposeStudy) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const proposed = options.proposeStudy(args)
        return proposed.ok
          ? {
              kind: 'awaiting-user-review',
              message: 'Study draft proposed, not applied or identified.',
            }
          : { kind: 'refused', reason: proposed.error }
      }
      if (name === 'propose_graph' && !child && options.proposeGraph) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const proposed = options.proposeGraph(args)
        return proposed.ok
          ? { kind: 'awaiting-user-review', message: 'Graph changes proposed, not saved.' }
          : { kind: 'refused', reason: proposed.error }
      }
      if (name === 'propose_pipeline' && !child && options.proposePipeline) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const proposed = options.proposePipeline(args)
        return proposed.ok
          ? {
              kind: 'awaiting-user-review',
              message: 'Pipeline draft prepared, not applied or evaluated.',
            }
          : { kind: 'refused', reason: proposed.error }
      }
      if (name === 'run_analysis' && !child) {
        if (requestedRun)
          return { kind: 'refused', reason: 'Only one analysis may be requested per question.' }
        const parsed = runSchema.safeParse(args)
        if (!parsed.success) return { kind: 'refused', reason: 'Supply a run key and a reason.' }
        requestedRun = true
        const result = await options.requestRun(
          parsed.data.key,
          parsed.data.reason,
          signal,
          parsed.data.subject,
          (pending) => {
            if (pending && waitingStarted === null) {
              waitingStarted = Date.now()
              deadline.pause()
            } else if (!pending && waitingStarted !== null) {
              waitedMs += Date.now() - waitingStarted
              waitingStarted = null
              deadline.resume()
            }
          },
        )
        completedRun =
          result.kind === 'completed' ||
          result.kind === 'calculated' ||
          result.kind === 'review-required'
        latestAnalysisThisQuestion = result
        analysisFeedback = result
        return result
      }
      if (name === 'propose_sql' && !child && options.proposeSql !== null) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const result = options.proposeSql(args)
        return result.ok
          ? { kind: 'awaiting-user-review', message: 'SQL draft prepared, not executed.' }
          : { kind: 'refused', reason: result.error }
      }
      if (name === 'propose_estimator' && !child && options.propose !== null) {
        if (++proposals > 3) throw new Error('The proposal limit was reached.')
        const result = options.propose(args)
        return result.ok
          ? { kind: 'awaiting-user-approval', estimator: result.value.choice.name }
          : typeof result.error === 'object'
            ? {
                kind: 'refused',
                reason: result.error.violations.map((item) => item.evidence).join(' '),
                requirements: result.error.violations,
              }
            : { kind: 'refused', reason: result.error }
      }
      if (name === 'finish') {
        const parsed = answerSchema.parse(args)
        result.answer = parsed
        return { accepted: true }
      }
      if (name === 'delegate' && !child) {
        const { tasks } = tasksSchema.parse(args)
        if (children + tasks.length > 2) throw new Error('Subagent budget exhausted.')
        const first = children
        children += tasks.length
        return Promise.all(
          tasks.map(async (task, i) => {
            try {
              const answer = await run(task, `Subagent ${first + i + 1}`, true)
              return { kind: 'answered', ...answer }
            } catch (error) {
              signal.throwIfAborted()
              return {
                kind: 'failed',
                reason: error instanceof Error ? error.message : 'Subagent failed.',
              }
            }
          }),
        )
      }
      throw new Error('This tool is not available.')
    })
    const system = assistantPrompt({
      role: child ? 'child' : 'parent',
      limits,
      memory: options.memory !== undefined,
      profile: options.profile !== undefined,
      studyRecords: options.readStudy !== undefined,
      savedRuns: options.readRun !== undefined,
      workspaceSettings: options.workspaceSettings !== undefined,
      proposals: [
        ...(options.propose ? ['estimator' as const] : []),
        ...(options.proposeSql ? ['sql' as const] : []),
        ...(options.proposePipeline ? ['pipeline' as const] : []),
        ...(options.proposeGraph ? ['graph' as const] : []),
        ...(options.proposeStudy ? ['study' as const] : []),
        ...(options.proposeStudySelection ? ['study-selection' as const] : []),
      ],
    })
    let iteration = 0
    let requestId = 0
    const callModel = openRouterModel(
      options.key,
      options.model,
      () => {
        if (requests >= limits.maxRequests)
          throw new Error(`The shared limit of ${limits.maxRequests} model requests was reached.`)
        requestId = ++requests
        trace('request', `Model request ${++iteration}`)
      },
      (reason) => trace('output', reason),
      {
        usage: (usage) =>
          options.trace({ agent, kind: 'usage', text: 'Provider usage', requestId, usage }),
        failure: (failure) =>
          options.trace({
            agent,
            kind: 'provider-failure',
            text: failure.message,
            requestId,
            failure,
          }),
      },
    )
    try {
      const session = await runCodeActLoop(
        recalled
          ? `Earlier conversation in this project, oldest first. Retain the user's stated choices and unresolved questions across page changes; later explicit corrections replace earlier choices. Revalidate column choices against the current source and read saved records before repeating work. Earlier assistant answers are recollections, not verified evidence. No run approval carries forward. Records may be shortened: use DAG_HISTORY or DAG_GREP for the user's original words rather than asking them to repeat a settled choice.\n${recalled}\n\nCurrent question:\n${question}`
          : question,
        {
          system,
          signal,
          maxIters: limits.maxIters,
          maxTokens: limits.maxTokens,
          callModel: (request) => {
            signal.throwIfAborted()
            if (requests >= limits.maxRequests)
              throw new Error(
                `The shared limit of ${limits.maxRequests} model requests was reached.`,
              )
            const budget: BudgetProgress = {
              step: request.messages.filter((message) => message.role === 'assistant').length + 1,
              maxSteps: limits.maxIters,
              requests: requests + 1,
              maxRequests: limits.maxRequests,
              elapsedMs:
                Date.now() -
                startedAt -
                waitedMs -
                (waitingStarted === null ? 0 : Date.now() - waitingStarted),
              deadlineMs: QUESTION_DEADLINE_MS,
              outputTokenAllowance: request.maxTokens,
            }
            const warning = budgetWarning(budget)
            options.trace({
              agent,
              kind: 'budget',
              text: warning ?? 'Execution budget',
              requestId: requests + 1,
              budget,
            })
            return callModel({
              ...request,
              messages: warning
                ? [...request.messages, { role: 'user', content: warning }]
                : request.messages,
            })
          },
          acquireExecutor: async () => ({
            execute: async (code) => {
              analysisFeedback = null
              const executed = await runtime.executeReport(code)
              return {
                stdout: [
                  executed.output,
                  analysisFeedback
                    ? `Analysis outcome recorded by Hirmos: ${JSON.stringify(analysisFeedback)}`
                    : '',
                ]
                  .filter(Boolean)
                  .join('\n'),
                stderr: executed.stderr,
                exit_code: executed.exitCode,
                answer: {
                  ready: result.answer !== null,
                  content: result.answer?.answer ?? '',
                  citations: result.answer?.evidence,
                },
                locals: executed.locals,
              }
            },
            resetAnswer: () => {
              result.answer = null
            },
          }),
          validateCitations: async (raw) => {
            const parsed = answerSchema.shape.evidence.safeParse(raw)
            if (!parsed.success) return { kind: 'uncited' }
            const citations: Citation[] = parsed.data.map((id) => ({
              nodeId: asNodeId(id),
              label: id,
            }))
            const invalid = citations.filter(
              ({ label }) =>
                (label === 'project-summary' && !inspected) ||
                (label === 'approved-run' && !completedRun) ||
                (label === 'conversation-memory' && !readMemory) ||
                (label === 'data-summary' && !readProfile) ||
                (label === 'study-record' && !readStudyRecord) ||
                (label === 'saved-run' && !readSavedRun),
            )
            if (isNonEmpty(invalid)) return { kind: 'rejected', invalid }
            return isNonEmpty(citations) ? { kind: 'grounded', citations } : { kind: 'uncited' }
          },
          guidance: {
            finish:
              'Continue with one Python block, or await tool("finish", {"answer":"...", "evidence":["project-summary"]}).',
            uncited:
              'ANSWER REJECTED: supply evidence from tools you have read or user-question. Study-record requires a successful study read; saved-run requires a successful run read; approved-run requires returned evidence from a user-approved computation.',
            rejected:
              'ANSWER REJECTED: you cited evidence you have not received. Inspect project for project-summary; read a saved study successfully for study-record or a saved run or calculation for saved-run. Cite approved-run only after this agent receives completed, calculated or review-required evidence from a user-approved computation. Review-required is not a saved result. A proposal is not a computation.',
          },
          onState: (state) => {
            switch (state.kind) {
              case 'writingCode': {
                const draft = /```(?:python|py)[ \t]*\r?\n([\s\S]*)/i.exec(state.text)
                if (draft) trace('writing', draft[1].replace(/\n```[\s\S]*$/, ''))
                return
              }
              case 'wroteCode':
                trace('python', state.code)
                return
              case 'ranCode':
                trace(
                  'output',
                  [state.result.stdout, state.result.stderr].filter(Boolean).join('\n') ||
                    '(no printed output)',
                )
                return
              case 'done':
                trace('answer', 'Answer ready')
                return
              case 'error':
                if (!signal.aborted) trace('failed', state.message)
                return
              case 'mcpResult':
                if (!state.ok) trace('output', state.summary)
                return
              case 'idle':
              case 'thinking':
              case 'wroteCypher':
              case 'executing':
              case 'observed':
              case 'mcpCall':
              case 'doneComputed':
              case 'notFound':
                return
              default:
                return assertNever(state)
            }
          },
        },
      )
      signal.throwIfAborted()
      if (session.kind === 'terminated' && session.outcome.kind === 'answered') {
        return answerSchema.parse({
          answer: session.outcome.answer,
          evidence: session.outcome.citations.map((c) => c.nodeId),
        })
      }
      throw new Error(
        session.kind === 'terminated' && session.outcome.kind === 'failed'
          ? session.outcome.message
          : 'The assistant could not provide a supported answer.',
      )
    } finally {
      runtime.close()
    }
  }
  try {
    return await run(options.question, 'Assistant', false)
  } finally {
    deadline.stop()
    controller.abort()
  }
}
import { learnerConfigurationSchema } from './estimatorConfiguration'
import { activeTimer } from './activeTimer'
