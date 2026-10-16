import type { EstimationRunEvent, PointwiseRetry } from '@/analysis/estimation/context'
import { useEstimationTools, useAnalysisRunTool } from '@/assistant/applicationTools'
import { useEncodingSettings } from '@/assistant/useEncodingSettings'
import { canConfigureEstimator } from '@/assistant/estimatorConfiguration'
import { estimateSummary, type AssistantRunOutcome } from '@/assistant/analysisRun'
import { describeEstimand } from '@/domain/study'
import { useStudySelectionTool } from '@/assistant/studySelection'
import { useJob } from '@/analysis/JobsProvider'
import { EChart } from '@/charts/EChart'
import { runComparisonOption, type RunComparisonRow } from '@/charts/estimation/runComparison'
import { useChartTheme } from '@/charts/theme'
import { EligibilityView } from '@/components/EligibilityView'
import { Icon } from '@/components/Icon'
import { IdentificationRecord } from '@/components/IdentificationRecord'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { JobNotice } from '@/components/ui/JobNotice'
import { Metadata } from '@/components/ui/Metadata'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterHelp, ParameterLabel } from '@/components/ui/ParameterLabel'
import { RadioList } from '@/components/ui/RadioList'
import {
  actionGap,
  button,
  chapterIntro,
  field,
  fieldHint,
  fieldLabel,
  fieldRow,
  literal,
  num,
  panel,
  sectionTitle,
  stepsStack,
} from '@/components/ui/recipes'
import { RunActions } from '@/components/ui/RunActions'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { useWorkflow, useWorkflowStore } from '@/components/WorkflowProvider'
import type { RunActivity } from '@/domain/activity'
import {
  describeDidCovariateRole,
  didCovariateRestrictions,
  staggeredCovariateRestrictions,
} from '@/domain/adjustedDid'
import { causalForestInputs } from '@/domain/causalForest'
import { forestAnalysisColumns, type ForestAnalysis } from '@/domain/causalForestAnalysis'
import { defaultConventionalDidInference } from '@/domain/conventionalDid'
import type { DagDocument } from '@/domain/dag'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { expandsDesign } from '@/domain/designMatrix'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  describeCovariance,
  configurationSettings,
  describeEstimator,
  describeFixedEffects,
  describeRun,
  ESTIMATOR_GROUPS,
  evaluateEstimatorEligibility,
  methodIdOf,
  propensityMethodForTarget,
  stationaryMarksOf,
  tLearnerInputs,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
  type EstimatorGroupId,
  type EstimatorId,
} from '@/domain/estimation'
import {
  estimationSelection,
  initialEstimationDraft,
  samePanelBinding,
  sameStudyDataBinding,
  type EstimationEvent,
  type PanelBinding,
  type StudyDataBinding,
} from '@/domain/estimationDraft'
import { ESTIMATION_METHODS, methodDefinition, type MethodEligibility } from '@/domain/methods'
import { assessPanelInterventionLayout, type PanelInterventionPreflight } from '@/domain/panel'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import {
  describeSeriesTransform,
  frequencyUnit,
  seriesTransformFor,
  type PreparedDatasetArtifact,
  type StationarityEvidenceArtifact,
} from '@/domain/preprocessing'
import { ridgeMethodDefinition } from '@/domain/ridgeAugmented'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import {
  describeIdentificationStrategy,
  estimableIdentification,
  estimandSentence,
  identifiedInstruments,
  type IdentificationArtifact,
  type StudySpecification,
} from '@/domain/study'
import { sunAbrahamComparison } from '@/domain/sunAbraham'
import type { SelectedSource } from '@/domain/workflow'
import { formatTime } from '@/lib/format/date'
import { formatCount, formatWords } from '@/lib/format/number'
import { lowerFirst } from '@/lib/text'
import { useRunActivity } from '@/lib/useRunActivity'
import { cn } from '@/lib/utils'
import { useEffect, useMemo, useState, type ReactNode } from 'react'
import { ArdlControls } from './ArdlControls'
import { BayesianGaussianControls } from './BayesianGaussianControls'
import { BootstrapIntervalFields } from './BootstrapIntervalFields'
import {
  CausalForestAnalysisControls,
  CausalForestSamplingControls,
} from './CausalForestAnalysisControls'
import { CausalForestControls } from './CausalForestControls'
import { CausalImpactControls } from './CausalImpactControls'
import type { DidCovariateCheck } from './DidCovariateChecklist'
import { ResultCard } from './EstimationResultCard'
import { FrontdoorControls } from './FrontdoorControls'
import { IngarchControls } from './IngarchControls'
import { LinearRegressionControls } from './LinearRegressionControls'
import { NegbinNutsControls } from './NegbinNutsControls'
import { OutcomeHistorySyntheticControls } from './OutcomeHistorySyntheticControls'
import { PanelInterventionControls } from './PanelInterventionControls'
import { PropensityControls } from './PropensityControls'
import {
  CLOSED_ADJUSTMENT_DRAFT,
  TemporalEffectControls,
  type ExplicitAdjustmentMemberDraft,
} from './TemporalEffectControls'
import { TLearnerControls } from './TLearnerControls'
import { useDidCovariates } from './useDidCovariates'
import { useSunAbrahamPanel } from './useSunAbrahamPanel'
import { VecmControls } from './VecmControls'

const ESTIMATOR_GROUP_LABELS: Readonly<Record<EstimatorGroupId, string>> = {
  'adjusted-outcome': 'Adjustment',
  'propensity-score': 'Propensity score',
  'identified-functional': 'Identified',
  'graph-adjusted-temporal': 'Temporal graph',
  'dynamic-time-series': 'Count intervention',
  'intervention-comparison': 'Interventions',
}

const eligibilityHint = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible':
      return 'Available: pre-run checks completed.'
    case 'caution':
      return 'Review: runnable, with conditions to assess.'
    case 'refused':
      return `Unavailable: ${eligibility.violations[0]?.evidence ?? 'a known requirement is not met.'}`
    default:
      return assertNever(eligibility)
  }
}

/** The run button's label. It shares a row with the running orb and the cancel button, so on a phone it stays short; results keep the full name. */
const runLabel = (estimator: EstimatorId): string => {
  switch (estimator) {
    case 'panel-intervention':
      return 'Run panel DiD'
    default:
      return `Run ${lowerFirst(describeEstimator(estimator))}`
  }
}

export function EstimationPanel({
  source,
  profile,
  prepared,
  stationarity,
  documents,
  studies,
  identifications,
  runs,
  sensitivityRuns,
  onRun,
  onDeleteRun,
  onOpenStudy,
  onActivity,
  selector,
  onOpenBacon,
}: {
  readonly selector?: ReactNode
  readonly onOpenBacon?: (outcome: ColumnId, treatment: ColumnId) => void
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly runs: readonly EstimationRunArtifact[]
  /** The recorded probes, so deleting a run can say which of them go with it. */
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly onRun: (run: EstimationRunArtifact) => void
  readonly onDeleteRun: (run: EstimationRunArtifact['id']) => void
  readonly onOpenStudy: () => void
}) {
  const columnName = (id: string): string =>
    profile.columns.find((column) => column.id === id)?.name ?? id
  const identified = identifications.filter((identification) =>
    estimableIdentification(identification.result),
  )
  const chartTheme = useChartTheme()
  const [pendingDelete, setPendingDelete] = useState<EstimationRunArtifact | null>(null)
  const [adjustmentDraft, setAdjustmentDraft] =
    useState<ExplicitAdjustmentMemberDraft>(CLOSED_ADJUSTMENT_DRAFT)
  const empty = useMemo(
    () => initialEstimationDraft(prepared, studies, [], []),
    [prepared, studies],
  )
  const state = useWorkflow((store) =>
    store.estimationDraft?.prepared === prepared.id ? store.estimationDraft.draft : empty,
  )
  const change = useWorkflow((store) => store.changeEstimation)
  const dispatch = (event: EstimationEvent) => change(prepared.id, event)
  const visibleEstimatorGroup = state.group
  const setVisibleEstimatorGroup = (group: EstimatorGroupId) =>
    dispatch({ type: 'group-chosen', group })
  const visibleGroup =
    ESTIMATOR_GROUPS.find((group) => group.id === visibleEstimatorGroup) ?? ESTIMATOR_GROUPS[0]
  const selectedEstimatorIsVisible = visibleGroup.estimators.includes(state.estimator)
  const identification =
    identified.find((candidate) => candidate.id === state.identification) ?? null
  const study =
    identification === null
      ? null
      : (studies.find((candidate) => candidate.id === identification.study) ?? null)
  const studyScale =
    prepared.kind === 'prepared-time-series' && study !== null
      ? [study.treatment, study.outcome]
          .map(
            (variable) =>
              `${variable.name}: ${describeSeriesTransform(seriesTransformFor(prepared.seriesTransforms, variable.column))}`,
          )
          .join('; ')
      : null
  const document =
    study === null
      ? null
      : (documents.find((candidate) => candidate.id === study.dagDocument) ?? null)
  const configuration = state.configurations[state.estimator]
  const method = methodDefinition(methodIdOf(state.estimator))
  const stepLabel =
    prepared.kind === 'prepared-time-series'
      ? frequencyUnit(prepared.sampling.frequency)
      : prepared.kind === 'prepared-panel'
        ? 'panel row'
        : 'row'
  const panelBinding = useMemo<PanelBinding | null>(
    () =>
      prepared.kind === 'prepared-panel' && study !== null
        ? {
            prepared: prepared.id,
            unit: prepared.sampling.unitColumn,
            time: prepared.sampling.timeColumn,
            outcome: study.outcome.column,
            treatment: study.treatment.column,
          }
        : null,
    [prepared, study],
  )
  const sunPanel = useSunAbrahamPanel(
    state.panelPreflight,
    configuration.kind === 'panel-intervention' && configuration.primary === 'sunAbraham'
      ? panelBinding
      : null,
  )
  const sunComparison = useMemo(
    () =>
      configuration.kind === 'panel-intervention' &&
      configuration.primary === 'sunAbraham' &&
      sunPanel.kind === 'ready'
        ? sunAbrahamComparison(sunPanel.catalog, configuration)
        : null,
    [configuration, sunPanel],
  )
  const sunBlocked =
    configuration.kind === 'panel-intervention' &&
    configuration.primary === 'sunAbraham' &&
    (sunPanel.kind !== 'ready' || sunComparison === null || !sunComparison.ok)
  const studyDataBinding = useMemo<StudyDataBinding | null>(
    () =>
      study === null
        ? null
        : {
            prepared: prepared.id,
            dagRevision: study.dagRevision,
            treatment: study.treatment.column,
            outcome: study.outcome.column,
          },
    [prepared.id, study],
  )
  const studyDataFacts = useMemo(() => {
    if (studyDataBinding === null)
      return { treatmentIsBinary: null, outcomeIsCount: null, observedGraphIsBinary: null } as const
    const job = state.studyDataPreflight
    if (job.kind !== 'ready' || !sameStudyDataBinding(job.binding, studyDataBinding))
      return { treatmentIsBinary: null, outcomeIsCount: null, observedGraphIsBinary: null } as const
    return {
      treatmentIsBinary: job.treatmentIsBinary,
      outcomeIsCount: job.outcomeIsCount,
      observedGraphIsBinary: job.observedGraphIsBinary,
    } as const
  }, [state.studyDataPreflight, studyDataBinding])
  const studyDataPending =
    studyDataBinding !== null &&
    (state.studyDataPreflight.kind === 'not-required' ||
      !('binding' in state.studyDataPreflight) ||
      !sameStudyDataBinding(state.studyDataPreflight.binding, studyDataBinding) ||
      state.studyDataPreflight.kind === 'loading')
  const studyDataError =
    studyDataBinding !== null &&
    state.studyDataPreflight.kind === 'failed' &&
    sameStudyDataBinding(state.studyDataPreflight.binding, studyDataBinding)
      ? state.studyDataPreflight.detail
      : null
  const panelPreflight = useMemo<PanelInterventionPreflight>(() => {
    if (panelBinding === null) return { kind: 'not-applicable' }
    const job = state.panelPreflight
    if (job.kind === 'not-required' || !samePanelBinding(job.binding, panelBinding))
      return { kind: 'pending' }
    switch (job.kind) {
      case 'loading':
        return { kind: 'pending' }
      case 'ready':
        return { kind: 'ready', layout: job.layout }
      case 'layout-refused':
      case 'refused':
        return { kind: 'refused', problem: job.problem }
      default:
        return assertNever(job)
    }
  }, [panelBinding, state.panelPreflight])

  useEffect(() => {
    if (panelBinding === null || prepared.kind !== 'prepared-panel') {
      dispatch({ type: 'panel-preflight-not-required' })
      return
    }
    let cancelled = false
    dispatch({ type: 'panel-preflight-started', binding: panelBinding })
    void (async () => {
      const { materializePanelInWorker } = await import('@/data/client')
      const matrix = await materializePanelInWorker(source.file, profile, {
        unit: panelBinding.unit,
        time: panelBinding.time,
        outcome: panelBinding.outcome,
        treatment: panelBinding.treatment,
      })
      if (cancelled) return
      if (!matrix.ok) {
        dispatch({
          type: 'panel-preflight-refused',
          binding: panelBinding,
          problem: { kind: 'panel-data', problem: matrix.error },
        })
        return
      }
      const layout = assessPanelInterventionLayout(matrix.value)
      if (!layout.ok) {
        dispatch({
          type: 'panel-layout-refused',
          binding: panelBinding,
          matrix: matrix.value,
          problem: { kind: 'panel-layout', problem: layout.error },
        })
        return
      }
      dispatch({
        type: 'panel-preflight-succeeded',
        binding: panelBinding,
        matrix: matrix.value,
        layout: layout.value,
      })
    })()
    return () => {
      cancelled = true
    }
  }, [panelBinding, prepared.kind, profile, source.file])

  useEffect(() => {
    if (studyDataBinding === null || study === null) {
      dispatch({ type: 'study-data-preflight-not-required' })
      return
    }
    let cancelled = false
    dispatch({ type: 'study-data-preflight-started', binding: studyDataBinding })
    void (async () => {
      const { materialisePrepared, describePreparedMaterialisationProblem } =
        await import('@/data/prepared')
      const observedColumns = study.graph.nodes.flatMap((node) =>
        node.column === null ? [] : [node.column],
      ) as unknown as NonEmptyArray<ColumnId>
      const matrix = await materialisePrepared(source, profile, prepared, observedColumns)
      if (cancelled) return
      if (!matrix.ok) {
        dispatch({
          type: 'study-data-preflight-failed',
          binding: studyDataBinding,
          detail: describePreparedMaterialisationProblem(matrix.error),
        })
        return
      }
      const treatmentPosition = observedColumns.findIndex(
        (column) => column === study.treatment.column,
      )
      const outcomePosition = observedColumns.findIndex((column) => column === study.outcome.column)
      const treatment = matrix.value.values.subarray(
        treatmentPosition * matrix.value.rowCount,
        (treatmentPosition + 1) * matrix.value.rowCount,
      )
      const outcome = matrix.value.values.subarray(
        outcomePosition * matrix.value.rowCount,
        (outcomePosition + 1) * matrix.value.rowCount,
      )
      dispatch({
        type: 'study-data-preflight-succeeded',
        binding: studyDataBinding,
        treatmentIsBinary: treatment.every((value) => value === 0 || value === 1),
        outcomeIsCount: outcome.every((value) => Number.isInteger(value) && value >= 0),
        observedGraphIsBinary: matrix.value.values.every((value) => value === 0 || value === 1),
      })
    })()
    return () => {
      cancelled = true
    }
  }, [prepared, profile, source, study, studyDataBinding])

  const eligibilityByEstimator = useMemo<ReadonlyMap<EstimatorId, MethodEligibility>>(() => {
    const evaluations = new Map<EstimatorId, MethodEligibility>()
    if (identification === null) return evaluations
    for (const estimator of ESTIMATOR_GROUPS.flatMap((group) => group.estimators)) {
      const definition = methodDefinition(methodIdOf(estimator))
      if (!definition.ok) continue
      evaluations.set(
        estimator,
        evaluateEstimatorEligibility(definition.value, {
          identification: identification.result,
          prepared,
          stationarity,
          configuration: state.configurations[estimator],
          outcomeIsCount: studyDataFacts.outcomeIsCount,
          treatmentIsBinary: studyDataFacts.treatmentIsBinary,
          observedGraphIsBinary: studyDataFacts.observedGraphIsBinary,
          document,
          study,
          panelPreflight,
          sunAbrahamComparison:
            sunPanel.kind === 'failed'
              ? { ok: false, error: sunPanel.detail }
              : (sunComparison ?? undefined),
        }),
      )
    }
    return evaluations
  }, [
    document,
    identification,
    panelPreflight,
    prepared,
    state.configurations,
    stationarity,
    study,
    studyDataFacts,
    sunPanel,
    sunComparison,
  ])
  const eligibility = eligibilityByEstimator.get(state.estimator) ?? null
  const configure = (next: EstimatorConfiguration) => {
    session.clearNotice()
    dispatch({ type: 'configured', configuration: next })
  }
  const adjustmentDraftOpen =
    configuration.kind === 'causal-effects-total' &&
    configuration.estimator.kind !== 'wrightParents' &&
    configuration.estimator.adjustment.kind === 'explicit' &&
    adjustmentDraft.kind === 'editing'

  useEffect(() => {
    const explicitAdjustmentActive =
      configuration.kind === 'causal-effects-total' &&
      configuration.estimator.kind !== 'wrightParents' &&
      configuration.estimator.adjustment.kind === 'explicit'
    if (!explicitAdjustmentActive && adjustmentDraft.kind === 'editing') {
      setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
    }
  }, [adjustmentDraft.kind, configuration])

  const session = useJob('estimation')
  const { job } = session
  const [pointwiseRetry, setPointwiseRetry] = useState<PointwiseRetry | null>(null)
  useRunActivity(
    onActivity,
    job.kind === 'running'
      ? {
          label: describeEstimator(state.estimator),
          progress:
            job.progress === null ? null : job.progress.completed / Math.max(1, job.progress.total),
        }
      : null,
  )
  const runBlocked =
    job.kind === 'running' ||
    session.blocked ||
    sunBlocked ||
    !selectedEstimatorIsVisible ||
    identification === null ||
    study === null ||
    eligibility === null ||
    eligibility.kind === 'refused' ||
    studyDataPending ||
    studyDataError !== null ||
    adjustmentDraftOpen ||
    !method.ok ||
    (configuration.kind === 'panel-intervention' &&
      configuration.primary !== 'staggered' &&
      configuration.primary !== 'sunAbraham' &&
      panelPreflight.kind !== 'ready')
  const execute = async (
    requested: EstimatorConfiguration = state.configurations[state.estimator],
    signal?: AbortSignal,
  ): Promise<AssistantRunOutcome> => {
    const configuration =
      requested.kind === 'panel-intervention' && requested.primary === 'did'
        ? { ...requested, inference: requested.inference ?? defaultConventionalDidInference }
        : requested
    if (
      signal?.aborted ||
      runBlocked ||
      identification === null ||
      study === null ||
      eligibility === null ||
      adjustmentDraftOpen ||
      !method.ok
    )
      return {
        kind: 'refused',
        reason: 'The current specification is not ready to run. Review Estimation.',
      }
    if (!estimableIdentification(identification.result))
      return { kind: 'refused', reason: 'The study is not identified for estimation.' }
    const current = session.start('analysis', describeEstimator(state.estimator))
    if (current === null) return { kind: 'refused', reason: 'Another analysis is running.' }
    let outcome: AssistantRunOutcome = { kind: 'cancelled' }
    const cancel = () => {
      if (session.current(current)) session.cancel()
    }
    signal?.addEventListener('abort', cancel, { once: true })
    setPointwiseRetry(null)
    const dispatch = (event: EstimationRunEvent) => {
      switch (event.type) {
        case 'run-progressed':
          session.progress(current, describeEstimator(state.estimator), event.progress)
          return
        case 'run-failed':
          outcome = { kind: 'failed', reason: event.detail }
          session.fail(current, event.detail)
          return
        case 'run-finished':
          session.finish(current)
          return
        default:
          assertNever(event)
      }
    }
    try {
      const { executeEstimation } = await import('@/analysis/estimation/execute')
      if (!session.current(current)) return { kind: 'cancelled' }
      await executeEstimation(
        {
          configuration,
          source,
          profile,
          prepared,
          study,
          identification,
          eligibility,
          encodings: state.encodings,
          panelPreflight: state.panelPreflight,
          panelBinding,
          sunPanel,
          document,
        },
        {
          isCurrent: () => session.current(current),
          emit: dispatch,
          onRun: (run) => {
            if (!session.current(current) || signal?.aborted) return
            onRun(run)
            outcome = {
              kind: 'completed',
              runId: run.id,
              method: describeRun(run.configuration),
              subject: {
                kind: 'saved-study',
                study: run.study,
                treatment: study.treatment.name,
                outcome: study.outcome.name,
              },
              summary: estimateSummary(run.estimate, {
                kind: 'saved-study',
                study: run.study,
                treatment: study.treatment.name,
                outcome: study.outcome.name,
              }),
            }
          },
          offerPointwiseRetry: setPointwiseRetry,
        },
      )
    } catch (cause: unknown) {
      dispatch({
        type: 'run-failed',
        detail: cause instanceof Error ? cause.message : String(cause),
      })
    } finally {
      signal?.removeEventListener('abort', cancel)
    }
    return signal?.aborted ? { kind: 'cancelled' } : outcome
  }

  const latestRun = runs.at(-1) ?? null
  const workflowStore = useWorkflowStore()
  const selectStudy = (id: string) => {
    session.clearNotice()
    dispatch({
      type: 'identification-chosen',
      selection: estimationSelection(
        identified.find((candidate) => candidate.id === id) ?? null,
        studies,
        prepared,
      ),
    })
    return workflowStore.getState().estimationDraft?.draft.identification === id
  }
  useStudySelectionTool(
    JSON.stringify({
      prepared: prepared.id,
      selected: state.identification,
      studies,
      identified,
      configuration,
      encodings: state.encodings,
      revisions: documents.map((item) => item.current.id),
    }),
    session.blocked || job.kind === 'running'
      ? null
      : {
          selected: state.identification,
          choices: identified.flatMap((candidate) => {
            const bound = studies.find((item) => item.id === candidate.study)
            return bound
              ? [
                  {
                    identification: candidate.id,
                    study: bound.id,
                    question: describeEstimand(bound),
                    treatment: bound.treatment.name,
                    outcome: bound.outcome.name,
                    adjustment:
                      'adjustment' in candidate.result
                        ? candidate.result.adjustment.variables
                            .map((variable) => variable.name)
                            .join(', ') || 'None'
                        : 'Not a back-door adjustment specification',
                  },
                ]
              : []
          }),
        },
    selectStudy,
  )
  useAnalysisRunTool(
    'estimation',
    JSON.stringify({
      source: profile.id,
      prepared: prepared.id,
      study,
      revision: document?.current.id,
      identification,
      configuration,
      encodings: state.encodings,
      selected: state.estimator,
    }),
    study && identification
      ? {
          section: 'Estimation',
          method: describeEstimator(state.estimator),
          study: {
            subject: {
              kind: 'saved-study',
              study: study.id,
              treatment: study.treatment.name,
              outcome: study.outcome.name,
            },
            question: describeEstimand(study),
            settings: [
              {
                label: 'Identified adjustment set',
                value:
                  'adjustment' in identification.result
                    ? identification.result.adjustment.variables
                        .map((variable) => variable.name)
                        .join(', ') || 'None'
                    : 'Not a back-door adjustment specification',
              },
              ...configurationSettings(configuration),
            ],
          },
          specification: JSON.stringify({
            study: study.id,
            treatment: study.treatment.name,
            outcome: study.outcome.name,
            target: study.estimand,
            configuration,
            identification: identification.result,
            encodings: state.encodings,
          }),
          sharedResult:
            'Run ID, method, target, headline effect, interval, standard error, sample counts and adjustment variables. No row-level predictions.',
        }
      : null,
    (signal) => execute(configuration, signal),
  )
  const temporalAdjustmentCandidates =
    document === null || study === null
      ? []
      : document.current.graph.nodes.flatMap((node, index) =>
          node.kind === 'observed' ? [{ index, name: node.name }] : [],
        )
  const temporalGraphLag = document === null ? 0 : stationaryMarksOf(document).statLag
  const temporalAdjustmentMaxLag =
    configuration.kind === 'causal-effects-total'
      ? temporalGraphLag + configuration.treatmentLag
      : 0
  useEstimationTools(
    JSON.stringify({
      prepared: prepared.id,
      study: study?.id,
      revision: document?.current.id,
      identification: identification?.id,
      configuration: state.configurations,
      encodings: state.encodings,
    }),
    study !== null &&
      identification !== null &&
      !session.blocked &&
      job.kind !== 'running' &&
      !adjustmentDraftOpen
      ? {
          study: {
            treatment: study.treatment.name,
            outcome: study.outcome.name,
            target: study.estimand.kind,
          },
          selected: state.estimator,
          ...(canConfigureEstimator(configuration)
            ? {
                configuration: {
                  current: configuration,
                  observations: prepared.observations,
                  ...(configuration.kind === 'causal-effects-total'
                    ? {
                        temporal: {
                          candidates: temporalAdjustmentCandidates,
                          graphLag: temporalGraphLag,
                        },
                      }
                    : {}),
                  columns: profile.columns
                    .filter((column) => prepared.columns.includes(column.id))
                    .map(({ id, name }) => ({ id, name })),
                },
              }
            : {}),
          choices: [...eligibilityByEstimator].map(([id, eligibility]) => ({
            id,
            name: describeEstimator(id),
            eligibility,
          })),
        }
      : null,
    (estimator) => dispatch({ type: 'estimator-chosen', estimator }),
    canConfigureEstimator(configuration) && identification !== null && method.ok
      ? {
          configure,
          evaluate: (next) =>
            evaluateEstimatorEligibility(method.value, {
              identification: identification.result,
              prepared,
              stationarity,
              configuration: next,
              outcomeIsCount: studyDataFacts.outcomeIsCount,
              treatmentIsBinary: studyDataFacts.treatmentIsBinary,
              observedGraphIsBinary: studyDataFacts.observedGraphIsBinary,
              document,
              study,
              panelPreflight,
            }),
        }
      : undefined,
  )
  const controlCandidates = useMemo(
    () =>
      study === null
        ? []
        : profile.columns.filter(
            (column) =>
              prepared.columns.includes(column.id) &&
              column.id !== study.outcome.column &&
              column.id !== study.treatment.column,
          ),
    [study, profile, prepared],
  )
  const didCandidates = useMemo(
    () =>
      controlCandidates.filter(
        (column) =>
          prepared.kind !== 'prepared-panel' ||
          (column.id !== prepared.panel.unitColumn && column.id !== prepared.panel.timeColumn),
      ),
    [controlCandidates, prepared],
  )
  const didCovariateMethod =
    configuration.kind === 'panel-intervention' &&
    (configuration.primary === 'staggered' || configuration.primary === 'adjusted')
      ? configuration
      : null
  const didInspection = useDidCovariates(
    didCovariateMethod !== null && prepared.kind === 'prepared-panel'
      ? { file: source.file, profile, binding: panelBinding }
      : null,
    didCandidates,
  )
  const didRestrictions = useMemo<ReadonlyMap<ColumnId, string>>(() => {
    if (didInspection.kind !== 'ready' || didCovariateMethod === null) return new Map()
    const restricted =
      didCovariateMethod.primary === 'staggered'
        ? staggeredCovariateRestrictions(didInspection.matrix)
        : didCovariateMethod.primary === 'adjusted' && didInspection.layout !== null
          ? didCovariateRestrictions(
              didInspection.matrix,
              didInspection.layout,
              didCovariateMethod.specification,
            )
          : []
    return new Map(restricted.map((item) => [item.column, describeDidCovariateRole(item.role)]))
  }, [didInspection, didCovariateMethod])
  const didCheck: DidCovariateCheck =
    didInspection.kind === 'pending'
      ? 'pending'
      : didInspection.kind === 'ready' &&
          (didCovariateMethod?.primary !== 'adjusted' || didInspection.layout !== null)
        ? 'checked'
        : 'unchecked'
  // A unit or cluster column identifies groups of rows; it cannot also be a regressor. A panel's own unit key comes first.
  const designColumns = new Set(
    identification !== null && 'adjustment' in identification.result
      ? identification.result.adjustment.variables.map((variable) => variable.column)
      : [],
  )
  const panelUnit =
    prepared.kind === 'prepared-panel'
      ? (profile.columns.find((column) => column.id === prepared.sampling.unitColumn) ?? null)
      : null
  const groupingCandidates = [
    ...(panelUnit === null ? [] : [panelUnit]),
    ...controlCandidates.filter(
      (column) => !designColumns.has(column.id) && column.id !== panelUnit?.id,
    ),
  ]
  const panelTime =
    prepared.kind === 'prepared-panel'
      ? (profile.columns.find((column) => column.id === prepared.sampling.timeColumn) ?? null)
      : null
  const timeCandidates = [
    ...(panelTime === null ? [] : [panelTime]),
    ...groupingCandidates.filter((column) => column.id !== panelTime?.id),
  ]

  const encodingVariables =
    identification?.result.kind !== 'identified'
      ? []
      : state.estimator === 'causal-forest'
        ? causalForestInputs(identification.result.adjustment.variables, study?.estimand ?? null)
        : state.estimator === 't-learner'
          ? tLearnerInputs(identification.result.adjustment.variables, study?.estimand ?? null)
          : identification.result.adjustment.variables
  const configureEncodings = (selected: readonly string[]) => {
    for (const variable of encodingVariables) {
      const kind = selected.includes(variable.column) ? 'categorical' : 'numeric'
      if ((state.encodings[variable.column]?.kind ?? 'numeric') !== kind)
        dispatch({ type: 'encoding-declared', column: variable.column, encoding: { kind } })
    }
  }
  useEncodingSettings({
    binding: JSON.stringify({
      prepared: prepared.id,
      study: study?.id,
      identification,
      estimator: state.estimator,
    }),
    available:
      selectedEstimatorIsVisible && expandsDesign(state.estimator) && encodingVariables.length > 0,
    blocked: session.job.kind === 'running' || session.blocked,
    columns: encodingVariables.map((variable) => ({ id: variable.column, name: variable.name })),
    encodings: state.encodings,
    configure: configureEncodings,
  })
  const categoricalChecklist =
    selectedEstimatorIsVisible && expandsDesign(state.estimator) && encodingVariables.length > 0 ? (
      <ColumnChecklist
        title="Categorical covariates"
        help="Tick the covariates whose values are categories rather than quantities. A ticked covariate becomes one column per level instead of one numeric column."
        columns={encodingVariables.map((variable) => ({
          id: variable.column,
          name: variable.name,
        }))}
        selected={encodingVariables
          .filter((variable) => state.encodings[variable.column]?.kind === 'categorical')
          .map((variable) => variable.column)}
        onChange={configureEncodings}
      />
    ) : null

  const covariatesStep = (number: number) =>
    categoricalChecklist === null ? null : (
      <SettingsStep number={number} title="Covariates">
        {categoricalChecklist}
      </SettingsStep>
    )

  const controls = ((): React.ReactNode => {
    switch (configuration.kind) {
      case 'sharp-rd':
        return (
          <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
            {study?.estimand.kind === 'local-cutoff-effect'
              ? `Treatment is 1 where ${study.estimand.running.name} is at least ${study.estimand.cutoff}, and 0 below. `
              : 'Choose a cutoff-local study. '}
            Local-linear fits use a triangular kernel, automatic mserd bandwidth and
            nearest-neighbor variance with three neighbors. The headline uses robust bias-corrected
            inference.
          </p>
        )
      case 'frontdoor-two-stage':
        return <FrontdoorControls configuration={configuration} configure={configure} />

      case 'instrumental-variable':
        return (
          <SettingsStep title="Report uncertainty">
            <div className={fieldRow.two}>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Bootstrap resamples"
                  help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapResamples}
                />
                <NumberInput
                  min={20}
                  max={5000}
                  aria-label="Instrumental-variable bootstrap resamples"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.simulations}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      simulations: Math.max(
                        20,
                        Math.min(5000, Math.floor(Number(event.target.value) || 20)),
                      ),
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Bootstrap seed"
                  help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapSeed}
                />
                <NumberInput
                  min={0}
                  aria-label="Instrumental-variable bootstrap seed"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.seed}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      seed: Math.max(0, Math.floor(Number(event.target.value) || 0)),
                    })
                  }
                />
              </label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
              The estimate is the ratio of the instrument’s effect on the outcome to its effect on
              the treatment: the Wald estimator for one binary instrument, a covariance ratio for
              one continuous instrument, and two-stage least squares otherwise. The effect is
              reported for the treatment set to 1 rather than 0; the interval uses a seeded row
              bootstrap.
            </p>
          </SettingsStep>
        )
      case 'backdoor-linear-regression':
        return (
          <LinearRegressionControls
            covariatesStep={covariatesStep}
            categoricalChecklist={categoricalChecklist}
            configuration={configuration}
            panelTime={panelTime}
            timeCandidates={timeCandidates}
            groupingCandidates={groupingCandidates}
            configure={configure}
            panelUnit={panelUnit}
          />
        )

      case 'dml-plr':
      case 'dml-irm':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep
              number={categoricalChecklist === null ? undefined : 2}
              title="Cross-fitting"
            >
              {configuration.kind === 'dml-irm' && (
                <div>
                  <span className={fieldLabel}>Study target</span>
                  <p className="mb-0 mt-1 text-body text-ink">
                    {configuration.att
                      ? 'Effect on the treated (ATT)'
                      : 'Average treatment effect (ATE)'}
                  </p>
                  <p className={cn(fieldHint, 'mt-1')}>
                    Change the target in Study design, not in the estimator.
                  </p>
                </div>
              )}
              <label className="block max-w-xs">
                <ParameterLabel
                  className={fieldLabel}
                  label="Fold seed"
                  help={ESTIMATION_PARAMETER_HELP.dml.foldSeed}
                />
                <NumberInput
                  min={0}
                  aria-label="Fold seed"
                  className={field('text', 'mt-1')}
                  value={configuration.seed}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      seed: Math.max(0, Math.floor(Number(event.target.value) || 0)),
                    })
                  }
                />
              </label>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
                Five shuffled folds, 200 random-forest trees, minimum leaf 5, learner seed 7. The
                Sensitivity section repeats this fit at the same seed before its refuters.
              </p>
            </SettingsStep>
          </div>
        )
      case 'propensity-weighting':
      case 'propensity-matching':
      case 'doubly-robust':
        return (
          <PropensityControls
            configuration={configuration}
            configure={configure}
            categoricalChecklist={categoricalChecklist}
          />
        )

      case 'continuous-gps': {
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? 1 : 2} title="Weight the sample">
              <div>
                <ParameterLabel
                  className={fieldLabel}
                  label="Weights"
                  help={ESTIMATION_PARAMETER_HELP.propensity.gpsScale}
                />
                <SegmentedControl
                  className="mt-1 max-w-md"
                  fill
                  ariaLabel="Weights"
                  value={configuration.scale}
                  options={[
                    { value: 'inverseDensity', label: 'Inverse density' },
                    { value: 'stabilized', label: 'Stabilized' },
                  ]}
                  onChange={(scale) =>
                    configure({
                      ...configuration,
                      scale: scale === 'stabilized' ? 'stabilized' : 'inverseDensity',
                    })
                  }
                />
              </div>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
                The treatment is regressed on the adjustment set and taken as normal around its
                fitted value with constant variance. The weighted model fits one slope, so the
                result is a single treatment response rather than a curve.
              </p>
            </SettingsStep>
            <SettingsStep number={categoricalChecklist === null ? 2 : 3} title="Report uncertainty">
              <BootstrapIntervalFields
                value={configuration.uncertainty}
                help={ESTIMATION_PARAMETER_HELP.propensity.uncertainty}
                onChange={(uncertainty) => configure({ ...configuration, uncertainty })}
              />
            </SettingsStep>
          </div>
        )
      }
      case 'causal-forest': {
        const analyse = (analysis: ForestAnalysis) =>
          configure({
            ...configuration,
            analysis: {
              ...analysis,
              labels: forestAnalysisColumns(analysis).flatMap((id) => {
                const column = profile.columns.find((c) => c.id === id)
                return column === undefined ? [] : [{ column: column.id, name: column.name }]
              }),
            },
          })
        const numbered = (number: number) => (categoricalChecklist === null ? undefined : number)
        return (
          <div className="grid grid-cols-1 gap-8 @6xl/panel:grid-cols-2 @6xl/panel:items-start @6xl/panel:gap-x-16">
            {covariatesStep(1)}
            <SettingsStep
              number={numbered(2)}
              title="Fit the causal forest"
              className="@6xl/panel:row-span-2"
            >
              <CausalForestControls configuration={configuration} onChange={configure} />
              <CausalForestSamplingControls
                value={configuration.analysis}
                columns={controlCandidates}
                onChange={analyse}
              />
            </SettingsStep>
            <SettingsStep number={numbered(3)} title="Summarize effect heterogeneity">
              <CausalForestAnalysisControls
                value={configuration.analysis}
                columns={controlCandidates}
                onChange={analyse}
              />
            </SettingsStep>
          </div>
        )
      }
      case 't-learner':
        return (
          <TLearnerControls
            configuration={configuration}
            covariatesStep={covariatesStep}
            categoricalChecklist={categoricalChecklist}
            configure={configure}
          />
        )

      case 'ardl-pss':
        return <ArdlControls configuration={configuration} configure={configure} />

      case 'vecm':
        return (
          <VecmControls configuration={configuration} configure={configure} prepared={prepared} />
        )

      case 'synthetic-control':
        return (
          <OutcomeHistorySyntheticControls
            prepared={prepared}
            configuration={configuration}
            configure={configure}
            panelPreflight={panelPreflight}
            source={source}
            profile={profile}
            study={study}
            controlCandidates={controlCandidates}
          />
        )

      case 'panel-intervention':
        return (
          <PanelInterventionControls
            configuration={configuration}
            configure={configure}
            prepared={prepared}
            study={study}
            onOpenBacon={onOpenBacon}
            job={job}
            session={session}
            sunPanel={sunPanel}
            didCandidates={didCandidates}
            didRestrictions={didRestrictions}
            didCheck={didCheck}
            profile={profile}
            panelPreflight={panelPreflight}
          />
        )

      case 'negbin-nuts':
        return <NegbinNutsControls configuration={configuration} configure={configure} />

      case 'bayesian-gaussian':
        return (
          <BayesianGaussianControls
            covariatesStep={covariatesStep}
            categoricalChecklist={categoricalChecklist}
            configuration={configuration}
            configure={configure}
          />
        )

      case 'discrete-bn-query':
        return (
          <SettingsStep title="Discretise and smooth">
            <div className={fieldRow.two}>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="State budget"
                  help={ESTIMATION_PARAMETER_HELP.discreteBn.stateBudget}
                />
                <NumberInput
                  min={2}
                  max={10}
                  aria-label="State budget"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.bins}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      bins: Math.max(2, Math.min(10, Math.floor(Number(event.target.value) || 2))),
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Equivalent sample size"
                  help={ESTIMATION_PARAMETER_HELP.discreteBn.equivalentSampleSize}
                />
                <NumberInput
                  min={0.1}
                  step="any"
                  aria-label="Equivalent sample size"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.equivalentSampleSize}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      equivalentSampleSize: Math.max(0.1, Number(event.target.value) || 0.1),
                    })
                  }
                />
              </label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
              Observed binary and ordinal states are preserved when they fit the budget;
              higher-cardinality values are divided at quantiles. The BDeu prior smooths the
              conditional tables, and the effect contrasts the lowest and highest treatment states.
            </p>
          </SettingsStep>
        )
      case 'binary-ett-idc-star':
        return (
          <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
            The run evaluates the two recorded IDC* expressions against the empirical binary joint
            distribution. It applies no discretisation and reports no sampling interval.
          </p>
        )
      case 'poisson-glm':
      case 'negative-binomial-p':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
              Log link on the expected count of {study?.outcome.name ?? 'the outcome'};
              exponentiating the treatment coefficient gives an expected-count ratio with a 95%
              normal interval.
            </p>
          </div>
        )
      case 'negative-binomial-ingarch':
        return <IngarchControls configuration={configuration} configure={configure} />

      case 'causal-effects-total':
        return (
          <TemporalEffectControls
            configuration={configuration}
            configure={configure}
            setAdjustmentDraft={setAdjustmentDraft}
            temporalAdjustmentCandidates={temporalAdjustmentCandidates}
            adjustmentDraft={adjustmentDraft}
            temporalAdjustmentMaxLag={temporalAdjustmentMaxLag}
          />
        )

      case 'causal-impact':
        return (
          <CausalImpactControls
            configuration={configuration}
            configure={configure}
            prepared={prepared}
            study={study}
            controlCandidates={controlCandidates}
          />
        )

      default:
        return assertNever(configuration)
    }
  })()

  const stage = (
    <section aria-labelledby="estimation-title" className="@container/panel flex flex-col gap-5">
      <div>
        <div className="mb-2 flex items-center gap-1.5">
          <ChapterHeading id="estimation-title" className="mb-0">
            Estimation
          </ChapterHeading>
          <ParameterHelp
            label="Estimation"
            help="Choose an estimator for the identified effect and run the analysis."
          />
        </div>
        <p className={chapterIntro}>
          Identification determines how to express the causal question using observed data.
          Estimation applies a statistical method to that expression. In this section, you choose a
          compatible estimator and examine the effect estimate, its uncertainty, and the
          method-specific diagnostics.
        </p>
      </div>

      {/* The selected method is named in the requirements panel, not above the choice that selects it. */}
      <section className={panel('p-(--panel-space)')} aria-label="Estimator setup">
        {selector !== undefined && <div className="mb-6">{selector}</div>}
        {identified.length === 0 ? (
          <Alert tone="info" live={false}>
            <p className="m-0">
              <button type="button" className="underline" onClick={onOpenStudy}>
                Open Study design
              </button>{' '}
              and identify a study first.
            </p>
          </Alert>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-4">
              <label className="block">
                <span className={fieldLabel}>Identified study</span>
                <Select
                  className={field('text', 'mt-1')}
                  value={state.identification ?? ''}
                  onChange={(event) => {
                    selectStudy(event.target.value)
                  }}
                >
                  {identified.map((candidate) => {
                    const bound = studies.find((item) => item.id === candidate.study)
                    return (
                      <option key={candidate.id} value={candidate.id}>
                        {bound === undefined
                          ? candidate.id
                          : `${estimandSentence(bound)}, ${bound.dagName}`}
                      </option>
                    )
                  })}
                </Select>
              </label>
              <div>
                <SegmentedControl
                  ariaLabel="Estimator family"
                  value={visibleEstimatorGroup}
                  onChange={setVisibleEstimatorGroup}
                  options={ESTIMATOR_GROUPS.map((group) => ({
                    value: group.id,
                    label: ESTIMATOR_GROUP_LABELS[group.id],
                    title: group.name,
                  }))}
                />
                <div className="mt-4">
                  <RadioList
                    frame="none"
                    columns={2}
                    legend={visibleGroup.name}
                    legendHidden
                    value={state.estimator}
                    onChange={(estimator) => {
                      session.clearNotice()
                      dispatch({ type: 'estimator-chosen', estimator })
                    }}
                    options={visibleGroup.estimators.flatMap((id) => {
                      const definition = methodDefinition(methodIdOf(id))
                      const candidateEligibility = eligibilityByEstimator.get(id)
                      return definition.ok && candidateEligibility !== undefined
                        ? [
                            {
                              value: id,
                              label: definition.value.name,
                              hint:
                                id === 'panel-intervention' &&
                                candidateEligibility.kind === 'refused'
                                  ? 'Review the selected panel method and its requirements below.'
                                  : eligibilityHint(candidateEligibility),
                              disabled:
                                candidateEligibility.kind === 'refused' &&
                                id !== 'panel-intervention' &&
                                !(id === 'synthetic-control' && prepared.kind === 'prepared-panel'),
                              title:
                                candidateEligibility.kind === 'refused'
                                  ? `${definition.value.name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}`
                                  : undefined,
                            },
                          ]
                        : []
                    })}
                  />
                </div>
                {!selectedEstimatorIsVisible && (
                  <p className={cn(fieldHint, 'mt-3')}>
                    Choose a method from this family to configure it.
                  </p>
                )}
              </div>
              {selectedEstimatorIsVisible && <div className="mt-8">{controls}</div>}
            </div>
            {/* One shrinkable column, so a long run row can never widen the notices above it. */}
            <div className={cn(actionGap, 'grid grid-cols-[minmax(0,1fr)] gap-3')}>
              {selectedEstimatorIsVisible && eligibility !== null && (
                <EligibilityView eligibility={eligibility} subject="this study" />
              )}
              {studyDataError !== null && (
                <Alert tone="danger">
                  <p className="m-0">
                    The treatment and outcome columns could not be checked: {studyDataError}
                  </p>
                </Alert>
              )}
              <JobNotice job={job} />
              {job.kind === 'failed' &&
                pointwiseRetry !== null &&
                pointwiseRetry.configuration === configuration &&
                pointwiseRetry.study === study?.id &&
                pointwiseRetry.prepared === prepared.id &&
                configuration.kind === 'panel-intervention' &&
                configuration.primary === 'staggered' &&
                configuration.specification.inference.kind === 'bootstrapSimultaneous' && (
                  <button
                    type="button"
                    className={button('quiet')}
                    disabled={
                      session.blocked ||
                      eligibility === null ||
                      eligibility.kind === 'refused' ||
                      studyDataPending ||
                      studyDataError !== null
                    }
                    onClick={() => {
                      if (configuration.specification.inference.kind !== 'bootstrapSimultaneous')
                        return
                      const next = {
                        ...configuration,
                        specification: {
                          ...configuration.specification,
                          inference: {
                            ...configuration.specification.inference,
                            kind: 'bootstrapPointwise' as const,
                          },
                        },
                      }
                      configure(next)
                      void execute(next)
                    }}
                  >
                    Run with pointwise bootstrap
                  </button>
                )}
              <RunActions
                className="min-h-10"
                testId="estimation-run-row"
                running={job.kind === 'running'}
                onCancel={session.cancel}
                orbLabel="Estimator running"
              >
                <button
                  type="button"
                  className={button('signal', 'min-w-0')}
                  disabled={runBlocked}
                  aria-busy={job.kind === 'running'}
                  onClick={() => void execute()}
                >
                  {!selectedEstimatorIsVisible
                    ? 'Choose a method'
                    : studyDataPending
                      ? 'Checking treatment and outcome…'
                      : configuration.kind === 'panel-intervention' &&
                          configuration.primary !== 'staggered' &&
                          configuration.primary !== 'sunAbraham' &&
                          panelPreflight.kind === 'pending'
                        ? 'Checking panel…'
                        : runLabel(state.estimator)}
                </button>
              </RunActions>
            </div>
          </>
        )}
      </section>

      {latestRun !== null && (
        <section aria-labelledby="estimation-results-title" className="grid grid-cols-1 gap-4">
          <div>
            <h2 id="estimation-results-title" className={cn(sectionTitle, 'm-0')}>
              Estimates
            </h2>
          </div>
          {(() => {
            const bound = studies.find((candidate) => candidate.id === latestRun.study)
            return bound === undefined ? null : (
              <ResultCard
                run={latestRun}
                study={bound}
                current
                stepLabel={stepLabel}
                others={runs}
              />
            )
          })()}
        </section>
      )}
    </section>
  )

  const [inspectorView, setInspectorView] = useState<'study' | 'method'>('method')
  const inspector = (
    <div className="flex flex-col gap-4">
      <h3 id="estimation-study-title" className="m-0 mt-1 text-body font-medium text-ink">
        {study === null ? 'No study chosen' : estimandSentence(study)}
      </h3>
      <SegmentedControl
        size="sm"
        fill
        ariaLabel="Estimation inspector"
        value={inspectorView}
        onChange={setInspectorView}
        options={[
          {
            value: 'study',
            label: (
              <span className="flex items-center gap-1.5">
                <Icon name="science" size={14} />
                Study
              </span>
            ),
          },
          {
            value: 'method',
            label: (
              <span className="flex items-center gap-1.5">
                <Icon name="function" size={14} />
                Method
              </span>
            ),
          },
        ]}
      />
      {inspectorView === 'study' && (
        <section aria-labelledby="estimation-study-title" className="flex flex-col gap-4">
          {study !== null &&
            identification !== null &&
            estimableIdentification(identification.result) && (
              <>
                <dl
                  className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body"
                  aria-label="Study binding"
                >
                  <dt className="text-faint">Treatment</dt>
                  <dd className="m-0 text-ink">{study.treatment.name}</dd>
                  <dt className="text-faint">Outcome</dt>
                  <dd className="m-0 text-ink">{study.outcome.name}</dd>
                  <dt className="text-faint">Graph</dt>
                  <dd className="m-0 text-ink">
                    <Metadata>
                      <span>{study.dagName}</span>
                      <span>
                        <span className={literal()}>{study.dagRevision.slice(0, 8)}</span>
                      </span>
                    </Metadata>
                  </dd>
                  <dt className="text-faint">Strategy</dt>
                  <dd className="m-0 text-ink">
                    {describeIdentificationStrategy(identification.result)}
                  </dd>
                  {identification.result.kind === 'identified' && (
                    <>
                      <dt className="text-faint">Adjustment set</dt>
                      <dd className="m-0 text-ink">
                        {identification.result.adjustment.variables.length === 0
                          ? 'none'
                          : identification.result.adjustment.variables
                              .map((variable) => variable.name)
                              .join(', ')}
                      </dd>
                    </>
                  )}
                  {identification.result.kind === 'graphically-identified' &&
                    identification.result.frontdoor.kind === 'identified' && (
                      <>
                        <dt className="text-faint">Front-door mediator</dt>
                        <dd className="m-0 text-ink">
                          {identification.result.frontdoor.mediators
                            .map((variable) => variable.name)
                            .join(', ')}
                        </dd>
                      </>
                    )}
                  {identifiedInstruments(identification.result) !== null && (
                    <>
                      <dt className="text-faint">Instruments</dt>
                      <dd className="m-0 text-ink">
                        {(identifiedInstruments(identification.result) ?? [])
                          .map((variable) => variable.name)
                          .join(', ')}
                      </dd>
                    </>
                  )}
                  <dt className="text-faint">Rows</dt>
                  <dd className={num('m-0 text-ink')}>
                    <Metadata>
                      <span>
                        {prepared.kind === 'prepared-time-series'
                          ? 'Time series'
                          : prepared.kind === 'prepared-panel'
                            ? 'Panel'
                            : 'Cross-section'}
                      </span>
                      <span>{formatCount(prepared.observations).text}</span>
                    </Metadata>
                  </dd>
                  {studyScale !== null && (
                    <>
                      <dt className="text-faint">Analysis scale</dt>
                      <dd className="m-0 text-ink">{studyScale}</dd>
                    </>
                  )}
                </dl>
              </>
            )}
          {identification !== null && (
            <IdentificationRecord identification={identification.result} />
          )}
        </section>
      )}
      {inspectorView === 'method' && (
        <section aria-label="Method" className="flex flex-col gap-4">
          <div>
            <h4 className="m-0 text-body font-medium text-ink">{visibleGroup.name}</h4>
            <p className="mb-0 mt-1 text-body text-muted">{visibleGroup.description}</p>
          </div>
          {configuration.kind === 'panel-intervention' &&
            configuration.primary !== 'staggered' &&
            configuration.primary !== 'sunAbraham' &&
            panelPreflight.kind === 'pending' && (
              <p className="m-0 text-body text-muted" role="status">
                Checking panel structure and treatment timing…
              </p>
            )}
          {configuration.kind === 'panel-intervention' &&
            configuration.primary !== 'staggered' &&
            configuration.primary !== 'sunAbraham' &&
            panelPreflight.kind === 'ready' && (
              <section aria-label="Checked panel structure" className="space-y-2">
                <p className="m-0 flex items-center gap-1.5 text-body text-ok">
                  <Icon name="check_circle" size={16} />
                  Panel structure checked
                </p>
                <MetricGrid label="Panel structure">
                  <MetricTile
                    size="compact"
                    frame="cell"
                    label="Treated / control units"
                    value={formatWords(
                      `${formatCount(panelPreflight.layout.treated.length).text} / ${formatCount(panelPreflight.layout.controls.length).text}`,
                    )}
                  />
                  <MetricTile
                    size="compact"
                    frame="cell"
                    label="Pre / post periods"
                    value={formatWords(
                      `${formatCount(panelPreflight.layout.prePeriods).text} / ${formatCount(panelPreflight.layout.postPeriods).text}`,
                    )}
                  />
                  <MetricTile
                    size="compact"
                    frame="cell"
                    label="Adoption period"
                    value={formatWords(panelPreflight.layout.adoption.label)}
                  />
                </MetricGrid>
              </section>
            )}
          {didRestrictions.size > 0 && (
            <section aria-label="Covariates not available" className="space-y-2">
              <h4 className="m-0 text-body font-medium text-ink">Covariates not available</h4>
              <ul className="m-0 list-none space-y-1 p-0 text-body text-muted">
                {didCandidates
                  .filter((c) => didRestrictions.has(c.id))
                  .map((c) => (
                    <li key={c.id}>
                      {c.name}: {didRestrictions.get(c.id)}
                    </li>
                  ))}
              </ul>
            </section>
          )}
          <MethodCaveats
            methods={
              method.ok
                ? [
                    propensityMethodForTarget(
                      configuration.kind === 'synthetic-control' &&
                        configuration.specification === 'ridge-augmented'
                        ? ridgeMethodDefinition(method.value)
                        : method.value,
                      study?.estimand.kind === 'average-treatment-effect-on-treated'
                        ? 'att'
                        : 'ate',
                    ),
                  ]
                : ESTIMATION_METHODS
            }
            eligibility={eligibility}
          />
        </section>
      )}
    </div>
  )

  // Runs for the selected study whose additive estimates share one axis; the newest carries the accent.
  const comparison = useMemo(() => {
    const comparable = runs.filter(
      (run) => run.study === identification?.study && run.estimate.effect.kind === 'additive',
    )
    if (comparable.length < 2) return null
    const rows: RunComparisonRow[] = comparable.map((run, index) => {
      const method = `${describeRun(run.configuration)}${
        run.kind === 'backdoor-linear-run'
          ? `${[
              describeFixedEffects(run.configuration.fixedEffects),
              describeCovariance(run.configuration.errors),
            ]
              .filter((part) => part !== null)
              .map((part) => `, ${part}`)
              .join('')}`
          : ''
      }`
      const time = formatTime(run.createdAt)
      return {
        label: `${method}, ${time}`,
        axisLabel: `${method}\n${time}`,
        estimate: run.estimate.effect.kind === 'additive' ? run.estimate.effect.value : 0,
        lower: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.lower,
        upper: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.upper,
        current: index === comparable.length - 1,
      }
    })
    return { option: runComparisonOption(rows, chartTheme), height: rows.length * 88 + 56 }
  }, [runs, identification, chartTheme])

  const ledger = (
    <>
      {comparison !== null && (
        <div className="border-b border-hair px-3 pb-1 pt-2">
          <EChart
            option={comparison.option}
            label="Recorded estimates compared on one axis"
            className="w-full"
            style={{ height: comparison.height }}
            testId="run-comparison"
          />
        </div>
      )}
      <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Estimation runs">
        {runs.length === 0 && <li className="px-3 py-2 text-faint">No saved runs.</li>}
        {[...runs].reverse().map((run) => {
          const bound = studies.find((candidate) => candidate.id === run.study)
          return bound === undefined ? null : (
            <ResultCard
              key={run.id}
              run={run}
              study={bound}
              current={false}
              stepLabel={stepLabel}
              onDelete={setPendingDelete}
            />
          )
        })}
      </ul>
    </>
  )
  const dependentProbes =
    pendingDelete === null
      ? 0
      : sensitivityRuns.filter((probe) =>
          probe.kind === 'honest-did-run' || probe.kind === 'pretest-coverage-run'
            ? probe.source.kind === 'estimation' && probe.source.run === pendingDelete.id
            : probe.estimationRun === pendingDelete.id,
        ).length

  const deleteDialog = (
    <ConfirmDialog
      open={pendingDelete !== null}
      title="Delete this estimate?"
      danger
      confirmLabel="Delete run"
      message={
        pendingDelete === null
          ? ''
          : `Removes the ${describeEstimator(pendingDelete.configuration.kind)} run${dependentProbes > 0 ? ` and the ${dependentProbes} sensitivity ${dependentProbes === 1 ? 'probe' : 'probes'} recorded against it` : ''}. Recorded results cannot be restored.`
      }
      onConfirm={() => {
        if (pendingDelete !== null) onDeleteRun(pendingDelete.id)
      }}
      onClose={() => setPendingDelete(null)}
    />
  )

  return (
    <>
      <WorkbenchLayout
        id="estimation"
        stage={stage}
        inspector={{
          trigger: { label: 'Requirements', icon: 'contract' },
          title: 'Study and method requirements',
          body: inspector,
        }}
        bottom={{
          trigger: { label: 'History', icon: 'history' },
          title: `Runs (${runs.length})`,
          body: (
            <>
              {ledger}
              {deleteDialog}
            </>
          ),
          defaultSize: comparison === null ? 150 : 150 + comparison.height,
        }}
      />
    </>
  )
}
