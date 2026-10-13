import { createContext, useContext, useLayoutEffect, useRef, useState, type ReactNode } from 'react'
import { createStore } from 'zustand/vanilla'
import { useStore } from 'zustand'
import { z } from 'zod'
import type { EstimatorId } from '@/domain/estimation'
import type { ColumnId } from '@/domain/dataset'
import {
  readEstimatorConfiguration,
  type EditableEstimatorConfiguration,
} from './estimatorConfiguration'
import type { MethodEligibility } from '@/domain/methods'
import { err, ok, type Result } from '@/domain/dop'
import type { RunCapability, RunInspection, AssistantRunOutcome } from './analysisRun'
import { proposalReviewSchema, type ProposalReview } from './proposalReview'
import type { StudySelectionCapability } from './studySelection'
import type { WorkspaceConfigurationCapability } from './workspaceConfiguration'
import {
  compareProfiles,
  profileToolSchema,
  type PreparationCapability,
  type ProfileReceipt,
  type PipelineProposal,
} from './preparation'

export interface EstimatorChoice {
  readonly id: EstimatorId
  readonly name: string
  readonly eligibility: MethodEligibility
}
export interface EstimationInspection {
  readonly study: { readonly treatment: string; readonly outcome: string; readonly target: string }
  readonly selected: EstimatorId
  readonly choices: readonly EstimatorChoice[]
  readonly configuration?: {
    readonly current: EditableEstimatorConfiguration
    readonly observations?: number
    readonly columns: readonly { readonly id: ColumnId; readonly name: string }[]
    readonly temporal?: {
      readonly candidates: readonly { readonly index: number; readonly name: string }[]
      readonly graphLag: number
    }
  }
}
interface Capability {
  readonly revision: string
  readonly inspection: EstimationInspection
  readonly select: (estimator: EstimatorId) => void
  readonly configure?: (configuration: EditableEstimatorConfiguration) => void
  readonly evaluate?: (configuration: EditableEstimatorConfiguration) => MethodEligibility
}
export interface SelectionProposal {
  readonly kind: 'select-estimator'
  readonly revision: string
  readonly choice: EstimatorChoice
  readonly reason: string
  readonly configuration?: EditableEstimatorConfiguration
}
export type ProposalProblem =
  | 'unavailable'
  | 'stale'
  | 'invalid'
  | 'refused'
  | {
      readonly kind: 'requirements-not-met'
      readonly violations: Extract<MethodEligibility, { kind: 'refused' }>['violations']
    }
export interface SqlInspection {
  readonly inputs: readonly { readonly alias: string; readonly format: string }[]
  readonly outputView: string
}
interface SqlCapability {
  readonly revision: string
  readonly inspection: SqlInspection
  readonly load: (statement: string) => Promise<void>
}
export interface SqlProposal extends ProposalReview {
  readonly kind: 'sql-draft'
  readonly revision: string
  readonly statement: string
}
const sqlProposalSchema = proposalReviewSchema
  .extend({
    statement: z.string().trim().min(1).max(16000),
  })
  .strict()
export function proposeSql(
  capability: SqlCapability | null,
  raw: unknown,
): Result<SqlProposal, ProposalProblem> {
  if (!capability) return err('unavailable')
  const parsed = sqlProposalSchema.safeParse(raw)
  if (!parsed.success) return err('invalid')
  return ok({ kind: 'sql-draft', revision: capability.revision, ...parsed.data })
}
const proposalSchema = z
  .object({
    estimator: z.string(),
    reason: z.string().trim().min(1).max(2000),
    configuration: z.unknown().optional(),
  })
  .strict()

export function proposeSelection(
  capability: Capability | null,
  raw: unknown,
): Result<SelectionProposal, ProposalProblem> {
  if (!capability) return err('unavailable')
  const parsed = proposalSchema.safeParse(raw)
  if (!parsed.success) return err('invalid')
  const choice = capability.inspection.choices.find((item) => item.id === parsed.data.estimator)
  if (!choice) return err('invalid')
  if (parsed.data.configuration !== undefined) {
    const settings = capability.inspection.configuration
    if (
      !settings ||
      !capability.configure ||
      !capability.evaluate ||
      choice.id !== settings.current.kind
    )
      return err('unavailable')
    const configuration = readEstimatorConfiguration(parsed.data.configuration, settings.columns)
    if (!configuration.ok) return configuration
    if (configuration.value.kind !== choice.id) return err('invalid')
    if (configuration.value.kind === 'vecm' && configuration.value.breakIndex !== null &&
      (settings.observations === undefined || configuration.value.breakIndex > settings.observations - 4))
      return err('invalid')
    if (configuration.value.kind === 'causal-effects-total') {
      const estimator = configuration.value.estimator
      const treatmentLag = configuration.value.treatmentLag
      if (estimator.kind !== 'wrightParents' && estimator.adjustment.kind === 'explicit') {
        const temporal = settings.temporal
        const nodes = estimator.adjustment.nodes
        if (!temporal || nodes.some(([index, lag]) =>
          !temporal.candidates.some((candidate) => candidate.index === index) ||
          -lag > temporal.graphLag + treatmentLag
        ) || new Set(nodes.map(([index, lag]) => `${index}:${lag}`)).size !== nodes.length)
          return err('invalid')
      }
    }
    if (
      (configuration.value.kind === 'dml-plr' || configuration.value.kind === 'dml-irm') &&
      (settings.current.kind === 'dml-plr' || settings.current.kind === 'dml-irm') &&
      configuration.value.att !== settings.current.att
    )
      return err('invalid')
    const eligibility = capability.evaluate(configuration.value)
    if (eligibility.kind === 'refused')
      return err({ kind: 'requirements-not-met', violations: eligibility.violations })
    return ok({
      kind: 'select-estimator',
      revision: capability.revision,
      choice: { ...choice, eligibility },
      reason: parsed.data.reason,
      configuration: configuration.value,
    })
  }
  if (choice.eligibility.kind === 'refused')
    return err({ kind: 'requirements-not-met', violations: choice.eligibility.violations })
  return ok({
    kind: 'select-estimator',
    revision: capability.revision,
    choice,
    reason: parsed.data.reason,
  })
}

export const createAssistantBridge = () => {
  const store = createStore<{
    readonly current: Capability | null
    readonly sql: SqlCapability | null
    readonly preparation: PreparationCapability | null
    readonly sourceData: Extract<PreparationCapability, { kind: 'source' }> | null
    readonly runs: Readonly<Record<string, RunCapability>>
    readonly studySelection: StudySelectionCapability | null
    readonly configurations: Readonly<Record<string, WorkspaceConfigurationCapability>>
  }>(() => ({
    current: null,
    sql: null,
    preparation: null,
    sourceData: null,
    runs: {},
    studySelection: null,
    configurations: {},
  }))
  const receipts = new Map<string, ProfileReceipt>()
  let profileProject: string | null = null
  return {
    store,
    publishSourceData(capability: Extract<PreparationCapability, { kind: 'source' }>) {
      store.setState({ sourceData: capability })
      return () => {
        if (store.getState().sourceData === capability) store.setState({ sourceData: null })
      }
    },
    publishPreparation(capability: PreparationCapability) {
      store.setState({ preparation: capability })
      return () => {
        if (store.getState().preparation === capability) store.setState({ preparation: null })
      }
    },
    clearProfiles() {
      receipts.clear()
    },
    scopeProfiles(project: string) {
      if (profileProject !== project) receipts.clear()
      profileProject = project
    },
    async profile(revision: string, raw: unknown, signal: AbortSignal) {
      const capability = store.getState().preparation ?? store.getState().sourceData
      if (!capability || capability.revision !== revision)
        return {
          kind: 'refused' as const,
          reason: 'The workspace changed. Inspect the current workspace again.',
        }
      const parsed = profileToolSchema.safeParse(raw)
      if (!parsed.success)
        return {
          kind: 'refused' as const,
          reason: 'Supply a table, up to 12 columns, and optionally categoryColumn, compareTo or up to eight key, time or join checks.',
        }
      const before = parsed.data.compareTo ? receipts.get(parsed.data.compareTo) : undefined
      if (parsed.data.compareTo && !before)
        return {
          kind: 'refused' as const,
          reason: 'That profile is no longer available. Capture a new baseline.',
        }
      try {
        const profile = await capability.profile(parsed.data, signal)
        signal.throwIfAborted()
        if ((store.getState().preparation ?? store.getState().sourceData) !== capability)
          return {
            kind: 'refused' as const,
            reason: 'The workspace changed during profiling. The summary was discarded.',
          }
        const receipt: ProfileReceipt = {
          id: crypto.randomUUID(),
          revision,
          capturedAt: new Date().toISOString(),
          profile,
        }
        receipts.set(receipt.id, receipt)
        if (receipts.size > 40) {
          const oldest = receipts.keys().next()
          if (!oldest.done) receipts.delete(oldest.value)
        }
        return {
          kind: 'profiled' as const,
          receipt,
          comparison: before
            ? { baseline: before.id, ...compareProfiles(before.profile, profile) }
            : null,
        }
      } catch (error) {
        signal.throwIfAborted()
        return {
          kind: 'refused' as const,
          reason: error instanceof Error ? error.message : 'Profiling failed.',
        }
      }
    },
    applyPipeline(proposal: PipelineProposal): Result<void, ProposalProblem> {
      const capability = store.getState().preparation
      if (!capability || capability.kind !== 'pipeline') return err('unavailable')
      if (capability.revision !== proposal.revision) return err('stale')
      capability.apply(proposal.graph)
      return ok(undefined)
    },
    publishRun(key: string, capability: RunCapability) {
      store.setState((state) => ({ runs: { ...state.runs, [key]: capability } }))
      return () => {
        if (store.getState().runs[key] === capability) {
          const runs = { ...store.getState().runs }
          delete runs[key]
          store.setState({ runs })
        }
      }
    },
    async run(key: string, revision: string, signal: AbortSignal): Promise<AssistantRunOutcome> {
      if (signal.aborted) return { kind: 'cancelled' }
      const capability = store.getState().runs[key]
      if (!capability || capability.revision !== revision)
        return { kind: 'refused', reason: 'The specification changed. Request a new run.' }
      return capability.execute(signal)
    },
    publishSql(capability: SqlCapability) {
      store.setState({ sql: capability })
      return () => {
        if (store.getState().sql === capability) store.setState({ sql: null })
      }
    },
    async loadSql(proposal: SqlProposal): Promise<Result<void, ProposalProblem>> {
      const capability = store.getState().sql
      if (!capability) return err('unavailable')
      if (capability.revision !== proposal.revision) return err('stale')
      const checked = proposeSql(capability, {
        statement: proposal.statement,
        summary: proposal.summary,
        checks: proposal.checks,
      })
      if (!checked.ok) return checked
      await capability.load(checked.value.statement)
      return ok(undefined)
    },
    publish(capability: Capability) {
      store.setState({ current: capability })
      return () => {
        if (store.getState().current === capability) store.setState({ current: null })
      }
    },
    apply(proposal: SelectionProposal): Result<void, ProposalProblem> {
      const capability = store.getState().current
      if (!capability) return err('unavailable')
      if (capability.revision !== proposal.revision) return err('stale')
      const checked = proposeSelection(capability, {
        estimator: proposal.choice.id,
        reason: proposal.reason,
        configuration: proposal.configuration,
      })
      if (!checked.ok) return checked
      if (checked.value.configuration) capability.configure!(checked.value.configuration)
      else capability.select(checked.value.choice.id)
      return ok(undefined)
    },
  }
}
const Context = createContext<ReturnType<typeof createAssistantBridge> | null>(null)
export function AssistantToolsProvider({ children }: { readonly children: ReactNode }) {
  const [bridge] = useState(createAssistantBridge)
  return <Context.Provider value={bridge}>{children}</Context.Provider>
}
export function useAssistantBridge() {
  const bridge = useContext(Context)
  if (!bridge) throw new Error('AssistantToolsProvider is required.')
  return bridge
}
export function useAssistantCapability() {
  const bridge = useAssistantBridge()
  return useStore(bridge.store, (state) => state.current)
}
export function useSqlCapability() {
  const bridge = useAssistantBridge()
  return useStore(bridge.store, (state) => state.sql)
}
export function usePreparationCapability() {
  const bridge = useAssistantBridge()
  return useStore(bridge.store, (state) => state.preparation ?? state.sourceData)
}
export function usePreparationTools(
  binding: string,
  capability:
    | Omit<Extract<PreparationCapability, { kind: 'sql' }>, 'revision'>
    | Omit<Extract<PreparationCapability, { kind: 'pipeline' }>, 'revision'>
    | null,
) {
  const bridge = useAssistantBridge()
  const latest = useRef(capability)
  useLayoutEffect(() => {
    latest.current = capability
  })
  const description = JSON.stringify(capability?.inspection ?? null)
  useLayoutEffect(() => {
    const value = latest.current
    if (!value) return
    return bridge.publishPreparation({ ...value, revision: crypto.randomUUID() })
  }, [bridge, binding, description])
}
export function useRunCapabilities() {
  const bridge = useAssistantBridge()
  return useStore(bridge.store, (state) => state.runs)
}
export function useAnalysisRunTool(
  key: string,
  binding: string,
  inspection: RunInspection | null,
  execute: RunCapability['execute'],
) {
  const bridge = useAssistantBridge()
  const latest = useRef({ inspection, execute })
  useLayoutEffect(() => {
    latest.current = { inspection, execute }
  })
  const description = JSON.stringify(inspection)
  useLayoutEffect(() => {
    const value = latest.current.inspection
    if (!value) return
    return bridge.publishRun(key, {
      revision: crypto.randomUUID(),
      inspection: value,
      execute: (signal) => latest.current.execute(signal),
    })
  }, [bridge, key, binding, description])
}
export function useSqlTools(
  binding: string,
  inspection: SqlInspection | null,
  load: (statement: string) => Promise<void>,
) {
  const bridge = useAssistantBridge()
  const latest = useRef({ inspection, load })
  useLayoutEffect(() => {
    latest.current = { inspection, load }
  })
  const description = JSON.stringify(inspection)
  useLayoutEffect(() => {
    const value = latest.current.inspection
    if (!value) return
    return bridge.publishSql({
      revision: crypto.randomUUID(),
      inspection: value,
      load: (statement) => latest.current.load(statement),
    })
  }, [bridge, binding, description])
}
export function useEstimationTools(
  binding: string,
  inspection: EstimationInspection | null,
  select: (estimator: EstimatorId) => void,
  configuration?: Pick<Capability, 'configure' | 'evaluate'>,
) {
  const bridge = useAssistantBridge()
  const latest = useRef({ inspection, select, configuration })
  useLayoutEffect(() => {
    latest.current = { inspection, select, configuration }
  })
  const description = JSON.stringify(inspection)
  useLayoutEffect(() => {
    const value = latest.current.inspection
    if (!value) return
    return bridge.publish({
      revision: crypto.randomUUID(),
      inspection: value,
      select: (estimator) => latest.current.select(estimator),
      ...(latest.current.configuration
        ? {
            configure: (settings: EditableEstimatorConfiguration) =>
              latest.current.configuration!.configure!(settings),
            evaluate: (settings: EditableEstimatorConfiguration) =>
              latest.current.configuration!.evaluate!(settings),
          }
        : {}),
    })
  }, [bridge, binding, description])
}
