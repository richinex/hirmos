import { EmptyState } from '@/components/ui/EmptyState'
import { Select } from '@/components/ui/Select'
import { useEffect, useMemo, useState, type ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { MethodCaveats } from '@/components/MethodCaveats'
import { EligibilityView } from '@/components/EligibilityView'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ParameterHelp, ParameterLabel } from '@/components/ui/ParameterLabel'
import {
  EVIDENCE_SCOPES,
  explainEvidenceScope,
  describeEvidenceScope,
  matchesEvidenceSelection,
  NO_EVIDENCE_SELECTION,
  selectsEverything,
  type EvidenceScope,
  type EvidenceSelection,
} from '@/domain/evidenceScope'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { CmlpLagPlot, NeuralSummaryPlot, OcsePlot, RpcmciMembershipPlot, RpcmciTimeGraphPlot, StructurePlot, TimeGraphPlot, WeightPlot } from './DiscoveryPlots'
import { RadioList } from '@/components/ui/RadioList'
import { button, field, figureGrid, iconControl, label, literal, num, panel, well } from '@/components/ui/recipes'
import type { DatasetProfile } from '@/domain/dataset'
import type { DagDocument } from '@/domain/dag'
import {
  DISCOVERY_LAG_OPTIONS,
  DISCOVERY_METHOD_GROUPS,
  DYNOTEARS_PENALTY_OPTIONS,
  OCSE_SHUFFLE_OPTIONS,
  PCMCI_ALPHA_OPTIONS,
  describeDiscoveryReadiness,
  describeDiscoveryRunProblem,
  discoveryMethodGroupById,
  discoveryMethodGroupFor,
  evaluateDiscoveryEligibility,
  newDiscoveryRunId,
  readyDiscoverySpecification,
  type DiscoveryConfiguration,
  type DiscoveryDraft,
  type DiscoveryEvent,
  type DiscoveryLag,
  type DiscoveryRunArtifact,
  type DynotearsPenalty,
  type OcseShuffles,
  type PcmciAlpha,
  type DiscoveryMethodChoice,
  type DiscoveryMethodGroupId,
} from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import {
  assessDiscoveryRunDeletion,
  type DeletableDiscoveryRun,
  type DiscoveryRunReference,
} from '@/domain/discoveryLifecycle'
import {
  DYNOTEARS_METHOD_ID,
  DIRECT_LINGAM_METHOD_ID,
  CDNOTS_PAR_CORR_METHOD_ID,
  CDNOTS_PLUS_PAR_CORR_METHOD_ID,
  GRACE_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  RPCMCI_PAR_CORR_METHOD_ID,
  OCSE_METHOD_ID,
  CMLP_METHOD_ID,
  CLSTM_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  VAR_LINGAM_METHOD_ID,
  methodDefinition,
  type MethodDefinition,
  type MethodEligibility,
} from '@/domain/methods'
import { describeSeriesTransform, type MissingnessDraft, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { interpretDiscoveryResult } from '@/domain/resultInterpretation'
import { formatTimestamp } from '@/lib/format/date'
import { formatCount } from '@/lib/format/number'
import { DISCOVERY_PARAMETER_HELP } from '@/domain/parameterHelp'
import type { AnalysisWorkerProblem, TemporalSamples } from '@/workers/analysisProtocol'
import type { PreparedMatrix, RoleAwarePreparedMatrix } from '@/data/prepared'

const DISCOVERY_GROUP_LABELS: Readonly<Record<DiscoveryMethodGroupId, string>> = {
  'pcmci-family': 'PCMCI',
  'nonstationary-constraint': 'Nonstationary',
  'lingam-family': 'LiNGAM',
  'continuous-optimization': 'DYNOTEARS',
  'causation-entropy': 'oCSE',
  'neural-granger': 'Neural',
}

interface DiscoveryPanelProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly runs: readonly DiscoveryRunArtifact[]
  readonly documents: readonly DagDocument[]
  readonly draft: DiscoveryDraft
  readonly onEvent: (event: DiscoveryEvent) => void
  readonly cancellation: { requested: boolean }
  readonly onRun: (artifact: DiscoveryRunArtifact) => void
  readonly onDeleteRun: (deletion: DeletableDiscoveryRun) => void
}

type DiscoveryDeletionDialog =
  | { readonly kind: 'closed' }
  | {
      readonly kind: 'confirming'
      readonly run: DiscoveryRunArtifact
      readonly deletion: DeletableDiscoveryRun
    }
  | {
      readonly kind: 'blocked'
      readonly run: DiscoveryRunArtifact
      readonly references: readonly [DiscoveryRunReference, ...DiscoveryRunReference[]]
    }
  | { readonly kind: 'not-found'; readonly run: DiscoveryRunArtifact }

const describeDiscoveryReference = (reference: DiscoveryRunReference): string => {
  switch (reference.kind) {
    case 'dag-origin-reference':
      return `${reference.documentName} records this run as part of its graph basis.`
    case 'edge-evidence-reference': {
      const timing = reference.timing.kind === 'contemporaneous' ? '' : ` at lag ${reference.timing.lag}`
      return `${reference.documentName}, revision ${formatTimestamp(reference.revisionCreatedAt)}: ${reference.cause} → ${reference.effect}${timing}.`
    }
    default: return assertNever(reference)
  }
}

const discoveryRunName = (run: DiscoveryRunArtifact): string => {
  const definition = methodDefinition(run.method)
  return definition.ok ? definition.value.name : 'discovery'
}

const lagFromValue = (value: string): DiscoveryLag | null =>
  DISCOVERY_LAG_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const alphaFromValue = (value: string): PcmciAlpha | null =>
  PCMCI_ALPHA_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const penaltyFromValue = (value: string): DynotearsPenalty | null =>
  DYNOTEARS_PENALTY_OPTIONS.find((candidate) => String(candidate) === value) ?? null

const shufflesFromValue = (value: string): OcseShuffles | null =>
  OCSE_SHUFFLE_OPTIONS.find((candidate) => String(candidate) === value) ?? null

type LagAwareMissingness = Extract<MissingnessDraft, { readonly kind: 'lag-aware-exclusion' }>
type DiscoveryInput =
  | { readonly kind: 'dense'; readonly matrix: PreparedMatrix }
  | {
      readonly kind: 'role-aware'
      readonly matrix: RoleAwarePreparedMatrix
      readonly missingness: LagAwareMissingness
    }

const maskTypeFor = (
  exclusions: LagAwareMissingness['analysisExclusions'],
): Extract<TemporalSamples, { readonly kind: 'role-aware' }>['maskType'] => {
  if (exclusions.kind === 'ignore') return 'none'
  const x = exclusions.roles.includes('candidate-cause')
  const y = exclusions.roles.includes('tested-outcome')
  const z = exclusions.roles.includes('conditioner')
  if (x && y && z) return 'xyz'
  if (x && y) return 'xy'
  if (x && z) return 'xz'
  if (y && z) return 'yz'
  if (x) return 'x'
  if (y) return 'y'
  if (z) return 'z'
  return 'none'
}

const temporalSamplesFor = (input: DiscoveryInput): TemporalSamples => {
  switch (input.kind) {
    case 'dense': return { kind: 'dense' }
    case 'role-aware': {
      const cutOff = (() => {
        switch (input.missingness.cutOff) {
          case 'method-default': return 'methodDefault'
          case '2xtau-max': return 'twoTauMax'
          case 'tau-max': return 'tauMax'
          case 'max-lag': return 'maxLag'
          case 'max-lag-or-tau-max': return 'maxLagOrTauMax'
          case '2xtau-max-future': return 'twoTauMaxFuture'
          default: return assertNever(input.missingness.cutOff)
        }
      })()
      return {
        kind: 'role-aware',
        validity: input.matrix.validity,
        analysisMask: input.matrix.analysisMask,
        cutOff,
        propagateThroughMaxLag: input.missingness.propagateThroughMaxLag,
        maskType: maskTypeFor(input.missingness.analysisExclusions),
      }
    }
    default: return assertNever(input)
  }
}

const validityFor = (input: DiscoveryInput): Uint8Array => {
  switch (input.kind) {
    case 'dense': return new Uint8Array(input.matrix.values.length).fill(1)
    case 'role-aware': return input.matrix.validity
    default: return assertNever(input)
  }
}

const methodIdForChoice = (method: DiscoveryMethodChoice) => {
  switch (method) {
    case 'direct-lingam': return DIRECT_LINGAM_METHOD_ID
    case 'pcmci-plus': return PCMCI_PLUS_PAR_CORR_METHOD_ID
    case 'lpcmci': return LPCMCI_PAR_CORR_METHOD_ID
    case 'rpcmci': return RPCMCI_PAR_CORR_METHOD_ID
    case 'cdnots': return CDNOTS_PAR_CORR_METHOD_ID
    case 'cdnots-plus': return CDNOTS_PLUS_PAR_CORR_METHOD_ID
    case 'grace': return GRACE_METHOD_ID
    case 'dynotears': return DYNOTEARS_METHOD_ID
    case 'var-lingam': return VAR_LINGAM_METHOD_ID
    case 'ocse': return OCSE_METHOD_ID
    case 'cmlp': return CMLP_METHOD_ID
    case 'clstm': return CLSTM_METHOD_ID
    default: return assertNever(method)
  }
}

const methodIdOf = (configuration: DiscoveryConfiguration) => methodIdForChoice(configuration.kind)

const eligibilityHint = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'Available: pre-run checks completed.'
    case 'caution': return 'Review: runnable, with conditions to assess.'
    case 'refused': return `Unavailable: ${eligibility.violations[0]?.evidence ?? 'a requirement is not met.'}`
    default: return assertNever(eligibility)
  }
}

const analysisFailureEvent = (problem: AnalysisWorkerProblem) => {
  switch (problem.kind) {
    case 'analysis-cancelled': return { type: 'run-cancelled' } as const
    case 'kernel-refused':
    case 'wasm-unavailable':
    case 'worker-unavailable':
    case 'worker-protocol-failed':
      return { type: 'run-failed', problem: { kind: 'analysis-refused', detail: problem.detail } } as const
    default: return assertNever(problem)
  }
}

const pValue = (value: number): string => value < 0.0001 ? '<0.0001' : value.toFixed(4)
const statistic = (value: number): string => Math.abs(value) >= 10_000 ? value.toExponential(4) : value.toFixed(6)

const asNumber = (value: string | number): number => (typeof value === 'number' ? value : Number(value))

interface LinkRow { readonly source: string; readonly target: string; readonly lag: number }

const linkColumns = <Row extends LinkRow>(): readonly EvidenceColumn<Row>[] => [
  { id: 'source', header: 'Source', value: (row) => row.source },
  { id: 'target', header: 'Target', value: (row) => row.target },
  { id: 'lag', header: 'Lag', align: 'right', value: (row) => row.lag },
]

const figureColumn = <Row,>(id: string, header: string, value: (row: Row) => number, print: (value: number) => string = statistic): EvidenceColumn<Row> =>
  ({ id, header, align: 'right', value, format: (value) => print(asNumber(value)) })

type RpcmciConfiguration = Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }>

function RpcmciControls({ configuration, onChange }: {
  readonly configuration: RpcmciConfiguration
  readonly onChange: (configuration: RpcmciConfiguration) => void
}) {
  const changeNumber = (fieldName: Exclude<keyof RpcmciConfiguration, 'kind'>, value: number) => {
    if (Number.isFinite(value)) onChange({ ...configuration, [fieldName]: value })
  }
  return (
    <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
      <div className="text-body text-ink">
        <ParameterLabel label="Regimes" help={DISCOVERY_PARAMETER_HELP.rpcmci.regimes} htmlFor="rpcmci-regimes" />
        <Select id="rpcmci-regimes" className={field('text', 'mt-1')} value={configuration.numRegimes} onChange={(event) => changeNumber('numRegimes', Number(event.target.value))}>
          {[2, 3, 4, 5, 6].map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum transitions" help={DISCOVERY_PARAMETER_HELP.rpcmci.maximumTransitions} htmlFor="rpcmci-maximum-transitions" />
        <input id="rpcmci-maximum-transitions" className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.maxTransitions} onChange={(event) => changeNumber('maxTransitions', event.currentTarget.valueAsNumber)} />
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.rpcmci.maximumLag} htmlFor="rpcmci-maximum-lag" />
        <Select id="rpcmci-maximum-lag" className={field('text', 'mt-1')} value={configuration.tauMax} onChange={(event) => changeNumber('tauMax', Number(event.target.value))}>
          {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Graph alpha" help={DISCOVERY_PARAMETER_HELP.rpcmci.graphAlpha} htmlFor="rpcmci-graph-alpha" />
        <Select id="rpcmci-graph-alpha" className={field('text', 'mt-1')} value={configuration.alphaLevel} onChange={(event) => changeNumber('alphaLevel', Number(event.target.value))}>
          {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <details className={well('@md/panel:col-span-2 @2xl/panel:col-span-3 px-3 py-2')}>
        <summary className="cursor-pointer text-body text-ink">Annealing and conditional-independence settings</summary>
        <div className="mt-3 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
          <div className="text-body text-ink">
            <ParameterLabel label="Minimum lag" help={DISCOVERY_PARAMETER_HELP.rpcmci.minimumLag} htmlFor="rpcmci-minimum-lag" />
            <Select id="rpcmci-minimum-lag" className={field('text', 'mt-1')} value={configuration.tauMin} onChange={(event) => changeNumber('tauMin', Number(event.target.value))}>
              {Array.from({ length: configuration.tauMax + 1 }, (_, value) => value).map((value) => <option key={value} value={value}>{value}</option>)}
            </Select>
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="PC alpha" help={DISCOVERY_PARAMETER_HELP.rpcmci.pcAlpha} htmlFor="rpcmci-pc-alpha" />
            <Select id="rpcmci-pc-alpha" className={field('text', 'mt-1')} value={configuration.pcAlpha} onChange={(event) => changeNumber('pcAlpha', Number(event.target.value))}>
              {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
            </Select>
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Switching threshold" help={DISCOVERY_PARAMETER_HELP.rpcmci.switchingThreshold} htmlFor="rpcmci-switching-threshold" />
            <input id="rpcmci-switching-threshold" className={field('text', 'mt-1')} type="number" min={0} max={1} step={0.01} value={configuration.switchThres} onChange={(event) => changeNumber('switchThres', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Iterations per annealing" help={DISCOVERY_PARAMETER_HELP.rpcmci.iterationsPerAnnealing} htmlFor="rpcmci-iterations" />
            <input id="rpcmci-iterations" className={field('text', 'mt-1')} type="number" min={1} max={100} step={1} value={configuration.numIterations} onChange={(event) => changeNumber('numIterations', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Annealing runs" help={DISCOVERY_PARAMETER_HELP.rpcmci.annealingRuns} htmlFor="rpcmci-annealing-runs" />
            <input id="rpcmci-annealing-runs" className={field('text', 'mt-1')} type="number" min={1} max={50} step={1} value={configuration.maxAnneal} onChange={(event) => changeNumber('maxAnneal', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Seed" help={DISCOVERY_PARAMETER_HELP.rpcmci.seed} htmlFor="rpcmci-seed" />
            <input id="rpcmci-seed" className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.seed} onChange={(event) => changeNumber('seed', event.currentTarget.valueAsNumber)} />
          </div>
        </div>
      </details>
    </div>
  )
}

const CDN_CONTEXT_OPTIONS = [
  { value: 'none', label: 'None' },
  { value: 'linear', label: 'Linear time' },
  { value: 'linearSine', label: 'Linear + sine' },
  { value: 'linearExponential', label: 'Linear + exponential' },
  { value: 'linearQuadratic', label: 'Linear + quadratic' },
  { value: 'step', label: 'Step' },
  { value: 'stepLinear', label: 'Step + linear' },
] as const

type CdnotsConfiguration = Extract<DiscoveryConfiguration, { readonly kind: 'cdnots' | 'cdnots-plus' }>

function CdnotsControls({ configuration, onChange }: {
  readonly configuration: CdnotsConfiguration
  readonly onChange: (configuration: CdnotsConfiguration) => void
}) {
  return (
    <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.cdnots.maximumLag} htmlFor="cdnots-maximum-lag" />
        <Select id="cdnots-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
          const maxLag = lagFromValue(event.target.value)
          if (maxLag !== null) onChange({ ...configuration, maxLag })
        }}>
          {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Alpha" help={DISCOVERY_PARAMETER_HELP.cdnots.alpha} htmlFor="cdnots-alpha" />
        <Select id="cdnots-alpha" className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => {
          const alpha = alphaFromValue(event.target.value)
          if (alpha !== null) onChange({ ...configuration, alpha })
        }}>
          {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Missing observations" help={DISCOVERY_PARAMETER_HELP.cdnots.missing} htmlFor="cdnots-missing" />
        <Select id="cdnots-missing" className={field('text', 'mt-1')} value={configuration.missing} onChange={(event) => {
          const missing = event.target.value
          if (missing === 'pairwiseComplete' || missing === 'varEm') onChange({ ...configuration, missing })
        }}>
          <option value="pairwiseComplete">Pairwise complete</option>
          <option value="varEm">VAR-EM imputation</option>
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Time context" help={DISCOVERY_PARAMETER_HELP.cdnots.context} htmlFor="cdnots-context" />
        <Select id="cdnots-context" className={field('text', 'mt-1')} value={configuration.context} onChange={(event) => {
          const option = CDN_CONTEXT_OPTIONS.find(({ value }) => value === event.target.value)
          if (option !== undefined) onChange({ ...configuration, context: option.value })
        }}>
          {CDN_CONTEXT_OPTIONS.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
        </Select>
      </div>
    </div>
  )
}

type GraceConfiguration = Extract<DiscoveryConfiguration, { readonly kind: 'grace' }>

function GraceControls({ configuration, onChange }: {
  readonly configuration: GraceConfiguration
  readonly onChange: (configuration: GraceConfiguration) => void
}) {
  const changeNumber = (fieldName: 'gateThreshold' | 'epochs' | 'patience' | 'seed', value: number) => {
    if (Number.isFinite(value)) onChange({ ...configuration, [fieldName]: value })
  }
  return (
    <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
      <div className="text-body text-ink">
        <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.grace.maximumLag} htmlFor="grace-maximum-lag" />
        <Select id="grace-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
          const maxLag = lagFromValue(event.target.value)
          if (maxLag !== null) onChange({ ...configuration, maxLag })
        }}>
          {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Skeleton alpha" help={DISCOVERY_PARAMETER_HELP.grace.alpha} htmlFor="grace-alpha" />
        <Select id="grace-alpha" className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => {
          const alpha = alphaFromValue(event.target.value)
          if (alpha !== null) onChange({ ...configuration, alpha })
        }}>
          {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Time context" help={DISCOVERY_PARAMETER_HELP.grace.context} htmlFor="grace-context" />
        <Select id="grace-context" className={field('text', 'mt-1')} value={configuration.context} onChange={(event) => {
          const option = CDN_CONTEXT_OPTIONS.find(({ value }) => value === event.target.value)
          if (option !== undefined) onChange({ ...configuration, context: option.value })
        }}>
          {CDN_CONTEXT_OPTIONS.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
        </Select>
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Gate threshold" help={DISCOVERY_PARAMETER_HELP.grace.gateThreshold} htmlFor="grace-gate-threshold" />
        <input id="grace-gate-threshold" className={field('text', 'mt-1')} type="number" min={0} max={1} step={0.05} value={configuration.gateThreshold} onChange={(event) => changeNumber('gateThreshold', event.currentTarget.valueAsNumber)} />
      </div>
      <details className={well('@md/panel:col-span-2 @2xl/panel:col-span-3 px-3 py-2')}>
        <summary className="cursor-pointer text-body text-ink">Training settings</summary>
        <div className="mt-3 grid gap-3 @md/panel:grid-cols-3">
          <div className="text-body text-ink">
            <ParameterLabel label="Epochs" help={DISCOVERY_PARAMETER_HELP.grace.epochs} htmlFor="grace-epochs" />
            <input id="grace-epochs" className={field('text', 'mt-1')} type="number" min={1} max={5000} step={1} value={configuration.epochs} onChange={(event) => changeNumber('epochs', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Patience" help={DISCOVERY_PARAMETER_HELP.grace.patience} htmlFor="grace-patience" />
            <input id="grace-patience" className={field('text', 'mt-1')} type="number" min={1} max={configuration.epochs} step={1} value={configuration.patience} onChange={(event) => changeNumber('patience', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Seed" help={DISCOVERY_PARAMETER_HELP.grace.seed} htmlFor="grace-seed" />
            <input id="grace-seed" className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.seed} onChange={(event) => changeNumber('seed', event.currentTarget.valueAsNumber)} />
          </div>
        </div>
      </details>
    </div>
  )
}

type NeuralConfiguration = Extract<DiscoveryConfiguration, { readonly kind: 'cmlp' | 'clstm' }>
type NeuralNumericField = 'lambda' | 'ridgeLambda' | 'learningRate' | 'maxIter' | 'checkEvery' | 'lookback' | 'seed'

function NeuralControls({ configuration, onChange }: {
  readonly configuration: NeuralConfiguration
  readonly onChange: (configuration: NeuralConfiguration) => void
}) {
  const changeNumber = (fieldName: NeuralNumericField, value: number) => {
    if (!Number.isFinite(value)) return
    if (configuration.kind === 'cmlp') onChange({ ...configuration, [fieldName]: value })
    else onChange({ ...configuration, [fieldName]: value })
  }
  const prefix = configuration.kind
  return (
    <div className="mt-4 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
      {configuration.kind === 'cmlp' ? (
        <div className="text-body text-ink">
          <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.neural.maximumLag} htmlFor="cmlp-lag" />
          <Select id="cmlp-lag" className={field('text', 'mt-1')} value={configuration.lag} onChange={(event) => {
            const value = lagFromValue(event.target.value)
            if (value !== null) onChange({ ...configuration, lag: value })
          }}>
            {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
          </Select>
        </div>
      ) : (
        <div className="text-body text-ink">
          <ParameterLabel label="Context length" help={DISCOVERY_PARAMETER_HELP.neural.context} htmlFor="clstm-context" />
          <input id="clstm-context" className={field('text', 'mt-1')} type="number" min={1} max={100} step={1} value={configuration.context} onChange={(event) => {
            const context = event.currentTarget.valueAsNumber
            if (Number.isFinite(context)) onChange({ ...configuration, context })
          }} />
        </div>
      )}
      <div className="text-body text-ink">
        <ParameterLabel label="Hidden width" help={DISCOVERY_PARAMETER_HELP.neural.hiddenWidth} htmlFor={`${prefix}-hidden`} />
        <input id={`${prefix}-hidden`} className={field('text', 'mt-1')} type="number" min={1} max={256} step={1} value={configuration.kind === 'cmlp' ? configuration.hidden[0] : configuration.hidden} onChange={(event) => {
          const value = event.currentTarget.valueAsNumber
          if (!Number.isFinite(value)) return
          onChange(configuration.kind === 'cmlp' ? { ...configuration, hidden: [value] } : { ...configuration, hidden: value })
        }} />
      </div>
      <div className="text-body text-ink">
        <ParameterLabel label="Sparsity λ" help={DISCOVERY_PARAMETER_HELP.neural.sparsity} htmlFor={`${prefix}-lambda`} />
        <input id={`${prefix}-lambda`} className={field('text', 'mt-1')} type="number" min={0} step={0.001} value={configuration.lambda} onChange={(event) => changeNumber('lambda', event.currentTarget.valueAsNumber)} />
      </div>
      {configuration.kind === 'cmlp' && (
        <>
          <div className="text-body text-ink">
            <ParameterLabel label="Activation" help={DISCOVERY_PARAMETER_HELP.neural.activation} htmlFor="cmlp-activation" />
            <Select id="cmlp-activation" className={field('text', 'mt-1')} value={configuration.activation} onChange={(event) => {
              const activation = event.target.value
              if (activation === 'sigmoid' || activation === 'tanh' || activation === 'relu' || activation === 'leakyRelu' || activation === 'identity') onChange({ ...configuration, activation })
            }}>
              <option value="relu">ReLU</option>
              <option value="leakyRelu">Leaky ReLU</option>
              <option value="tanh">Tanh</option>
              <option value="sigmoid">Sigmoid</option>
              <option value="identity">Identity</option>
            </Select>
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Structured penalty" help={DISCOVERY_PARAMETER_HELP.neural.penalty} htmlFor="cmlp-penalty" />
            <Select id="cmlp-penalty" className={field('text', 'mt-1')} value={configuration.penalty} onChange={(event) => {
              const penalty = event.target.value
              if (penalty === 'groupLasso' || penalty === 'groupSparseGroupLasso' || penalty === 'hierarchical') onChange({ ...configuration, penalty })
            }}>
              <option value="hierarchical">Hierarchical</option>
              <option value="groupLasso">Group lasso</option>
              <option value="groupSparseGroupLasso">Group sparse group lasso</option>
            </Select>
          </div>
        </>
      )}
      <details className={well('@md/panel:col-span-2 @2xl/panel:col-span-3 px-3 py-2')}>
        <summary className="cursor-pointer text-body text-ink">Training settings</summary>
        <div className="mt-3 grid gap-3 @md/panel:grid-cols-2 @2xl/panel:grid-cols-3">
          <div className="text-body text-ink">
            <ParameterLabel label="Ridge λ" help={DISCOVERY_PARAMETER_HELP.neural.ridge} htmlFor={`${prefix}-ridge`} />
            <input id={`${prefix}-ridge`} className={field('text', 'mt-1')} type="number" min={0} step={0.001} value={configuration.ridgeLambda} onChange={(event) => changeNumber('ridgeLambda', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Learning rate" help={DISCOVERY_PARAMETER_HELP.neural.learningRate} htmlFor={`${prefix}-learning-rate`} />
            <input id={`${prefix}-learning-rate`} className={field('text', 'mt-1')} type="number" min={0.000001} step={0.001} value={configuration.learningRate} onChange={(event) => changeNumber('learningRate', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Maximum iterations" help={DISCOVERY_PARAMETER_HELP.neural.iterations} htmlFor={`${prefix}-iterations`} />
            <input id={`${prefix}-iterations`} className={field('text', 'mt-1')} type="number" min={1} max={configuration.kind === 'cmlp' ? 50_000 : 20_000} step={1} value={configuration.maxIter} onChange={(event) => changeNumber('maxIter', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Check every" help={DISCOVERY_PARAMETER_HELP.neural.checkEvery} htmlFor={`${prefix}-check-every`} />
            <input id={`${prefix}-check-every`} className={field('text', 'mt-1')} type="number" min={1} max={configuration.maxIter} step={1} value={configuration.checkEvery} onChange={(event) => changeNumber('checkEvery', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Early-stop lookback" help={DISCOVERY_PARAMETER_HELP.neural.lookback} htmlFor={`${prefix}-lookback`} />
            <input id={`${prefix}-lookback`} className={field('text', 'mt-1')} type="number" min={1} step={1} value={configuration.lookback} onChange={(event) => changeNumber('lookback', event.currentTarget.valueAsNumber)} />
          </div>
          <div className="text-body text-ink">
            <ParameterLabel label="Seed" help={DISCOVERY_PARAMETER_HELP.neural.seed} htmlFor={`${prefix}-seed`} />
            <input id={`${prefix}-seed`} className={field('text', 'mt-1')} type="number" min={0} step={1} value={configuration.seed} onChange={(event) => changeNumber('seed', event.currentTarget.valueAsNumber)} />
          </div>
        </div>
      </details>
    </div>
  )
}


function ResultEligibility({ eligibility }: { readonly eligibility: MethodEligibility }) {
  switch (eligibility.kind) {
    case 'eligible': return <span className="text-ok">Available · {eligibility.satisfied.length} requirements checked</span>
    case 'caution': return <span className="text-warn">Available · {eligibility.unresolved.length} requirements to review</span>
    case 'refused': return <span className="text-danger">Unavailable · {eligibility.violations.length} requirements fail</span>
    default: return assertNever(eligibility)
  }
}

function RunRecord({ run }: { readonly run: DiscoveryRunArtifact }) {
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <summary className="cursor-pointer text-ink">Run details</summary>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Prepared dataset</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        <dt>Method</dt><dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </details>
  )
}

/** One run's card: a disclosure whose summary carries the method, title and figures, so a reader can keep one run open and fold the rest. */
function ResultCard({ run, method, title, meta, open, current, children }: {
  readonly run: DiscoveryRunArtifact
  readonly method: string
  readonly title: ReactNode
  readonly meta: ReactNode
  readonly open: boolean
  /** The newest run: the one card that carries the emphasised border. */
  readonly current: boolean
  readonly children: ReactNode
}) {
  return (
    <article aria-labelledby={`run-${run.id}`}>
      <details className={`group rounded-xl border bg-panel ${current ? 'border-edge' : 'border-hair'}`} open={open}>
        <summary className="flex cursor-pointer list-none items-start gap-3 rounded-xl py-4 pl-4 pr-14 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={16} className="mt-1 shrink-0 text-faint transition-transform duration-150 group-open:rotate-180" />
          <div className="min-w-0 flex-1">
            <span className={label(current ? 'text-signal' : 'text-faint')}>{method}</span>
            <h3 id={`run-${run.id}`} className="mb-1 mt-1 text-title font-medium text-ink">{title}</h3>
            <p className="m-0 text-body text-faint">{meta}</p>
          </div>
        </summary>
        <div className="px-4 pb-4">
          <ResultInterpretation interpretation={interpretDiscoveryResult(run)} className="mb-3" />
          {children}
        </div>
      </details>
    </article>
  )
}

type TimeGraphRun = Extract<DiscoveryRunArtifact, { readonly kind: 'pcmci-plus-run' | 'lpcmci-run' }>

/**
 * The scope, source, target and lag controls for a lag-graph evidence table.
 *
 * Every ordered pair appears at every lag, so the unfiltered table is mostly absences. These
 * controls answer the questions a reader actually brings to it.
 */
function EvidenceScopeControls({ selection, onChange, variables, tauMax, alpha }: {
  readonly selection: EvidenceSelection
  readonly onChange: (next: EvidenceSelection) => void
  readonly variables: readonly string[]
  readonly tauMax: number
  readonly alpha: number
}) {
  const lags = Array.from({ length: tauMax + 1 }, (_, lag) => lag)
  return (
    <>
      <SegmentedControl<EvidenceScope['kind']>
        size="sm"
        ariaLabel="Which cells to show"
        value={selection.scope.kind}
        onChange={(kind) => onChange({ ...selection, scope: EVIDENCE_SCOPES.find((scope) => scope.kind === kind) ?? { kind: 'all' } })}
        options={EVIDENCE_SCOPES.map((scope) => ({ value: scope.kind, label: describeEvidenceScope(scope), title: explainEvidenceScope(scope, alpha) }))}
      />
      <Select className={field('text', 'w-28')} aria-label="Filter by source" value={selection.source ?? ''} onChange={(event) => onChange({ ...selection, source: event.target.value === '' ? null : event.target.value })}>
        <option value="">Any source</option>
        {variables.map((name) => <option key={name} value={name}>{name}</option>)}
      </Select>
      <Select className={field('text', 'w-28')} aria-label="Filter by target" value={selection.target ?? ''} onChange={(event) => onChange({ ...selection, target: event.target.value === '' ? null : event.target.value })}>
        <option value="">Any target</option>
        {variables.map((name) => <option key={name} value={name}>{name}</option>)}
      </Select>
      <Select className={field('text', 'w-24')} aria-label="Filter by lag" value={selection.lag === null ? '' : String(selection.lag)} onChange={(event) => onChange({ ...selection, lag: event.target.value === '' ? null : Number(event.target.value) })}>
        <option value="">Any lag</option>
        {lags.map((lag) => <option key={lag} value={lag}>lag {lag}</option>)}
      </Select>
    </>
  )
}

function TimeGraphResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: TimeGraphRun }) {
  const [selection, setSelection] = useState<EvidenceSelection>(NO_EVIDENCE_SELECTION)
  const rows = useMemo(
    () => run.result.graph.flatMap((targets, sourceIndex) =>
      targets.flatMap((lags, targetIndex) =>
        lags.map((mark, lag) => ({
          key: `${sourceIndex}:${targetIndex}:${lag}`,
          source: run.variables[sourceIndex].name,
          target: run.variables[targetIndex].name,
          lag,
          mark,
          p: run.result.pMatrix[sourceIndex][targetIndex][lag],
          value: run.result.valMatrix[sourceIndex][targetIndex][lag],
        })),
      ),
    ),
    [run],
  )
  const selected = useMemo(
    () => (selectsEverything(selection) ? rows : rows.filter((row) => matchesEvidenceSelection(row, selection, run.result.pcAlpha))),
    [rows, selection, run.result.pcAlpha],
  )
  const variableNames = useMemo(() => run.variables.map((variable) => variable.name), [run])
  const isLpcmci = run.kind === 'lpcmci-run'
  const methodLabel = isLpcmci ? 'LPCMCI · ParCorr' : 'PCMCI+ · ParCorr'
  const resultTitle = isLpcmci ? 'Latent-aware partial ancestral graph evidence' : 'Stationary lag-graph evidence'
  const tableLabel = isLpcmci ? 'LPCMCI raw evidence' : 'PCMCI+ raw evidence'
  return (
    <ResultCard run={run} open={open} current={current} method={methodLabel} title={resultTitle} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.tauMax} · alpha {run.result.pcAlpha}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label={`${methodLabel} structure`} />
      <TimeGraphPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Empty marks appear as “—”. {isLpcmci ? 'The legend under the structure view defines each mark.' : 'Unoriented same-period links keep the o-o mark.'}</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title={tableLabel}
        rows={selected}
        total={rows.length}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        exportName={`${isLpcmci ? 'lpcmci' : 'pcmci-plus'}-evidence`}
        filters={<EvidenceScopeControls selection={selection} onChange={setSelection} variables={variableNames} tauMax={run.result.tauMax} alpha={run.result.pcAlpha} />}
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'mark', header: 'Mark', mono: true, value: (row) => row.mark, format: (value) => (value === '' ? '—' : value) },
          figureColumn<typeof rows[number]>('p', 'p', (row) => row.p, pValue),
          figureColumn<typeof rows[number]>('value', 'ParCorr', (row) => row.value),
        ]}
      />
    </ResultCard>
  )
}

function RpcmciResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'rpcmci-run' }> }) {
  const [regime, setRegime] = useState(0)
  const [selection, setSelection] = useState<EvidenceSelection>(NO_EVIDENCE_SELECTION)
  const graph = run.result.graphs[regime]
  const pMatrix = run.result.pMatrices[regime]
  const valMatrix = run.result.valMatrices[regime]
  const rows = useMemo(
    () => graph.flatMap((targets, sourceIndex) => targets.flatMap((lags, targetIndex) => lags.map((mark, lag) => ({
      key: `${regime}:${sourceIndex}:${targetIndex}:${lag}`,
      source: run.variables[sourceIndex].name,
      target: run.variables[targetIndex].name,
      lag,
      mark,
      p: pMatrix[sourceIndex][targetIndex][lag],
      value: valMatrix[sourceIndex][targetIndex][lag],
    })))),
    [graph, pMatrix, regime, run.variables, valMatrix],
  )
  const selected = useMemo(
    () => (selectsEverything(selection) ? rows : rows.filter((row) => matchesEvidenceSelection(row, selection, run.result.alphaLevel))),
    [rows, run.result.alphaLevel, selection],
  )
  const variableNames = useMemo(() => run.variables.map((variable) => variable.name), [run.variables])
  return (
    <ResultCard
      run={run}
      open={open}
      current={current}
      method="RPCMCI · ParCorr"
      title={<>Regime-dependent lag-graph evidence</>}
      meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · {run.result.numRegimes} regimes · lags {run.result.tauMin}–{run.result.tauMax} · graph alpha {run.result.alphaLevel}</>}
    >
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <RpcmciMembershipPlot run={run} />
      <div className="mt-3 flex items-center gap-2">
        <label className="text-body text-ink">
          Regime shown
          <Select className={field('text', 'ml-2 w-auto')} value={regime} onChange={(event) => setRegime(Number(event.target.value))}>
            {run.result.graphs.map((_, index) => <option key={index} value={index}>Regime {index + 1}</option>)}
          </Select>
        </label>
      </div>
      <StructurePlot run={run} regime={regime} label={`RPCMCI regime ${regime + 1} structure`} />
      <RpcmciTimeGraphPlot run={run} regime={regime} />
      <p className="mb-3 mt-3 text-body text-muted">The selected regime changes both the graph and its partial-correlation matrix. Regime numbers are labels and may be exchanged without changing the fitted model.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title={`RPCMCI regime ${regime + 1} raw evidence`}
        rows={selected}
        total={rows.length}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        exportName={`rpcmci-regime-${regime + 1}-evidence`}
        filters={<EvidenceScopeControls selection={selection} onChange={setSelection} variables={variableNames} tauMax={run.result.tauMax} alpha={run.result.alphaLevel} />}
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'mark', header: 'Mark', mono: true, value: (row) => row.mark, format: (value) => (value === '' ? '—' : value) },
          figureColumn<typeof rows[number]>('p', 'p', (row) => row.p, pValue),
          figureColumn<typeof rows[number]>('value', 'ParCorr', (row) => row.value),
        ]}
      />
      <p className="mb-0 mt-3 text-micro text-faint">{run.result.errorFreeAnnealings} of {run.result.maxAnneal} annealing runs completed without an optimisation error · switch threshold {run.result.switchThres} · transition budget {run.result.maxTransitions} · seed {run.result.seed}</p>
    </ResultCard>
  )
}

type CdnotsRun = Extract<DiscoveryRunArtifact, { readonly kind: 'cdnots-run' | 'cdnots-plus-run' }>

function CdnotsResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: CdnotsRun }) {
  const method = run.kind === 'cdnots-run' ? 'CD-NOTS' : 'CD-NOTS+'
  const names = [...run.variables.map(({ name }) => name), ...run.result.contextVariables]
  const rows = run.result.graph.flatMap((targets, source) => targets.flatMap((lags, target) => lags.map((mark, lag) => ({
    key: `${source}:${target}:${lag}`,
    source: names[source],
    target: names[target],
    lag,
    mark,
    p: run.result.pMatrix[source][target][lag],
    value: run.result.valMatrix[source][target][lag],
  }))))
  const context = run.result.contextVariables.length === 0 ? 'no time-context node' : run.result.contextVariables.join(' + ')
  return (
    <ResultCard run={run} open={open} current={current} method={`${method} · ParCorr`} title={<>Nonstationary time-graph evidence</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.observedVariables} observed variables · maximum lag {run.result.maxLag} · {context}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label={`${method} structure`} />
      <p className="mb-3 mt-3 text-body text-muted">Generated context nodes appear in this evidence graph but are not dataset columns and cannot be copied into the editable DAG.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title={`${method} raw evidence`}
        rows={rows}
        rowKey={(row) => row.key}
        noun="cell"
        empty="The run reported no cell."
        exportName={`${run.kind === 'cdnots-run' ? 'cdnots' : 'cdnots-plus'}-evidence`}
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'mark', header: 'Mark', value: (row) => row.mark || '—' },
          figureColumn<typeof rows[number]>('p', 'p', (row) => row.p, pValue),
          figureColumn<typeof rows[number]>('value', 'Partial r', (row) => row.value),
        ]}
      />
    </ResultCard>
  )
}

function GraceResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'grace-run' }> }) {
  const rows = run.result.gateValues.flatMap((targets, source) => targets.flatMap((lags, target) => lags.map((gate, lag) => ({
    key: `${source}:${target}:${lag}`,
    source: run.variables[source].name,
    target: run.variables[target].name,
    lag,
    skeleton: run.result.skeleton[source][target][lag],
    gate,
    selected: run.result.graph[source][target][lag],
  }))))
  const finalLoss = run.result.loss.at(-1)
  const finalRmse = run.result.rmse.at(-1)
  return (
    <ResultCard run={run} open={open} current={current} method="GRACE" title={<>Gated lag-graph refinement</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.maxLag} · {run.result.epochs} epochs · seed {run.result.seed}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label="GRACE retained relations" />
      <dl className={figureGrid('mt-3 grid-cols-2 @2xl/panel:grid-cols-4')}>
        <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Gate threshold</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{statistic(run.result.gateThreshold)}</dd></div>
        <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>L0 λ</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{statistic(run.result.lambdaL0)}</dd></div>
        <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Final loss</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{finalLoss === undefined ? '—' : statistic(finalLoss)}</dd></div>
        <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Final RMSE</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{finalRmse === undefined ? '—' : statistic(finalRmse)}</dd></div>
      </dl>
      <p className="mb-3 mt-3 text-body text-muted">The skeleton limits which links can be trained. A link is retained when its fitted hard-concrete gate meets the recorded threshold. {run.result.imputedCells === 0 ? 'The neural input contained no missing cells.' : `VAR-EM filled ${formatCount(run.result.imputedCells).text} missing cells before the dense neural windows were constructed.`}</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="GRACE gate values"
        rows={rows}
        rowKey={(row) => row.key}
        noun="gate"
        empty="The run reported no gate."
        exportName="grace-gates"
        columns={[
          ...linkColumns<typeof rows[number]>(),
          { id: 'skeleton', header: 'In skeleton', value: (row) => row.skeleton ? 'Yes' : 'No' },
          figureColumn<typeof rows[number]>('gate', 'Gate', (row) => row.gate),
          { id: 'selected', header: 'Retained', value: (row) => row.selected ? 'Yes' : 'No' },
        ]}
      />
    </ResultCard>
  )
}

function DynotearsResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'dynotears-run' }> }) {
  const weights = [run.result.contemporaneousWeights, ...run.result.laggedWeights].flatMap((matrix, lag) =>
    matrix.flatMap((targets, source) => targets.map((weight, target) => ({ lag, source, target, weight }))),
  )
  const rows = weights.map((cell) => ({ key: `${cell.lag}:${cell.source}:${cell.target}`, source: run.variables[cell.source].name, target: run.variables[cell.target].name, lag: cell.lag, weight: cell.weight }))
  return (
    <ResultCard run={run} open={open} current={current} method="DYNOTEARS" title={<>Sparse dynamic structural equation model weights</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.maxLag} · λW {run.result.lambdaW} · λA {run.result.lambdaA}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label="DYNOTEARS structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">All fitted weights are retained without a display threshold. Lag 0 is contemporaneous; a row denotes source(t−lag) → target(t).</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="DYNOTEARS raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight)]}
      />
    </ResultCard>
  )
}

function VarLingamResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'var-lingam-run' }> }) {
  const weights = [run.result.contemporaneousWeights, ...run.result.laggedWeights].flatMap((matrix, lag) =>
    matrix.flatMap((targets, source) => targets.map((weight, target) => ({ lag, source, target, weight }))),
  )
  const rows = weights.map((cell) => ({ key: `${cell.lag}:${cell.source}:${cell.target}`, source: run.variables[cell.source].name, target: run.variables[cell.target].name, lag: cell.lag, weight: cell.weight }))
  const order = run.result.causalOrder.map((index) => run.variables[index]?.name ?? String(index))
  return (
    <ResultCard run={run} open={open} current={current} method="VAR-LiNGAM" title={<>Non-Gaussian structural vector autoregression weights</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · Bayesian information criterion lag {run.result.selectedLag} of at most {run.result.lags} · {run.result.prune ? 'adaptive-lasso pruned' : 'unpruned'}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-1 mt-3 text-body text-muted">Contemporaneous causal order from residual non-Gaussianity:</p>
      <p className={num('mb-3 mt-0 text-body text-ink')} aria-label="VAR-LiNGAM causal order">{order.join(' → ')}</p>
      <StructurePlot run={run} label="VAR-LiNGAM structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Lag 0 is contemporaneous; a row denotes source(t−lag) → target(t).</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="VAR-LiNGAM raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight)]}
      />
    </ResultCard>
  )
}

function DirectLingamResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'direct-lingam-run' }> }) {
  const rows = run.result.weights.flatMap((targets, source) => targets.map((weight, target) => ({
    key: `${source}:${target}`,
    source: run.variables[source].name,
    target: run.variables[target].name,
    weight,
  })))
  const order = run.result.causalOrder.map((index) => run.variables[index]?.name ?? String(index))
  return (
    <ResultCard run={run} open={open} current={current} method="DirectLiNGAM" title={<>Linear non-Gaussian directed structure</>} meta={<>{formatCount(run.result.observations).text} independent observations · {run.result.variables} variables · adaptive-lasso adjacency</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-1 mt-3 text-body text-muted">Causal order inferred from non-Gaussianity:</p>
      <p className={num('mb-3 mt-0 text-body text-ink')} aria-label="DirectLiNGAM causal order">{order.join(' → ')}</p>
      <StructurePlot run={run} label="DirectLiNGAM structure" />
      <WeightPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">A nonzero row denotes source → target. The coefficient is a fitted structural weight on the variables' observed scales, not an intervention-effect estimate.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="DirectLiNGAM raw weights"
        rows={rows}
        rowKey={(row) => row.key}
        noun="weight"
        empty="The run reported no weight."
        columns={[
          { id: 'source', header: 'Source', value: (row) => row.source },
          { id: 'target', header: 'Target', value: (row) => row.target },
          figureColumn<typeof rows[number]>('weight', 'Weight', (row) => row.weight),
        ]}
      />
    </ResultCard>
  )
}

function OcseResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'ocse-run' }> }) {
  const rows = run.result.edges.map((edge, index) => ({ key: `${edge.source}:${edge.target}:${edge.lag}:${index}`, source: run.variables[edge.source].name, target: run.variables[edge.target].name, lag: edge.lag, cmi: edge.cmi, pValue: edge.pValue }))
  return (
    <ResultCard run={run} open={open} current={current} method="Optimal causation entropy" title={<>Conditional-information network evidence</>} meta={<>{formatCount(run.result.observations).text} rows · maximum lag {run.result.maxLag} · {run.result.method} conditional mutual information · {run.result.nShuffles} shuffles · seed {run.result.seed}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <StructurePlot run={run} label="oCSE structure" />
      <OcsePlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">Selected lagged relations can be reviewed in the DAG workspace. They are not estimates of intervention effects.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="oCSE raw evidence"
        rows={rows}
        rowKey={(row) => row.key}
        noun="edge"
        empty="No edge survived forward and backward selection."
        columns={[...linkColumns<typeof rows[number]>(), figureColumn<typeof rows[number]>('cmi', 'CMI', (row) => row.cmi), figureColumn<typeof rows[number]>('p', 'p-value', (row) => row.pValue, pValue)]}
      />
    </ResultCard>
  )
}

function CmlpResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'cmlp-run' }> }) {
  const rows = run.result.lagOrder.flatMap((lag, position) => run.result.lagScores.flatMap((targets, source) =>
    targets.map((scores, target) => ({
      key: `${source}:${target}:${lag}`,
      source: run.variables[source].name,
      target: run.variables[target].name,
      lag,
      score: scores[position],
      active: run.result.lagActive[source][target][position],
    })),
  ))
  return (
    <ResultCard run={run} open={open} current={current} method="cMLP" title={<>Lag-resolved neural Granger evidence</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · maximum lag {run.result.lag} · {run.result.iterations} ISTA iterations · seed {run.result.seed}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-0 mt-3 text-body text-muted">Each selected column was centered and divided by its recorded population standard deviation before training.</p>
      <StructurePlot run={run} label="cMLP lag-resolved relations" />
      <NeuralSummaryPlot run={run} />
      <CmlpLagPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">A selected row means the source’s past at that lag contributes to predicting the target under the fitted component-wise network and sparsity penalty.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="cMLP lag-group scores"
        rows={rows}
        rowKey={(row) => row.key}
        noun="score"
        empty="The run reported no lag score."
        columns={[
          ...linkColumns<typeof rows[number]>(),
          figureColumn<typeof rows[number]>('score', 'Input norm', (row) => row.score),
          { id: 'active', header: 'Selected', value: (row) => row.active ? 'Yes' : 'No' },
        ]}
      />
    </ResultCard>
  )
}

function ClstmResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: Extract<DiscoveryRunArtifact, { readonly kind: 'clstm-run' }> }) {
  const rows = run.result.summaryScores.flatMap((targets, source) => targets.map((score, target) => ({
    key: `${source}:${target}`,
    source: run.variables[source].name,
    target: run.variables[target].name,
    score,
    active: run.result.summaryActive[source][target],
  })))
  return (
    <ResultCard run={run} open={open} current={current} method="cLSTM" title={<>Window-level neural Granger evidence</>} meta={<>{formatCount(run.result.observations).text} rows · {run.result.variables} variables · context {run.result.context} · {run.result.iterations} ISTA iterations · seed {run.result.seed}</>}>
      <p className="mb-0 mt-3 text-body"><ResultEligibility eligibility={run.eligibility} /></p>
      <RunRecord run={run} />
      <p className="mb-0 mt-3 text-body text-muted">Each selected column was centered and divided by its recorded population standard deviation before training.</p>
      <NeuralSummaryPlot run={run} />
      <p className="mb-3 mt-3 text-body text-muted">cLSTM selects whether a source history helps predict a target. It does not select an individual lag, so Hirmos does not render this result as a lag graph.</p>
      <EvidenceTable<typeof rows[number]>
        frame="none"
        title="cLSTM input-group scores"
        rows={rows}
        rowKey={(row) => row.key}
        noun="score"
        empty="The run reported no score."
        columns={[
          { id: 'source', header: 'Source', value: (row) => row.source },
          { id: 'target', header: 'Target', value: (row) => row.target },
          figureColumn<typeof rows[number]>('score', 'Input norm', (row) => row.score),
          { id: 'active', header: 'Selected', value: (row) => row.active ? 'Yes' : 'No' },
        ]}
      />
    </ResultCard>
  )
}

function DiscoveryResult({ run, open, current }: { readonly open: boolean; readonly current: boolean; readonly run: DiscoveryRunArtifact }) {
  switch (run.kind) {
    case 'direct-lingam-run': return <DirectLingamResult run={run} open={open} current={current} />
    case 'pcmci-plus-run': return <TimeGraphResult run={run} open={open} current={current} />
    case 'lpcmci-run': return <TimeGraphResult run={run} open={open} current={current} />
    case 'rpcmci-run': return <RpcmciResult run={run} open={open} current={current} />
    case 'cdnots-run': return <CdnotsResult run={run} open={open} current={current} />
    case 'cdnots-plus-run': return <CdnotsResult run={run} open={open} current={current} />
    case 'grace-run': return <GraceResult run={run} open={open} current={current} />
    case 'dynotears-run': return <DynotearsResult run={run} open={open} current={current} />
    case 'var-lingam-run': return <VarLingamResult run={run} open={open} current={current} />
    case 'ocse-run': return <OcseResult run={run} open={open} current={current} />
    case 'cmlp-run': return <CmlpResult run={run} open={open} current={current} />
    case 'clstm-run': return <ClstmResult run={run} open={open} current={current} />
    default: return assertNever(run)
  }
}

export function DiscoveryPanel({ source, profile, prepared, stationarity, runs, documents, draft, onEvent: dispatch, cancellation, onRun, onDeleteRun }: DiscoveryPanelProps) {
  const [expanded, setExpanded] = useState<'latest' | 'all' | 'none'>('latest')
  const [deletionDialog, setDeletionDialog] = useState<DiscoveryDeletionDialog>({ kind: 'closed' })
  const configuration = draft.configuration
  const methodId = methodIdOf(configuration)
  const selectedMethod = methodDefinition(methodId)
  const [visibleMethodGroup, setVisibleMethodGroup] = useState<DiscoveryMethodGroupId>(() => discoveryMethodGroupFor(configuration.kind).id)
  useEffect(() => setVisibleMethodGroup(discoveryMethodGroupFor(configuration.kind).id), [configuration.kind])
  const visibleGroup = discoveryMethodGroupById(visibleMethodGroup)
  const selectedMethodIsVisible = visibleGroup.methods.includes(configuration.kind)
  const preparedColumns = profile.columns.filter((column) => prepared.columns.includes(column.id))
  if (!selectedMethod.ok) {
    return <p role="alert" className="text-body text-danger">The selected discovery method is not registered.</p>
  }
  const method: MethodDefinition = selectedMethod.value
  const eligibility = evaluateDiscoveryEligibility(method, prepared, stationarity)
  const readiness = readyDiscoverySpecification(configuration, prepared)
  const methodOptions = visibleGroup.methods.flatMap((value) => {
    const definition = methodDefinition(methodIdForChoice(value))
    if (!definition.ok) return []
    const candidateEligibility = evaluateDiscoveryEligibility(definition.value, prepared, stationarity)
    return [{
      value,
      label: definition.value.name.replace(' with ParCorr', ''),
      hint: eligibilityHint(candidateEligibility),
      disabled: draft.job.kind === 'running' || candidateEligibility.kind === 'refused',
      title: candidateEligibility.kind === 'refused'
        ? `${definition.value.name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}`
        : undefined,
    }]
  })

  const requestDeletion = (run: DiscoveryRunArtifact) => {
    const decision = assessDiscoveryRunDeletion(runs.map((candidate) => candidate.id), documents, run.id)
    switch (decision.kind) {
      case 'deletable':
        setDeletionDialog({ kind: 'confirming', run, deletion: decision.deletion })
        return
      case 'referenced':
        setDeletionDialog({ kind: 'blocked', run, references: decision.references })
        return
      case 'not-found':
        setDeletionDialog({ kind: 'not-found', run })
        return
      default: return assertNever(decision)
    }
  }

  const execute = async () => {
    const specification = readyDiscoverySpecification(configuration, prepared)
    if (!selectedMethodIsVisible || !specification.ok || eligibility.kind === 'refused') return
    cancellation.requested = false
    dispatch({ type: 'run-started' })
    try {
      const [{ materialisePrepared, materialiseRoleAwarePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([
        import('@/data/prepared'),
        import('@/analysis/client'),
      ])
      const roleAwarePrepared = prepared.kind === 'prepared-time-series' && prepared.missingness.kind === 'lag-aware-exclusion'
        ? { prepared, missingness: prepared.missingness }
        : null
      let input: DiscoveryInput
      if (roleAwarePrepared === null) {
        const materialised = await materialisePrepared(source, profile, prepared, prepared.columns)
        if (!materialised.ok) {
          dispatch(materialised.error.kind === 'missing-values-remain'
            ? { type: 'run-failed', problem: { kind: 'missing-values-remain', cells: materialised.error.cells } }
            : { type: 'run-failed', problem: { kind: 'materialization-refused', detail: describePreparedMaterialisationProblem(materialised.error) } })
          return
        }
        input = { kind: 'dense', matrix: materialised.value }
      } else {
        const materialised = await materialiseRoleAwarePrepared(
          source,
          profile,
          roleAwarePrepared.prepared,
          roleAwarePrepared.prepared.columns,
        )
        if (!materialised.ok) {
          dispatch(materialised.error.kind === 'missing-values-remain'
            ? { type: 'run-failed', problem: { kind: 'missing-values-remain', cells: materialised.error.cells } }
            : { type: 'run-failed', problem: { kind: 'materialization-refused', detail: describePreparedMaterialisationProblem(materialised.error) } })
          return
        }
        input = {
          kind: 'role-aware',
          matrix: materialised.value,
          missingness: roleAwarePrepared.missingness,
        }
      }
      if (cancellation.requested) {
        dispatch({ type: 'run-cancelled' })
        return
      }
      const matrix = input.matrix

      switch (specification.value.kind) {
      case 'direct-lingam': {
        const result = await analysis.runDirectLingam(matrix.values, matrix.rowCount, matrix.columns.length, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'direct-lingam-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: DIRECT_LINGAM_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'pcmci-plus': {
        if (prepared.kind !== 'prepared-time-series') return
        const result = await analysis.runPcmciPlus(
          matrix.values,
          matrix.rowCount,
          matrix.columns.length,
          specification.value.tauMax,
          specification.value.pcAlpha,
          temporalSamplesFor(input),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = {
          kind: 'pcmci-plus-run',
          id: newDiscoveryRunId(),
          preparedDataset: prepared.id,
          createdAt: new Date().toISOString(),
          method: PCMCI_PLUS_PAR_CORR_METHOD_ID,
          variables: matrix.columns,
          eligibility,
          result: result.value,
        }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'lpcmci': {
        if (prepared.kind !== 'prepared-time-series') return
        const result = await analysis.runLpcmci(matrix.values, matrix.rowCount, matrix.columns.length, specification.value.tauMax, specification.value.pcAlpha, temporalSamplesFor(input), (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'lpcmci-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: LPCMCI_PAR_CORR_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'rpcmci': {
        const result = await analysis.runRpcmci(
          matrix.values,
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = {
          kind: 'rpcmci-run',
          id: newDiscoveryRunId(),
          preparedDataset: prepared.id,
          createdAt: new Date().toISOString(),
          method: RPCMCI_PAR_CORR_METHOD_ID,
          variables: matrix.columns,
          eligibility,
          result: result.value,
        }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'cdnots': {
        const result = await analysis.runCdnots(
          matrix.values,
          validityFor(input),
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'cdnots-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: CDNOTS_PAR_CORR_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'cdnots-plus': {
        const result = await analysis.runCdnotsPlus(
          matrix.values,
          validityFor(input),
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'cdnots-plus-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: CDNOTS_PLUS_PAR_CORR_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'grace': {
        const result = await analysis.runGrace(
          matrix.values,
          validityFor(input),
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'grace-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: GRACE_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'dynotears': {
        const result = await analysis.runDynotears(matrix.values, matrix.rowCount, matrix.columns.length, specification.value.maxLag, specification.value.lambdaW, specification.value.lambdaA, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'dynotears-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: DYNOTEARS_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'var-lingam': {
        const result = await analysis.runVarLingam(matrix.values, matrix.rowCount, matrix.columns.length, specification.value.maxLag, specification.value.prune, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'var-lingam-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: VAR_LINGAM_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'ocse': {
        const result = await analysis.runOcse(matrix.values, matrix.rowCount, matrix.columns.length, specification.value.maxLag, specification.value.alpha, specification.value.nShuffles, specification.value.method, specification.value.k, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'ocse-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: OCSE_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'cmlp': {
        const result = await analysis.runCmlp(
          matrix.values,
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'cmlp-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: CMLP_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
      case 'clstm': {
        const result = await analysis.runClstm(
          matrix.values,
          matrix.rowCount,
          matrix.columns.length,
          specification.value,
          (progress) => dispatch({ type: 'run-progressed', progress }),
        )
        if (!result.ok) {
          dispatch(analysisFailureEvent(result.error))
          return
        }
        const artifact: DiscoveryRunArtifact = { kind: 'clstm-run', id: newDiscoveryRunId(), preparedDataset: prepared.id, createdAt: new Date().toISOString(), method: CLSTM_METHOD_ID, variables: matrix.columns, eligibility, result: result.value }
        dispatch({ type: 'run-succeeded', artifact })
        onRun(artifact)
        return
      }
        default: return assertNever(specification.value)
      }
    } catch (cause: unknown) {
      dispatch({
        type: 'run-failed',
        problem: {
          kind: 'execution-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        },
      })
    }
  }

  const cancelRun = () => {
    cancellation.requested = true
    void import('@/analysis/client').then(({ cancelAnalysisRuns }) => cancelAnalysisRuns())
  }

  const inspector = (
    <div className="flex flex-col gap-4">
      <section aria-labelledby="prepared-input-title">
        <h3 id="prepared-input-title" className="mb-3 mt-0 text-body font-medium text-ink">Prepared dataset</h3>
        <dl className={figureGrid('m-0 grid-cols-2')}>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Structure</dt><dd className="m-0 mt-1 text-body text-ink">{prepared.kind === 'prepared-time-series' ? `Regular ${prepared.sampling.frequency} series` : prepared.kind === 'prepared-panel' ? `Panel · ${prepared.panel.units} units × ${prepared.panel.periods} periods` : 'Independent observations'}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Rows</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{formatCount(prepared.observations).text}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Variables</dt><dd className={num('m-0 mt-1 text-title text-ink')}>{prepared.columns.length}</dd></div>
          <div className="bg-panel px-3 py-2"><dt className={label('text-faint')}>Stationarity</dt><dd className="m-0 mt-1 text-body text-ink">{prepared.kind !== 'prepared-time-series' ? 'Not applicable' : stationarity === null ? 'Tests not run' : `${formatCount(stationarity.observations).text} rows tested`}</dd></div>
        </dl>
        {prepared.kind === 'prepared-time-series' && prepared.seriesTransforms.some((record) => record.transform.kind !== 'levels') && (
          <p className="mb-0 mt-3 text-body text-muted"><span className={label('text-faint')}>Prepared scale</span><br />{prepared.seriesTransforms.filter((record) => record.transform.kind !== 'levels').map((record) => `${profile.columns.find((column) => column.id === record.column)?.name ?? record.column}: ${describeSeriesTransform(record.transform)}`).join(' · ')}</p>
        )}
        <p className={literal('mb-0 mt-3 break-all text-micro text-faint')}>Dataset version {prepared.id.slice(0, 8)}</p>
      </section>
      {selectedMethodIsVisible
        ? <MethodCaveats methods={[method]} eligibility={eligibility} />
        : <p className="m-0 text-body text-faint">Choose a method from this family to review its requirements.</p>}
    </div>
  )

  const stage = (
    <section aria-labelledby="discovery-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>03 · Discovery lab</span>
        <h2 id="discovery-title" className="mb-2 mt-2 text-heading text-ink">Examine candidate relationships</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">Causal discovery uses patterns in data to propose relations between variables, including same-period and lagged relations when time is part of the study. In this chapter, choose a method suited to the observation structure and compare the candidate relations it produces. The result depends on the method's assumptions and does not establish a causal graph on its own.</p>
      </div>

      <div className="grid gap-4">
        <section className={panel('p-4')} aria-labelledby="discovery-method-title">
          <h3 id="discovery-method-title" className="mb-3 mt-0 text-title font-medium text-ink">Discovery method</h3>
          <SegmentedControl
            wrap
            size="sm"
            ariaLabel="Discovery method family"
            value={visibleMethodGroup}
            onChange={setVisibleMethodGroup}
            disabled={draft.job.kind === 'running'}
            options={DISCOVERY_METHOD_GROUPS.map((group) => ({ value: group.id, label: DISCOVERY_GROUP_LABELS[group.id], title: group.name }))}
          />
          <div className="mb-2 mt-3">
            <h4 className="m-0 text-body font-medium text-ink">{visibleGroup.name}</h4>
            <p className="mb-0 mt-0.5 max-w-[65ch] text-label text-faint">{visibleGroup.description}</p>
          </div>
          <RadioList frame="none"
            className="mt-1"
            legend="Discovery method"
            legendHidden
            value={configuration.kind}
            onChange={(method) => dispatch({ type: 'method-selected', method })}
            options={methodOptions}
          />
          {!selectedMethodIsVisible && <p className="mb-0 mt-3 text-body text-faint">Choose a method from this family to configure it.</p>}

          {selectedMethodIsVisible && (
            <>
          {(configuration.kind === 'pcmci-plus' || configuration.kind === 'lpcmci') && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.pcmci.maximumLag} htmlFor="pcmci-maximum-lag" />
                <Select
                  id="pcmci-maximum-lag"
                  className={field('text', 'mt-1')}
                  value={configuration.tauMax}
                  onChange={(event) => {
                    const value = lagFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'tau-max-selected', value })
                  }}
                >
                  {DISCOVERY_LAG_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="PC alpha" help={DISCOVERY_PARAMETER_HELP.pcmci.pcAlpha} htmlFor="pcmci-pc-alpha" />
                <Select
                  id="pcmci-pc-alpha"
                  className={field('text', 'mt-1')}
                  value={configuration.pcAlpha}
                  onChange={(event) => {
                    const value = alphaFromValue(event.target.value)
                    if (value !== null) dispatch({ type: 'pc-alpha-selected', value })
                  }}
                >
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
            </div>
          )}

          {configuration.kind === 'rpcmci' && (
            <RpcmciControls
              configuration={configuration}
              onChange={(next) => dispatch({ type: 'rpcmci-configured', configuration: next })}
            />
          )}

          {(configuration.kind === 'cdnots' || configuration.kind === 'cdnots-plus') && (
            <CdnotsControls
              configuration={configuration}
              onChange={(next) => dispatch({ type: 'cdnots-configured', configuration: next })}
            />
          )}

          {configuration.kind === 'grace' && (
            <GraceControls
              configuration={configuration}
              onChange={(next) => dispatch({ type: 'grace-configured', configuration: next })}
            />
          )}

          {configuration.kind === 'dynotears' && (
            <div className="mt-4 grid gap-3 @2xl/panel:grid-cols-3">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.dynotears.maximumLag} htmlFor="dynotears-maximum-lag" />
                <Select id="dynotears-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Contemporaneous λ" help={DISCOVERY_PARAMETER_HELP.dynotears.contemporaneousPenalty} htmlFor="dynotears-contemporaneous-penalty" />
                <Select id="dynotears-contemporaneous-penalty" className={field('text', 'mt-1')} value={configuration.lambdaW} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-w-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Lagged λ" help={DISCOVERY_PARAMETER_HELP.dynotears.laggedPenalty} htmlFor="dynotears-lagged-penalty" />
                <Select id="dynotears-lagged-penalty" className={field('text', 'mt-1')} value={configuration.lambdaA} onChange={(event) => {
                  const value = penaltyFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'dynotears-lambda-a-selected', value })
                }}>
                  {DYNOTEARS_PENALTY_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
            </div>
          )}

          {configuration.kind === 'var-lingam' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.varLingam.maximumLag} htmlFor="var-lingam-maximum-lag" />
                <Select id="var-lingam-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 6).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="self-end pb-2 text-body text-ink">
                <div className="flex items-center gap-1">
                  <label htmlFor="var-lingam-prune" className="flex items-center gap-2">
                    <input id="var-lingam-prune" type="checkbox" checked={configuration.prune} onChange={(event) => dispatch({ type: 'var-lingam-prune-selected', value: event.target.checked })} />
                    Adaptive-lasso pruning
                  </label>
                  <ParameterHelp label="Adaptive-lasso pruning" help={DISCOVERY_PARAMETER_HELP.varLingam.prune} />
                </div>
                <span className="block pl-6 text-faint">BIC selects the lag order up to the maximum.</span>
              </div>
            </div>
          )}

          {configuration.kind === 'ocse' && (
            <div className="mt-4 grid gap-3 @md/panel:grid-cols-2">
              <div className="text-body text-ink">
                <ParameterLabel label="Maximum lag" help={DISCOVERY_PARAMETER_HELP.ocse.maximumLag} htmlFor="ocse-maximum-lag" />
                <Select id="ocse-maximum-lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => {
                  const value = lagFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'max-lag-selected', value })
                }}>
                  {DISCOVERY_LAG_OPTIONS.filter((value) => value <= 8).map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Information estimator" help={DISCOVERY_PARAMETER_HELP.ocse.informationEstimator} htmlFor="ocse-information-estimator" />
                <Select id="ocse-information-estimator" className={field('text', 'mt-1')} value={configuration.method} onChange={(event) => {
                  if (event.target.value === 'gaussian' || event.target.value === 'knn') dispatch({ type: 'ocse-method-selected', value: event.target.value })
                }}>
                  <option value="gaussian">Gaussian conditional mutual information</option>
                  <option value="knn">k-nearest neighbours (k = 5)</option>
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Test alpha" help={DISCOVERY_PARAMETER_HELP.ocse.testAlpha} htmlFor="ocse-test-alpha" />
                <Select id="ocse-test-alpha" className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => {
                  const value = alphaFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-alpha-selected', value })
                }}>
                  {PCMCI_ALPHA_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
              <div className="text-body text-ink">
                <ParameterLabel label="Permutation shuffles" help={DISCOVERY_PARAMETER_HELP.ocse.permutationShuffles} htmlFor="ocse-permutation-shuffles" />
                <Select id="ocse-permutation-shuffles" className={field('text', 'mt-1')} value={configuration.nShuffles} onChange={(event) => {
                  const value = shufflesFromValue(event.target.value)
                  if (value !== null) dispatch({ type: 'ocse-shuffles-selected', value })
                }}>
                  {OCSE_SHUFFLE_OPTIONS.map((value) => <option key={value} value={value}>{value}</option>)}
                </Select>
              </div>
            </div>
          )}

          {(configuration.kind === 'cmlp' || configuration.kind === 'clstm') && (
            <NeuralControls
              configuration={configuration}
              onChange={(next) => dispatch({ type: 'neural-configured', configuration: next })}
            />
          )}


          <EligibilityView eligibility={eligibility} />
          {!readiness.ok && <p role="status" className="mb-0 mt-3 text-body text-faint">{describeDiscoveryReadiness(readiness.error)}</p>}
          {draft.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">{describeDiscoveryRunProblem(draft.job.problem)}</p></Alert>}
          {draft.job.kind === 'running' && draft.job.progress !== null && (
            <div className="mt-3" role="status" aria-live="polite">
              <div className="mb-1 flex items-center justify-between gap-3 text-micro text-faint">
                <span>{draft.job.progress.stage}</span>
                <span className={num()}>{draft.job.progress.completed} / {draft.job.progress.total}</span>
              </div>
              <div className="bar-live h-1.5 w-full overflow-hidden rounded-full bg-line" role="progressbar" aria-valuemin={0} aria-valuemax={draft.job.progress.total} aria-valuenow={draft.job.progress.completed}>
                <span className="bar-live__fill block rounded-full bg-signal" style={{ width: `${Math.round((draft.job.progress.completed / Math.max(1, draft.job.progress.total)) * 100)}%` }} />
              </div>
            </div>
          )}
          <div className="mt-4 flex items-center gap-3">
            <button
              type="button"
              className={button('signal')}
              disabled={!readiness.ok || eligibility.kind === 'refused' || draft.job.kind === 'running'}
              aria-busy={draft.job.kind === 'running'}
              onClick={() => void execute()}
            >
              Run {method.name}
            </button>
            {draft.job.kind === 'running' && <button type="button" className={button('quiet')} onClick={cancelRun}>Cancel run</button>}
          </div>
            </>
          )}
        </section>
      </div>

      <section aria-labelledby="discovery-runs-title">
        <div className="mb-3 flex items-end justify-between gap-3">
          <div>
            <h2 id="discovery-runs-title" className="m-0 text-title font-medium text-ink">Discovery runs</h2>
          </div>
          <div className="flex items-center gap-2">
            {runs.length > 1 && (
              <>
                <button type="button" className={button('quiet', 'h-7 px-2 text-label')} onClick={() => setExpanded('all')}>Expand all</button>
                <button type="button" className={button('quiet', 'h-7 px-2 text-label')} onClick={() => setExpanded('none')}>Collapse all</button>
              </>
            )}
            <span className={num('text-body text-faint')}>{runs.length} run{runs.length === 1 ? '' : 's'}</span>
          </div>
        </div>
        {deletionDialog.kind === 'blocked' && (
          <Alert tone="warn" className="mb-3">
            <div className="flex items-start justify-between gap-3">
              <div>
                <p className="m-0">The {discoveryRunName(deletionDialog.run)} run cannot be deleted because a DAG audit record refers to it.</p>
                <ul className="mb-0 mt-2 pl-5 text-label text-muted">
                  {deletionDialog.references.map((reference) => (
                    <li key={reference.kind === 'dag-origin-reference'
                      ? `origin:${reference.document}`
                      : `edge:${reference.document}:${reference.revision}:${reference.edge}:${reference.candidate}`}
                    >{describeDiscoveryReference(reference)}</li>
                  ))}
                </ul>
              </div>
              <button type="button" className={iconControl('quiet')} aria-label="Dismiss deletion notice" onClick={() => setDeletionDialog({ kind: 'closed' })}><Icon name="close" size={14} /></button>
            </div>
          </Alert>
        )}
        {deletionDialog.kind === 'not-found' && (
          <Alert tone="danger" className="mb-3">
            <div className="flex items-center justify-between gap-3">
              <p className="m-0">The {discoveryRunName(deletionDialog.run)} run is no longer present in the project.</p>
              <button type="button" className={iconControl('quiet')} aria-label="Dismiss deletion notice" onClick={() => setDeletionDialog({ kind: 'closed' })}><Icon name="close" size={14} /></button>
            </div>
          </Alert>
        )}
        {runs.length === 0 ? (
          <EmptyState>Choose a method and run discovery.</EmptyState>
        ) : (
          <div className="space-y-4">
            {[...runs].reverse().map((run, index) => (
              <div key={`${run.id}:${expanded}`} className="relative">
                <DiscoveryResult run={run} open={expanded === 'all' || (expanded === 'latest' && index === 0)} current={index === 0} />
                <button type="button" className={iconControl('danger', 'absolute right-4 top-4 z-10')} aria-label={`Delete ${discoveryRunName(run)} run`} title={`Delete ${discoveryRunName(run)} run`} onClick={() => requestDeletion(run)}><Icon name="delete" size={14} /></button>
              </div>
            ))}
          </div>
        )}
        <ConfirmDialog
          open={deletionDialog.kind === 'confirming'}
          title={deletionDialog.kind === 'confirming' ? `Delete ${discoveryRunName(deletionDialog.run)} run?` : 'Delete discovery run?'}
          danger
          confirmLabel="Delete run"
          message={deletionDialog.kind === 'confirming' ? 'The recorded result will be removed from this project and cannot be restored.' : ''}
          onConfirm={() => {
            if (deletionDialog.kind === 'confirming') onDeleteRun(deletionDialog.deletion)
          }}
          onClose={() => setDeletionDialog({ kind: 'closed' })}
        />
      </section>
    </section>
  )

  return <WorkbenchLayout id="discovery" stage={stage} inspector={{ title: 'Prepared dataset and method requirements', body: inspector }} />
}
