import { useEffect, useRef, useState } from 'react'
import {
  proposeWorkspaceConfiguration,
  readWorkspaceSettings,
  WorkspaceConfigurationCard,
  type WorkspaceConfigurationProposal,
} from './workspaceConfiguration'
import { useWorkflow, useWorkflowStore } from '@/components/WorkflowProvider'
import { graphBinding, proposeGraph, type GraphProposal } from './graphProposal'
import { GraphProposalCard } from './GraphProposalCard'
import { studyBinding, proposeStudy, type StudyProposal } from './studyProposal'
import { StudyProposalCard } from './StudyProposalCard'
import { cn } from '@/lib/utils'
import { Icon } from '@/components/Icon'
import { iconControl, label } from '@/components/ui/recipes'
import { assistantContext } from './context'
import { readStudy } from './studyRead'
import { readRun } from './runRead'
import { configurationSettings } from '@/domain/estimation'
import { ProposalActivity, ProposalActions, ProposalCard, ProposalSettings } from './ProposalParts'
import { appendTrace } from './traceCells'
import { runAssistant } from './agent'
import { assistantFailureMessage } from './failureMessage'
import { CodeHighlight } from '@/components/ui/CodeHighlight'
import {
  useStudySelection,
  proposeStudySelection,
  StudySelectionCard,
  type StudySelectionProposal,
} from './studySelection'
import { conversationRecords } from './conversation'
import { assistantLimits, QUESTION_DEADLINE_MS } from './limits'
import {
  useAssistantConnection,
  configureAssistantConnection,
  approveAssistantProject,
  forgetAssistantConnection,
} from './connectionStore'
import { readKeyCredit, type CreditStatus } from './budget'
import { beginMemoryTask, loadMemoryTurns } from './memory/access'
import { getActiveSessionId, rotateSession } from './memory/session'
import { clearProjectMemory } from './memory/dagStore'
import {
  clearDisplayHistory,
  loadDisplayTurns,
  mergeTurnHistory,
  saveDisplayTurn,
} from './displayHistory'
import { Composer } from './Composer'
import {
  proposeSelection,
  useAssistantBridge,
  useAssistantCapability,
  type SelectionProposal,
  type ProposalProblem,
  proposeSql,
  useSqlCapability,
  type SqlProposal,
  useRunCapabilities,
} from './applicationTools'
import { RunApprovalCards, useRunApproval } from './RunApproval'
import { err } from '@/domain/dop'
import { EligibilityView } from '@/components/EligibilityView'
import { SqlProposalCard } from './SqlProposalCard'
import { PipelineProposalCard } from './PipelineProposalCard'
import { proposePipeline, type PipelineProposal } from './preparation'
import { usePreparationCapability } from './applicationTools'
import { AssistantMark } from './AssistantMark'
import { ConnectionBar } from './AssistantConnection'
import { TurnView, type Turn } from './TurnView'
import {
  askAssistant,
  clearAssistantSeed,
  closeAssistant,
  dockAssistant,
  floatAssistant,
  setAssistantRunning,
  toggleAssistantExpanded,
  useAssistantExpanded,
  useAssistantPlacement,
  useAssistantSeed,
} from './assistantStore'
import { ASSISTANT_DRAG } from './AssistantFloat'

export default function AssistantPanel() {
  const workflow = useWorkflow((state) => state.workflow)
  const capability = useAssistantCapability()
  const sql = useSqlCapability()
  const runs = useRunCapabilities()
  const preparation = usePreparationCapability()
  const studySelection = useStudySelection()
  const context = assistantContext(
    workflow,
    capability?.inspection ?? null,
    sql?.inspection ?? null,
    Object.fromEntries(Object.entries(runs).map(([key, value]) => [key, value.inspection])),
    preparation?.inspection ?? null,
    studySelection?.inspection ?? null,
  )
  const project = 'project' in workflow ? workflow.project.id : 'none'
  return <Session key={project} project={project} context={context} />
}

function Starters({
  estimation,
  sql,
  onAsk,
}: {
  readonly estimation: boolean
  readonly sql: boolean
  readonly onAsk: (question: string) => void
}) {
  const starters = [
    { icon: 'dataset', text: 'What does this project contain?' },
    { icon: 'help_outline', text: 'What information would an analysis need?' },
    ...(estimation ? [{ icon: 'tune', text: 'Which estimator fits this study, and why?' }] : []),
    ...(sql
      ? [{ icon: 'sql', text: 'Which transformation would prepare this data for analysis?' }]
      : []),
  ]
  return (
    <div className="flex flex-col gap-0.5">
      {starters.map((starter) => (
        <button
          key={starter.text}
          type="button"
          onClick={() => onAsk(starter.text)}
          className="group flex w-full items-center gap-2.5 rounded-md px-2 py-1.5 text-left text-body text-bone transition-colors hover:bg-well hover:text-ink pointer-coarse:min-h-11"
        >
          <Icon
            name={starter.icon}
            size={16}
            className="shrink-0 text-faint transition-colors group-hover:text-muted"
          />
          <span className="min-w-0 flex-1">{starter.text}</span>
        </button>
      ))}
    </div>
  )
}

function Session({ context, project }: { readonly context: string; readonly project: string }) {
  const workflowStore = useWorkflowStore()
  const [restoredTurns, setRestoredTurns] = useState<ReadonlySet<string>>(new Set())
  const [graphProposals, setGraphProposals] = useState<
    readonly (GraphProposal & { readonly turnId: string })[]
  >([])
  const [studyProposals, setStudyProposals] = useState<
    readonly (StudyProposal & { readonly turnId: string })[]
  >([])
  const [studySelections, setStudySelections] = useState<
    readonly (StudySelectionProposal & { readonly turnId: string })[]
  >([])
  const [workspaceProposals, setWorkspaceProposals] = useState<
    readonly (WorkspaceConfigurationProposal & { readonly turnId: string })[]
  >([])
  const studySelection = useStudySelection()
  const bridge = useAssistantBridge()
  const preparation = usePreparationCapability()
  const [pipelineProposals, setPipelineProposals] = useState<
    readonly (PipelineProposal & { readonly turnId: string })[]
  >([])
  useEffect(() => {
    bridge.scopeProfiles(project)
  }, [bridge, project])
  const runApproval = useRunApproval()
  const ownedRun = useRef(false)
  const sql = useSqlCapability()
  const seed = useAssistantSeed()
  const expanded = useAssistantExpanded()
  const placement = useAssistantPlacement()
  const [sqlProposals, setSqlProposals] = useState<
    readonly (SqlProposal & { readonly turnId: string })[]
  >([])
  const capability = useAssistantCapability()
  const [proposals, setProposals] = useState<
    readonly {
      readonly proposal: SelectionProposal
      readonly turnId: string
      readonly status: 'pending' | 'applied' | 'dismissed' | ProposalProblem
    }[]
  >([])
  const { apiKey, model, maxIters, maxRequests, maxTokens, approvedProjects } =
    useAssistantConnection()
  const limits = assistantLimits.safeParse({
    maxIters: Number(maxIters),
    maxRequests: Number(maxRequests),
    maxTokens: Number(maxTokens),
  })
  const approved = approvedProjects.has(project)
  const credentials = apiKey.trim() !== '' && model.trim() !== '' && limits.success
  const connected = credentials && approved
  const [settingsOpen, setSettingsOpen] = useState(!connected)
  const [wasConnected, setWasConnected] = useState(connected)
  if (connected !== wasConnected) {
    setWasConnected(connected)
    setSettingsOpen(!connected)
  }
  const [turns, setTurns] = useState<readonly Turn[]>([])
  const settled = turns.filter((turn) => turn.outcome.kind !== 'running').length
  const [credit, setCredit] = useState<CreditStatus>({ kind: 'unchecked' })
  // Read on connecting and after each question, so the gauge follows what the key has spent.
  useEffect(() => {
    if (!connected) {
      setCredit({ kind: 'unchecked' })
      return
    }
    const controller = new AbortController()
    setCredit((previous) => (previous.kind === 'known' ? previous : { kind: 'checking' }))
    void readKeyCredit(apiKey.trim(), controller.signal).then((status) => {
      if (!controller.signal.aborted) setCredit(status)
    })
    return () => controller.abort()
  }, [connected, apiKey, settled])
  const [memory, setMemory] = useState<
    { kind: 'loading' } | { kind: 'ready'; session: string } | { kind: 'failed'; message: string }
  >({ kind: 'loading' })
  const [memoryWarning, setMemoryWarning] = useState<string | null>(null)
  useEffect(() => {
    let current = true
    void (async () => {
      try {
        const session = getActiveSessionId(project)
        const [legacy, display] = await Promise.all([
          loadMemoryTurns(session),
          loadDisplayTurns(session),
        ])
        const saved = mergeTurnHistory(legacy, display)
        if (current) {
          setTurns(saved)
          setRestoredTurns(new Set(saved.map((turn) => turn.id)))
          setMemory({ kind: 'ready', session })
        }
      } catch {
        if (current)
          setMemory({
            kind: 'failed',
            message:
              'Conversation memory could not be opened. Check browser storage access and reopen the assistant.',
          })
      }
    })()
    return () => {
      current = false
    }
  }, [project])
  const active = useRef<{ id: string; controller: AbortController } | null>(null)
  const pendingTurns = useRef(new Set<Promise<void>>())
  const scroller = useRef<HTMLDivElement>(null)
  useEffect(() => () => active.current?.controller.abort(), [])
  const running = turns.some((turn) => turn.outcome.kind === 'running')
  useEffect(() => {
    setAssistantRunning(running)
  }, [running])
  useEffect(() => () => setAssistantRunning(false), [])
  // Child effects run first, so the composer has taken the seed before it is cleared.
  useEffect(() => {
    if (seed !== null) clearAssistantSeed()
  }, [seed])
  const update = (id: string, change: (turn: Turn) => Turn) =>
    setTurns((previous) => previous.map((turn) => (turn.id === id ? change(turn) : turn)))
  const stop = () => {
    const current = active.current
    if (!current) return
    current.controller.abort()
    active.current = null
    update(current.id, (turn) => ({ ...turn, outcome: { kind: 'cancelled' } }))
  }
  useEffect(() => {
    const current = active.current
    if (current && !ownedRun.current) {
      current.controller.abort()
      active.current = null
      update(current.id, (turn) => ({ ...turn, outcome: { kind: 'cancelled' } }))
    }
  }, [context])
  const ask = async (question: string) => {
    if (active.current || !approved || !credentials || !limits.success || memory.kind !== 'ready')
      return
    const id = crypto.randomUUID()
    const controller = new AbortController()
    ownedRun.current = false
    const runCapabilities = bridge.store.getState().runs
    const configurationSnapshot = bridge.store.getState().configurations
    const graphRevision = graphBinding(workflowStore.getState().workflow)
    const studyRevision = studyBinding(workflowStore.getState().workflow)
    active.current = { id, controller }
    const startedAt = Date.now()
    let savedTurn: Turn = {
      id,
      question,
      context,
      trace: [],
      outcome: { kind: 'running' },
      limits: {
        maxSteps: limits.data.maxIters,
        maxRequests: limits.data.maxRequests,
        deadlineMs: QUESTION_DEADLINE_MS,
      },
    }
    const initialTurn = savedTurn
    const draftWrites: Promise<void>[] = []
    const rememberDraft = (title: string, value: unknown) => {
      const drafts = [
        ...(savedTurn.drafts ?? []),
        { title, content: JSON.stringify(value, null, 2) },
      ]
      savedTurn = { ...savedTurn, drafts }
      update(id, (turn) => ({ ...turn, drafts }))
      const checkpoint: Turn = { ...savedTurn, outcome: { kind: 'cancelled' } }
      draftWrites.push(
        (draftWrites.at(-1) ?? Promise.resolve())
          .then(() => saveDisplayTurn(project, memory.session, startedAt, checkpoint))
          .catch(() => setMemoryWarning('The proposal could not be saved for reload recovery.')),
      )
    }
    setTurns((previous) => [...previous, initialTurn])
    let finishPending!: () => void
    const pending = new Promise<void>((resolve) => {
      finishPending = resolve
    })
    pendingTurns.current.add(pending)
    let task: Awaited<ReturnType<typeof beginMemoryTask>> | undefined
    try {
      task = await beginMemoryTask(memory.session, question, context, id, setMemoryWarning)
      controller.signal.throwIfAborted()
      const answer = await runAssistant({
        workspaceSettings: (raw) =>
          readWorkspaceSettings(bridge.store.getState().configurations, raw),
        readProject: () => {
          const current = bridge.store.getState()
          return assistantContext(
            workflowStore.getState().workflow,
            current.current?.inspection ?? null,
            current.sql?.inspection ?? null,
            Object.fromEntries(
              Object.entries(current.runs).map(([key, value]) => [key, value.inspection]),
            ),
            (current.preparation ?? current.sourceData)?.inspection ?? null,
            current.studySelection?.inspection ?? null,
          )
        },
        proposeWorkspace: (raw) => {
          const result = proposeWorkspaceConfiguration(configurationSnapshot, raw)
          if (
            result.ok &&
            bridge.store.getState().configurations[result.value.key]?.revision !==
              result.value.revision
          )
            return err('The workspace changed. Ask for a new proposal.')
          if (result.ok) rememberDraft('Workspace settings', result.value)
          if (result.ok)
            setWorkspaceProposals((previous) => [...previous, { ...result.value, turnId: id }])
          return result
        },
        key: apiKey.trim(),
        model: model.trim(),
        question,
        context,
        limits: limits.data,
        readStudy: (raw) => readStudy(workflowStore.getState().workflow, raw),
        readRun: (raw) => readRun(workflowStore.getState().workflow, raw),
        history: conversationRecords(turns),
        memory: task.access,
        proposeStudySelection: studySelection
          ? (raw) => {
              const current = bridge.store.getState().studySelection
              if (
                controller.signal.aborted ||
                !current ||
                current.revision !== studySelection.revision
              )
                return err('The available studies or settings changed. Ask for a new proposal.')
              const result = proposeStudySelection(current, raw)
              if (result.ok) rememberDraft('Study selection', result.value)
              if (result.ok)
                setStudySelections((previous) => [...previous, { ...result.value, turnId: id }])
              return result
            }
          : undefined,
        proposeStudy:
          studyRevision === null
            ? undefined
            : (raw) => {
                const workflow = workflowStore.getState().workflow
                if (controller.signal.aborted || studyBinding(workflow) !== studyRevision)
                  return err('The data, graph or study draft changed. Ask for a new proposal.')
                const result = proposeStudy(workflow, raw)
                if (result.ok) rememberDraft('Study draft', result.value)
                if (result.ok)
                  setStudyProposals((previous) => [...previous, { ...result.value, turnId: id }])
                return result
              },
        proposeGraph:
          graphRevision === null
            ? undefined
            : (raw) => {
                const workflow = workflowStore.getState().workflow
                if (controller.signal.aborted || graphBinding(workflow) !== graphRevision)
                  return err('The data or graphs changed. Ask for a new proposal.')
                const result = proposeGraph(workflow, raw)
                if (result.ok) rememberDraft('Graph changes', result.value)
                if (result.ok)
                  setGraphProposals((previous) => [...previous, { ...result.value, turnId: id }])
                return result
              },
        profile: preparation
          ? (raw, signal) => bridge.profile(preparation.revision, raw, signal)
          : undefined,
        proposePipeline:
          preparation?.kind === 'pipeline'
            ? (raw) => {
                if (
                  controller.signal.aborted ||
                  bridge.store.getState().preparation !== preparation
                )
                  return err('The workspace changed. Ask for a new proposal.')
                const result = proposePipeline(preparation, raw)
                if (result.ok) rememberDraft('Pipeline draft', result.value)
                if (result.ok)
                  setPipelineProposals((previous) => [...previous, { ...result.value, turnId: id }])
                return result
              }
            : undefined,
        signal: controller.signal,
        requestRun: (key, reason, signal, subject, waiting) => {
          const run = runCapabilities[key]
          if (!run)
            return Promise.resolve({
              kind: 'refused' as const,
              reason:
                key === 'identification'
                  ? 'Open Study design, choose the graph, treatment and outcome, then request identification again.'
                  : key === 'swig' || key === 'swig-separation'
                    ? 'Open DAG workspace. Use dag-workspace settings to select the source DAG and open SWIG, then review the check settings before running.'
                    : 'This analysis is not available. Open its section and review the controls.',
            })
          return runApproval.request(
            key,
            run.revision,
            reason,
            signal,
            () => {
              ownedRun.current = true
            },
            subject,
            waiting,
            id,
          )
        },
        proposeSql:
          sql === null
            ? null
            : (raw) => {
                if (
                  controller.signal.aborted ||
                  bridge.store.getState().sql?.revision !== sql.revision
                )
                  return err('stale' as const)
                const result = proposeSql(sql, raw)
                if (result.ok) rememberDraft('SQL draft', result.value)
                if (result.ok)
                  setSqlProposals((previous) => [...previous, { ...result.value, turnId: id }])
                return result
              },
        propose:
          capability === null
            ? null
            : (raw) => {
                if (
                  controller.signal.aborted ||
                  bridge.store.getState().current?.revision !== capability.revision
                )
                  return err('stale' as const)
                const result = proposeSelection(capability, raw)
                if (result.ok) rememberDraft('Estimator settings', result.value)
                if (result.ok)
                  setProposals((previous) => [
                    ...previous,
                    { proposal: result.value, status: 'pending', turnId: id },
                  ])
                return result
              },
        trace: (event) => {
          if (!controller.signal.aborted) {
            task?.trace(event)
            savedTurn = {
              ...savedTurn,
              trace: appendTrace(savedTurn.trace, { ...event, at: Date.now() }),
            }
            const trace = savedTurn.trace
            update(id, (turn) => ({ ...turn, trace }))
          }
        },
      })
      if (!controller.signal.aborted) await task.finish(answer)
      savedTurn = {
        ...savedTurn,
        outcome: controller.signal.aborted ? { kind: 'cancelled' } : { kind: 'answered', answer },
      }
    } catch (error) {
      savedTurn = {
        ...savedTurn,
        outcome: controller.signal.aborted
          ? { kind: 'cancelled' }
          : {
              kind: 'failed',
              message: assistantFailureMessage(error),
            },
      }
    } finally {
      try {
        await task?.flush()
        await Promise.all(draftWrites)
        await saveDisplayTurn(project, memory.session, startedAt, savedTurn)
      } catch {
        setMemoryWarning(
          'The conversation trace could not be saved. It may be unavailable after a reload.',
        )
      }
      const outcome = savedTurn.outcome
      update(id, (turn) => ({ ...turn, outcome }))
      pendingTurns.current.delete(pending)
      finishPending()
      if (active.current?.id === id) {
        active.current = null
        ownedRun.current = false
      }
    }
  }
  const count = turns.length
  useEffect(() => {
    if (count > 0) scroller.current?.scrollTo({ top: scroller.current.scrollHeight })
  }, [count])
  return (
    <div
      className="flex min-h-0 flex-1 flex-col text-body"
      style={{ fontFeatureSettings: '"calt" 0' }}
    >
      <header
        className={cn(
          'workbench-pane-header flex h-9 shrink-0 select-none items-center gap-1.5 border-b border-hair px-3',
          placement === 'floating' && [ASSISTANT_DRAG, 'cursor-grab active:cursor-grabbing'],
        )}
      >
        <AssistantMark
          state={connected ? (running ? 'working' : 'open') : 'offline'}
          size={18}
          className="text-ink"
        />
        <h2 className={label('m-0 flex-1 text-ink')}>CausalBuddy</h2>
        <button
          type="button"
          aria-label="CausalBuddy settings"
          aria-expanded={settingsOpen}
          aria-controls="causalbuddy-settings"
          title={connected ? 'Settings: connected to OpenRouter' : 'Settings: not connected yet'}
          className={iconControl(
            settingsOpen ? 'selected' : 'quiet',
            cn('h-7 w-7 border-transparent', settingsOpen ? 'text-ink' : 'text-faint'),
          )}
          onClick={() => setSettingsOpen((open) => !open)}
        >
          <Icon name="settings" size={15} />
        </button>
        <button
          type="button"
          aria-pressed={expanded}
          aria-label={expanded ? 'Show the chapter' : 'Expand the assistant'}
          className={iconControl('quiet', 'h-7 w-7 text-faint md:hidden')}
          onClick={toggleAssistantExpanded}
        >
          <Icon name={expanded ? 'close_fullscreen' : 'open_in_full'} size={15} />
        </button>
        <button
          type="button"
          aria-label={placement === 'floating' ? 'Dock the assistant' : 'Float the assistant'}
          title={
            placement === 'floating' ? 'Dock (or drag to the right edge)' : 'Float over the chapter'
          }
          className={iconControl('quiet', 'hidden h-7 w-7 text-faint md:grid')}
          onClick={placement === 'floating' ? dockAssistant : floatAssistant}
        >
          <Icon name={placement === 'floating' ? 'right_panel_open' : 'open_in_new'} size={15} />
        </button>
        {memory.kind !== 'ready' ? null : (
          <button
            type="button"
            aria-label="Clear conversation"
            title="Clear conversation and stored memory for this project"
            disabled={running}
            className={iconControl('danger', 'h-7 w-7')}
            onClick={() => {
              setMemory({ kind: 'loading' })
              void Promise.all([...pendingTurns.current])
                .then(() =>
                  Promise.all([clearProjectMemory(project), clearDisplayHistory(project)]),
                )
                .then(() => {
                  const session = rotateSession(project)
                  setTurns([])
                  setProposals([])
                  setStudySelections([])
                  setWorkspaceProposals([])
                  setSqlProposals([])
                  setPipelineProposals([])
                  setGraphProposals([])
                  setStudyProposals([])
                  bridge.clearProfiles()
                  runApproval.clear()
                  setMemoryWarning(null)
                  setMemory({ kind: 'ready', session })
                })
                .catch(() =>
                  setMemory({
                    kind: 'failed',
                    message:
                      'Stored conversation memory could not be deleted. It has not been marked as cleared.',
                  }),
                )
            }}
          >
            <Icon name="delete" size={15} />
          </button>
        )}
        <button
          type="button"
          aria-label="Close the assistant"
          className={iconControl('quiet', 'h-7 w-7 text-faint')}
          onClick={closeAssistant}
        >
          <Icon name="close" size={15} />
        </button>
      </header>
      <ConnectionBar
        open={settingsOpen}
        context={context}
        apiKey={apiKey}
        model={model}
        onApiKey={(apiKey) => configureAssistantConnection({ apiKey })}
        onModel={(model) => configureAssistantConnection({ model })}
        maxIters={maxIters}
        maxRequests={maxRequests}
        maxTokens={maxTokens}
        onMaxIters={(maxIters) => configureAssistantConnection({ maxIters })}
        onMaxRequests={(maxRequests) => configureAssistantConnection({ maxRequests })}
        onMaxTokens={(maxTokens) => configureAssistantConnection({ maxTokens })}
        connected={connected}
        credit={credit}
        disabled={running}
        onConnect={() => approveAssistantProject(project)}
        onForget={forgetAssistantConnection}
      />
      {memory.kind === 'failed' || memoryWarning !== null ? (
        <p role="alert" className="m-0 border-b border-line px-4 py-2 text-label text-danger">
          {memory.kind === 'failed' ? memory.message : memoryWarning}
        </p>
      ) : null}
      <ProposalActivity.Provider value={running}>
        <div ref={scroller} className="panel-scroll min-h-0 flex-1 overflow-y-auto">
          {turns.length === 0 ? (
            <div className="flex min-h-full flex-col px-4 pb-3 pt-7">
              <div className="mt-auto -mx-2">
                <Starters
                  estimation={capability !== null}
                  sql={sql !== null}
                  onAsk={(question) =>
                    connected && approved ? void ask(question) : askAssistant(question)
                  }
                />
              </div>
            </div>
          ) : (
            <div className="px-4">
              {turns.map((turn) => (
                <section key={turn.id} aria-label="Conversation turn">
                  <TurnView turn={turn} />
                  {restoredTurns.has(turn.id) &&
                    turn.drafts?.map((draft, index) => (
                      <ProposalCard key={index} name="Saved proposal" title={draft.title}>
                        <p className="m-0 text-label">
                          This proposal was saved with the conversation. Check current settings
                          before asking the assistant to propose it again; no approval carries
                          forward.
                        </p>
                        <details>
                          <summary className="cursor-pointer text-label">
                            Saved draft contents
                          </summary>
                          <CodeHighlight
                            code={draft.content}
                            lang="json"
                            copyable
                            copyLabel="Copy saved draft"
                            className="max-h-64 overflow-auto text-label"
                          />
                        </details>
                      </ProposalCard>
                    ))}
                  {sqlProposals
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <SqlProposalCard key={index} proposal={proposal} />
                    ))}
                  {pipelineProposals
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <PipelineProposalCard key={index} proposal={proposal} />
                    ))}
                  <RunApprovalCards
                    requests={runApproval.requests.filter((request) => request.turnId === turn.id)}
                  />
                  {graphProposals
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <GraphProposalCard key={index} proposal={proposal} />
                    ))}
                  {studyProposals
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <StudyProposalCard key={index} proposal={proposal} />
                    ))}
                  {studySelections
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <StudySelectionCard key={index} proposal={proposal} />
                    ))}
                  {workspaceProposals
                    .filter((proposal) => proposal.turnId === turn.id)
                    .map((proposal, index) => (
                      <WorkspaceConfigurationCard key={index} proposal={proposal} />
                    ))}
                  {proposals.map((entry, index) => {
                    if (entry.turnId !== turn.id) return null
                    const stale = entry.proposal.revision !== capability?.revision
                    return (
                      <ProposalCard
                        key={index}
                        name="Estimator proposal"
                        title={`${entry.proposal.configuration ? 'Configure' : 'Select'} ${entry.proposal.choice.name}`}
                      >
                        {entry.proposal.configuration ? (
                          <ProposalSettings
                            settings={configurationSettings(entry.proposal.configuration)}
                          />
                        ) : null}
                        <p className="m-0">{entry.proposal.reason}</p>
                        <EligibilityView
                          eligibility={entry.proposal.choice.eligibility}
                          subject="this proposal"
                        />
                        {entry.status === 'pending' && !stale ? (
                          <ProposalActions
                            action={
                              entry.proposal.configuration ? 'Apply settings' : 'Apply selection'
                            }
                            help={
                              entry.proposal.configuration
                                ? 'This updates the method settings. The study and its adjustment set stay unchanged. No analysis is run.'
                                : 'This changes the selected method only. It does not run an estimate.'
                            }
                            disabled={running}
                            onAction={() => {
                              const result = bridge.apply(entry.proposal)
                              setProposals((previous) =>
                                previous.map((item, i) =>
                                  i === index
                                    ? { ...item, status: result.ok ? 'applied' : result.error }
                                    : item,
                                ),
                              )
                            }}
                            onDismiss={() =>
                              setProposals((previous) =>
                                previous.map((item, i) =>
                                  i === index ? { ...item, status: 'dismissed' } : item,
                                ),
                              )
                            }
                          />
                        ) : (
                          <p role="status" className="m-0 text-label text-muted">
                            {entry.status === 'applied'
                              ? entry.proposal.configuration
                                ? 'Settings applied. Review them in Estimation.'
                                : 'Selection applied. Review it in Estimation.'
                              : entry.status === 'dismissed'
                                ? 'Proposal dismissed.'
                                : typeof entry.status === 'object'
                                  ? entry.status.violations.map((item) => item.evidence).join(' ')
                                  : 'The study, settings or available tools changed. Ask for a new proposal.'}
                          </p>
                        )}
                      </ProposalCard>
                    )
                  })}
                </section>
              ))}
            </div>
          )}
        </div>
      </ProposalActivity.Provider>
      <Composer
        onSend={(question) => void ask(question)}
        onStop={stop}
        isStreaming={running}
        history={turns.map((turn) => turn.question)}
        unavailable={!approved || !credentials || memory.kind !== 'ready'}
        placeholder="Ask about your analysis…"
        seed={seed}
      />
    </div>
  )
}
