import { surrogateRunSchema, surrogateRunMatchesProfile, type SurrogateRun } from './surrogateRun'
import { swigAnalysisSchema, type SwigAnalysis } from './swig'
import { z } from 'zod'
import {
  predictorSyntheticConfigurationSchema,
  predictorSyntheticEvidenceSchema,
  predictorSyntheticCatalogSchema,
  predictorSyntheticRecordMatches,
  predictorSyntheticEstimateMatches,
} from './predictorSyntheticControl'
import {
  causalForestConfigurationSchema,
  causalForestEvidenceSchema,
  causalForestTarget,
  sameCausalForestTarget,
  causalForestSettingsMatch,
} from './causalForest'
import { ridgeRecordMatches } from './ridgeAugmented'
import { sunAbrahamRecordMatches } from './sunAbraham'
import { staggeredRecordMatches, staggeredConfigurationSchema } from './staggeredDid'
import { sharpRdConfigurationSchema, parseSharpRdEvidence, sharpRdRecordMatches } from './sharpRd'
import {
  EMPTY_ROOT_CAUSE,
  rootCauseWorkspaceSchema,
  type RootCauseWorkspace,
} from './rootCauseAnalysis'
import type { CounterfactualRunArtifact } from './counterfactual'
import type { DagDocument } from './dag'
import type { DagCheckArtifact } from './dagValidation'
import { parseDatasetProfile, type DatasetProfile } from './dataset'
import type { DiscoveryRunArtifact } from './discovery'
import type { GrangerEvidenceArtifact } from './granger'
import type { CountSeriesModelArtifact } from './countSeries'
import type { InterventionQueryArtifact } from './intervention'
import { networkQueryArtifactSchema } from './networkQuery'
import { conditionalGaussianArtifactSchema } from './conditionalGaussianQuery'
import { brand, err, ok, type Result } from './dop'
import type { EstimationRunArtifact } from './estimation'
import { backdoorLinearConfigurationSchema, backdoorLinearEvidenceSchema } from './estimation'
import {
  tLearnerEvidenceSchema,
  crossFittedTLearnerEvidenceSchema,
  tLearnerConfigurationSchema,
  tLearnerRunMatches,
  parseCausalImpactEvidence,
  bayesianImpactSettingsSchema,
  impactInferenceMatches,
} from './estimation'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from './preprocessing'
import type { SensitivityRunArtifact } from './sensitivity'
import { honestRunSchema, honestRunMatches } from './honestDid'
import { didSensitivityRunSchema, didSensitivityRunMatches } from './didSensitivity'
import {
  EMPTY_STUDY_DRAFT,
  type IdentificationArtifact,
  type StudyDesignDraft,
  type StudySpecification,
} from './study'
import type { ProjectOrigin } from './projectOrigin'
import type { Project, SelectedSource, Workflow } from './workflow'
import type { SurvivalRunArtifact } from './survival'
import { timeSeriesRunMatches, timeSeriesRunSchema, type TimeSeriesRun } from './timeSeries'
import { parseSourceRecipe, type SourceRecipe } from './sqlPreparation'

/**
 * What a project keeps between sessions: the manifest, the recorded artifacts and a description of the
 * source file, never its rows (DESIGN.md §5.6). A reopened project asks for the file again and checks
 * its fingerprint against the stored profile before any artifact is trusted with it.
 */

export interface SourceDescriptor {
  readonly name: string
  readonly bytes: number
  readonly mediaType: string
  readonly lastModified: number
  readonly format: SelectedSource['format']
  readonly recipe: SourceRecipe
}

export interface PersistedProject {
  readonly kind: 'hirmos-project'
  readonly version: 1
  readonly savedAt: string
  readonly origin: ProjectOrigin
  readonly project: Project
  readonly source: SourceDescriptor | null
  readonly profile: DatasetProfile | null
  readonly prepared: PreparedDatasetArtifact | null
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly discoveryRuns: readonly DiscoveryRunArtifact[]
  readonly dagDocuments: readonly DagDocument[]
  readonly dagChecks: readonly DagCheckArtifact[]
  readonly interventionQueries: readonly InterventionQueryArtifact[]
  readonly swigAnalyses: readonly SwigAnalysis[]
  readonly studyDraft: StudyDesignDraft
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
  readonly surrogateRuns: readonly SurrogateRun[]
  readonly survivalRuns: readonly SurvivalRunArtifact[]
  readonly timeSeriesRuns: readonly TimeSeriesRun[]
  readonly rootCause: RootCauseWorkspace
}

/** The lines a project list shows without opening the record. */
export interface SavedProjectHeader {
  readonly id: Project['id']
  readonly name: string
  readonly savedAt: string
  readonly sourceName: string | null
  /** The fingerprint of a source cached in this browser, so reopening can skip the file prompt and deleting can clear the cache. */
  readonly cachedSource: DatasetProfile['source']['fingerprint'] | null
  readonly estimationRuns: number
}

export const headerOf = (snapshot: PersistedProject): SavedProjectHeader => ({
  id: snapshot.project.id,
  name: snapshot.project.name,
  savedAt: snapshot.savedAt,
  sourceName: snapshot.source?.name ?? null,
  cachedSource:
    snapshot.profile !== null && snapshot.profile.source.persistence.kind === 'cached-locally'
      ? snapshot.profile.source.fingerprint
      : null,
  estimationRuns: snapshot.estimationRuns.length + snapshot.surrogateRuns.length,
})

const describeSource = (source: SelectedSource): SourceDescriptor => ({
  name: source.name,
  bytes: source.bytes,
  mediaType: source.mediaType,
  lastModified: source.lastModified,
  format: source.format,
  recipe: source.recipe,
})

/** The record to write for the workflow as it stands, or null when nothing durable exists yet or a restore is pending. */
export function snapshotWorkflow(workflow: Workflow, savedAt: string): PersistedProject | null {
  switch (workflow.kind) {
    case 'awaiting-project':
      return null
    case 'awaiting-data':
      if (workflow.restore !== null) return null
      return {
        kind: 'hirmos-project',
        version: 1,
        savedAt,
        origin: workflow.origin,
        project: workflow.project,
        source: null,
        profile: null,
        prepared: null,
        stationarity: null,
        grangerEvidence: [],
        countSeriesModels: [],
        discoveryRuns: [],
        dagDocuments: [],
        dagChecks: [],
        interventionQueries: [],
        swigAnalyses: [],
        studyDraft: EMPTY_STUDY_DRAFT,
        studies: [],
        identifications: [],
        estimationRuns: [],
        sensitivityRuns: [],
        counterfactualRuns: [],
        surrogateRuns: [],
        survivalRuns: [],
        timeSeriesRuns: [],
        rootCause: EMPTY_ROOT_CAUSE,
      }
    case 'sql-inputs-chosen':
    case 'pipeline-opened':
    case 'awaiting-editor-files':
    case 'source-selected':
    case 'profiling':
    case 'import-failed':
      return null
    case 'profiled':
      return {
        kind: 'hirmos-project',
        version: 1,
        savedAt,
        origin: workflow.origin,
        project: workflow.project,
        source: describeSource(workflow.source),
        profile: workflow.profile,
        prepared: workflow.prepared,
        stationarity: workflow.stationarity,
        grangerEvidence: workflow.grangerEvidence,
        countSeriesModels: workflow.countSeriesModels,
        discoveryRuns: workflow.discoveryRuns,
        dagDocuments: workflow.dagDocuments,
        dagChecks: workflow.dagChecks,
        interventionQueries: workflow.interventionQueries,
        swigAnalyses: workflow.swigAnalyses,
        studyDraft: workflow.studyDraft,
        studies: workflow.studies,
        identifications: workflow.identifications,
        estimationRuns: workflow.estimationRuns,
        sensitivityRuns: workflow.sensitivityRuns,
        counterfactualRuns: workflow.counterfactualRuns,
        surrogateRuns: workflow.surrogateRuns,
        survivalRuns: workflow.survivalRuns,
        timeSeriesRuns: workflow.timeSeriesRuns,
        rootCause: workflow.rootCause,
      }
    default:
      return null
  }
}

const F64 = '$f64'

/** JSON with typed arrays tagged, so a record round-trips through a string store without losing its numeric columns. */
export const serialiseSnapshot = (snapshot: PersistedProject): string =>
  JSON.stringify(snapshot, (_key, value: unknown) =>
    value instanceof Float64Array ? { [F64]: Array.from(value) } : value,
  )

/**
 * Whether two records contain the same durable analysis. `savedAt` describes a write, not the
 * analysis, so opening an unchanged project must not make it appear newly edited.
 */
export const samePersistedProjectContent = (
  left: PersistedProject,
  right: PersistedProject,
): boolean =>
  serialiseSnapshot({ ...left, savedAt: '' }) === serialiseSnapshot({ ...right, savedAt: '' })

const revive = (_key: string, value: unknown): unknown => {
  if (typeof value === 'object' && value !== null && F64 in value) {
    const numbers = (value as Record<string, unknown>)[F64]
    return Array.isArray(numbers) ? Float64Array.from(numbers as number[]) : value
  }
  return value
}

export type SnapshotProblem =
  | { readonly kind: 'not-json'; readonly detail: string }
  | { readonly kind: 'invalid-snapshot'; readonly detail: string }
  | { readonly kind: 'unsupported-version'; readonly version: number }

const artifact = z.object({ id: z.string().min(1) }).passthrough()
const projectOriginSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('user') }).strict(),
  z
    .object({
      kind: z.literal('shipped-example'),
      exportedAt: z.string().datetime({ offset: true }),
    })
    .strict(),
])
const seriesTransformSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('levels') }).strict(),
  z.object({ kind: z.literal('difference'), order: z.literal(1) }).strict(),
  z.object({ kind: z.literal('linear-detrend') }).strict(),
])

const sourceMetadataSchema = z.object({
  name: z.string(),
  bytes: z.number().int().nonnegative(),
  mediaType: z.string(),
  lastModified: z.number(),
  format: z.enum(['csv', 'tsv', 'parquet']),
})
const currentSourceSchema = sourceMetadataSchema.extend({ recipe: z.unknown() }).strict()

const envelopeSchema = z.object({
  kind: z.literal('hirmos-project'),
  version: z.number().int(),
  savedAt: z.string().min(1),
  origin: projectOriginSchema.default({ kind: 'user' }),
  project: z
    .object({
      id: z.string().min(1),
      name: z.string().min(1),
      createdAt: z.string().min(1),
    })
    .strict(),
  source: currentSourceSchema.nullable(),
  profile: z.unknown().nullable(),
  prepared: artifact.nullable(),
  stationarity: artifact.nullable(),
  grangerEvidence: z.array(artifact).default([]),
  countSeriesModels: z.array(artifact).default([]),
  discoveryRuns: z.array(artifact),
  dagDocuments: z.array(artifact),
  dagChecks: z.array(artifact).default([]),
  swigAnalyses: z.array(swigAnalysisSchema).default([]),
  interventionQueries: z
    .array(
      artifact.superRefine((value, ctx) => {
        if (value.kind === 'network-query') {
          const parsed = networkQueryArtifactSchema.safeParse(value)
          if (!parsed.success)
            ctx.addIssue({ code: 'custom', message: z.prettifyError(parsed.error) })
        }
        if (value.kind === 'conditional-gaussian-query') {
          const parsed = conditionalGaussianArtifactSchema.safeParse(value)
          if (!parsed.success)
            ctx.addIssue({ code: 'custom', message: z.prettifyError(parsed.error) })
        }
      }),
    )
    .default([]),
  studyDraft: z.object({}).passthrough(),
  studies: z.array(artifact),
  identifications: z.array(artifact),
  estimationRuns: z.array(artifact),
  sensitivityRuns: z.array(
    artifact.superRefine((run, ctx) => {
      const schema =
        run.kind === 'honest-did-run'
          ? honestRunSchema
          : run.kind === 'did-sensitivity-run'
            ? didSensitivityRunSchema
            : null
      if (schema !== null) {
        const parsed = schema.safeParse(run)
        if (!parsed.success)
          ctx.addIssue({ code: 'custom', message: z.prettifyError(parsed.error) })
      }
    }),
  ),
  counterfactualRuns: z.array(artifact),
  surrogateRuns: z.array(surrogateRunSchema).default([]),
  survivalRuns: z.array(artifact).default([]),
  timeSeriesRuns: z.array(timeSeriesRunSchema).default([]),
  rootCause: rootCauseWorkspaceSchema.default(EMPTY_ROOT_CAUSE),
})
type ParsedEnvelope = z.output<typeof envelopeSchema>

/**
 * Reads a stored record back. The envelope, the project and the dataset profile are parsed in full;
 * artifact bodies are checked for their identity and otherwise trusted as the shapes this build wrote,
 * since the record only ever comes from this application's own store.
 */
export function parseSnapshot(raw: string): Result<PersistedProject, SnapshotProblem> {
  let value: unknown
  try {
    value = JSON.parse(raw, revive)
  } catch (cause) {
    return err({ kind: 'not-json', detail: cause instanceof Error ? cause.message : String(cause) })
  }
  return parseSnapshotValue(value)
}

/** Tagged-JSON helpers shared with the export bundle, so a bundle and a stored record read the same way. */
export const taggedJsonReplacer = (_key: string, value: unknown): unknown =>
  value instanceof Float64Array ? { [F64]: Array.from(value) } : value
export const taggedJsonReviver = revive

/** Validation is derived from the graph, so a stored revision's copy is replaced by this build's reading of it. */
const upgradeDagDocumentRecord = (value: Record<string, unknown>): Record<string, unknown> => {
  const dataset = Reflect.get(value, 'dataset') as DagDocument['dataset'] | undefined
  if (dataset === undefined) return value
  const refresh = (revision: unknown): unknown => {
    if (revision === null || typeof revision !== 'object') return revision
    const graph = Reflect.get(revision, 'graph') as EditableDag | undefined
    return graph === undefined
      ? revision
      : { ...revision, validation: inspectDagStructure(graph, dataset) }
  }
  const list = (key: string) =>
    Array.isArray(Reflect.get(value, key))
      ? (Reflect.get(value, key) as unknown[]).map(refresh)
      : Reflect.get(value, key)
  return {
    ...value,
    current: refresh(Reflect.get(value, 'current')),
    history: list('history'),
    future: list('future'),
    audit: list('audit'),
  }
}

/** Add preparation fields introduced while the version-1 envelope remained stable. */
const upgradePreparedTransformRecord = (
  value: ParsedEnvelope['prepared'],
): Result<ParsedEnvelope['prepared'], SnapshotProblem> => {
  if (value === null || Reflect.get(value, 'kind') !== 'prepared-time-series') return ok(value)
  const coverage = Reflect.get(value, 'calendarCoverage')
  if (coverage !== undefined && !calendarEdgeSchema.safeParse(coverage).success)
    return err({ kind: 'invalid-snapshot', detail: 'The saved calendar coverage is invalid.' })
  const columns = Reflect.get(value, 'columns')
  if (!Array.isArray(columns) || !columns.every((column) => typeof column === 'string')) {
    return err({ kind: 'invalid-snapshot', detail: 'prepared time series: columns are missing' })
  }
  return ok({
    ...value,
    seriesTransforms: Array.isArray(Reflect.get(value, 'seriesTransforms'))
      ? Reflect.get(value, 'seriesTransforms')
      : columns.map((column) => ({ column, transform: { kind: 'levels' } })),
    resampling: Reflect.get(value, 'resampling') ?? { kind: 'none' },
  })
}

/** Rename the version-1 stationarity display field; no numerical evidence is recomputed. */
const upgradeStationarityTransformRecord = (
  value: ParsedEnvelope['stationarity'],
): Result<ParsedEnvelope['stationarity'], SnapshotProblem> => {
  if (
    value === null ||
    Reflect.get(value, 'kind') !== 'stationarity-evidence' ||
    Reflect.get(value, 'diagnosticTransform') !== undefined
  )
    return ok(value)
  const legacy = seriesTransformSchema.safeParse(Reflect.get(value, 'transform'))
  if (!legacy.success)
    return err({
      kind: 'invalid-snapshot',
      detail: 'stationarity evidence: diagnostic transform is missing',
    })
  const { transform: _legacyTransform, ...rest } = value
  return ok({ ...rest, diagnosticTransform: legacy.data })
}

/**
 * Upgrade estimation records written before applied adjustment and synthetic inference became
 * tagged evidence. The numerical result is retained; inference that was never computed is recorded
 * explicitly rather than fabricated during project loading.
 */
const upgradeEstimationRunRecord = (record: Record<string, unknown>): Record<string, unknown> => {
  // A run saved before covariate encodings were declarable entered every column as a number.
  let value = Reflect.get(record, 'encodings') === undefined ? { ...record, encodings: {} } : record
  const estimate = Reflect.get(value, 'estimate')
  let upgradedEstimate = estimate

  if (typeof upgradedEstimate === 'object' && upgradedEstimate !== null) {
    const effect = Reflect.get(upgradedEstimate, 'effect')
    if (
      typeof effect === 'object' &&
      effect !== null &&
      Reflect.get(effect, 'kind') === 'incidenceRateRatio'
    ) {
      upgradedEstimate = {
        ...upgradedEstimate,
        effect: { ...effect, kind: 'expectedCountRatio' },
      }
    }
  }

  if (
    typeof upgradedEstimate === 'object' &&
    upgradedEstimate !== null &&
    Reflect.get(upgradedEstimate, 'adjustment') === undefined
  ) {
    const legacy = Reflect.get(upgradedEstimate, 'adjustmentSet')
    if (Array.isArray(legacy)) {
      const { adjustmentSet: _legacyAdjustment, ...rest } = upgradedEstimate as Record<
        string,
        unknown
      >
      upgradedEstimate = {
        ...rest,
        adjustment:
          legacy.length === 0 ? { kind: 'none' } : { kind: 'contemporaneous', variables: legacy },
      }
    }
  }

  const legacyConfiguration = Reflect.get(value, 'configuration')
  const legacyEvidence = Reflect.get(value, 'evidence')
  if (
    typeof legacyConfiguration === 'object' &&
    legacyConfiguration !== null &&
    Reflect.get(legacyConfiguration, 'primary') === 'sunAbraham' &&
    Reflect.get(legacyConfiguration, 'referenceCohorts') === undefined &&
    typeof legacyEvidence === 'object' &&
    legacyEvidence !== null
  ) {
    const request = Reflect.get(legacyEvidence, 'request')
    const references =
      typeof request === 'object' && request !== null
        ? Reflect.get(request, 'referenceCohorts')
        : undefined
    if (Array.isArray(references) && references.length === 0)
      value = { ...value, configuration: { ...legacyConfiguration, referenceCohorts: [] } }
  }
  const configuration = Reflect.get(value, 'configuration')
  const rawEvidence = Reflect.get(value, 'evidence')
  const propensityRun = [
    'propensity-weighting-run',
    'propensity-matching-run',
    'doubly-robust-run',
  ].includes(String(Reflect.get(value, 'kind')))
  const historicAte =
    typeof upgradedEstimate === 'object' &&
    upgradedEstimate !== null &&
    typeof Reflect.get(upgradedEstimate, 'estimand') === 'object' &&
    Reflect.get(upgradedEstimate, 'estimand') !== null &&
    Reflect.get(Reflect.get(upgradedEstimate, 'estimand'), 'kind') === 'average-treatment-effect'
  const evidence =
    propensityRun &&
    historicAte &&
    typeof rawEvidence === 'object' &&
    rawEvidence !== null &&
    Reflect.get(rawEvidence, 'target') === undefined
      ? { ...rawEvidence, target: 'ate' }
      : rawEvidence
  if (evidence !== rawEvidence) value = { ...value, evidence }
  // An adjusted regression saved before the error process was a choice: its covariance name
  // becomes the error treatment, and its evidence records that no ARMA fit was made.
  if (
    Reflect.get(value, 'kind') === 'backdoor-linear-run' &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    const covariance = Reflect.get(configuration, 'covariance')
    const { covariance: _legacyCovariance, ...rest } = configuration as Record<string, unknown>
    // A run saved before fixed effects were a choice absorbed none.
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: {
        fixedEffects: { kind: 'none' },
        ...(covariance === 'hac' || covariance === 'classical'
          ? { ...rest, errors: { kind: covariance } }
          : configuration),
      },
      evidence: { errorModel: { kind: 'neweyWest' }, fixedEffects: { kind: 'none' }, ...evidence },
    }
  }
  // A propensity run saved while boosted scoring was a flag: a cross-fitted run's recorded
  // validation AUC was the AUC of its cross-fitted scores, so that is the one figure it keeps.
  if (
    (Reflect.get(value, 'kind') === 'propensity-weighting-run' ||
      Reflect.get(value, 'kind') === 'propensity-matching-run') &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    const boosted = Reflect.get(configuration, 'boosted')
    const model = Reflect.get(evidence, 'treatmentModel')
    const flagged = (holder: unknown) =>
      typeof holder === 'object' &&
      holder !== null &&
      typeof Reflect.get(holder, 'crossFitted') === 'boolean'
    if (flagged(boosted) || flagged(model)) {
      const scoringOf = (holder: object) =>
        Reflect.get(holder, 'crossFitted') === true ? 'cross-fitted' : 'one-model'
      const withoutFlag = (holder: object) =>
        Object.fromEntries(Object.entries(holder).filter(([key]) => key !== 'crossFitted'))
      const upgradedBoosted = flagged(boosted)
        ? { ...withoutFlag(boosted as object), scoring: scoringOf(boosted as object) }
        : boosted
      const upgradedModel = flagged(model)
        ? (() => {
            const { validationAuc, fittedAuc, ...rest } = withoutFlag(model as object)
            return {
              ...rest,
              scoring:
                Reflect.get(model as object, 'crossFitted') === true
                  ? { kind: 'crossFitted', auc: fittedAuc }
                  : { kind: 'oneModel', validationAuc, fittedAuc },
            }
          })()
        : model
      return {
        ...value,
        estimate: upgradedEstimate,
        configuration: { ...configuration, boosted: upgradedBoosted },
        evidence: { ...evidence, treatmentModel: upgradedModel },
      }
    }
  }
  // A T-learner saved before the outcome model was a choice fitted random forests, and one saved
  // before intervals existed requested none.
  if (
    Reflect.get(value, 'kind') === 't-learner-run' &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    const model = Reflect.get(configuration, 'model') ?? {
      kind: 'forest',
      uncertainty: { kind: 'none' },
      seed: Reflect.get(configuration, 'seed'),
      ...(Reflect.get(configuration, 'uncertainty') === undefined
        ? {}
        : { uncertainty: Reflect.get(configuration, 'uncertainty') }),
    }
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: { kind: 't-learner', model },
      evidence:
        Reflect.get(evidence, 'kind') === 'tLearner'
          ? { uncertainty: { kind: 'none' }, ...evidence }
          : evidence,
    }
  }
  // An impact run saved before the evaluated window was a choice covered every row after the
  // intervention, so its window runs through the last row and its window ends at the last row.
  if (
    Reflect.get(value, 'kind') === 'causal-impact-run' &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    const observations = Reflect.get(evidence, 'observations')
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: { window: { kind: 'through-last-row' }, ...configuration },
      evidence: {
        preInterventionPath: { kind: 'notReported' },
        ...(typeof observations === 'number' ? { postEnd: observations } : {}),
        ...evidence,
      },
    }
  }
  // A generalised propensity score run saved before its weights were recorded keeps its estimate
  // and records the weights as missing.
  if (
    Reflect.get(value, 'kind') === 'continuous-gps-run' &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    return { ...value, estimate: upgradedEstimate, evidence: { weights: null, ...evidence } }
  }
  if (
    Reflect.get(value, 'kind') === 'synthetic-control-run' &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null
  ) {
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: {
        crossFitFolds: 3,
        alpha: 0.05,
        ...configuration,
      },
      evidence: {
        crossFit: {
          kind: 'unavailable',
          reason:
            'This saved run predates cross-fitted inference; run the estimator again to compute it.',
        },
        donorPlacebo: {
          kind: 'unavailable',
          reason:
            'This saved run predates donor-placebo inference; run the estimator again to compute it.',
        },
        conformalBand: {
          kind: 'unavailable',
          reason:
            'This saved run predates prediction bands; run the estimator again to compute them.',
        },
        gaussianBand: {
          kind: 'unavailable',
          reason:
            'This saved run predates prediction bands; run the estimator again to compute them.',
        },
        ...evidence,
      },
    }
  }
  if (
    Reflect.get(value, 'kind') === 'panel-intervention-run' &&
    typeof configuration === 'object' &&
    configuration !== null &&
    typeof evidence === 'object' &&
    evidence !== null &&
    Reflect.get(evidence, 'kind') !== 'panelDid' &&
    Reflect.get(evidence, 'kind') !== 'panelAdjusted' &&
    Reflect.get(evidence, 'kind') !== 'staggeredDid' &&
    Reflect.get(evidence, 'kind') !== 'sunAbraham'
  ) {
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: {
        placeboReplications: 100,
        seed: 0,
        ...configuration,
      },
      evidence: {
        syntheticControlPlacebo: {
          kind: 'unavailable',
          reason:
            'This saved run predates panel placebo inference; run the estimator again to compute it.',
        },
        syntheticDidPlacebo: {
          kind: 'unavailable',
          reason:
            'This saved run predates panel placebo inference; run the estimator again to compute it.',
        },
        syntheticControlInTime: {
          kind: 'unavailable',
          reason:
            'This saved run predates the in-time placebo; run the estimator again to compute it.',
        },
        syntheticDidInTime: {
          kind: 'unavailable',
          reason:
            'This saved run predates the in-time placebo; run the estimator again to compute it.',
        },
        ...evidence,
      },
    }
  }
  // A DML run saved before group effects carries the plain average and nothing to group by.
  if (
    Reflect.get(value, 'kind') === 'double-ml-run' &&
    typeof evidence === 'object' &&
    evidence !== null &&
    Reflect.get(evidence, 'groups') === undefined
  ) {
    return {
      ...value,
      estimate: upgradedEstimate,
      evidence: { ...evidence, groups: { kind: 'none' } },
    }
  }
  return upgradedEstimate === estimate ? value : { ...value, estimate: upgradedEstimate }
}

/**
 * An identification recorded before the instrument search existed carries no instrument set. DoWhy's
 * search is a graph result, so the record is completed with "not identified" rather than re-run: a
 * back-door or ID-algorithm record stays exactly what it was, and the instrument route stays closed
 * until the study is identified again.
 */
const upgradeIdentificationRecord = (value: Record<string, unknown>): Record<string, unknown> => {
  const result = Reflect.get(value, 'result')
  const evidence = Reflect.get(value, 'evidence')
  const resultKind =
    typeof result === 'object' && result !== null ? Reflect.get(result, 'kind') : null
  const needsResult =
    (resultKind === 'identified' || resultKind === 'graphically-identified') &&
    Reflect.get(result as object, 'instruments') === undefined
  const needsEvidence =
    typeof evidence === 'object' &&
    evidence !== null &&
    Reflect.get(evidence, 'instruments') === undefined
  if (!needsResult && !needsEvidence) return value
  return {
    ...value,
    ...(needsResult
      ? {
          result: {
            ...(result as Record<string, unknown>),
            instruments: { kind: 'not-identified' },
          },
        }
      : {}),
    ...(needsEvidence
      ? {
          evidence: {
            ...(evidence as Record<string, unknown>),
            instruments: { kind: 'notIdentified' },
          },
        }
      : {}),
  }
}

/** Preserve early survival runs while recording that their chart diagnostics were not stored. */
const upgradeSurvivalRunRecord = (value: Record<string, unknown>): Record<string, unknown> => {
  const kind = Reflect.get(value, 'kind')
  if (kind === 'two-group-survival-run') {
    const evidence = Reflect.get(value, 'evidence')
    if (typeof evidence !== 'object' || evidence === null) return value
    const additions = {
      ...(Reflect.get(evidence, 'diagnostics') === undefined
        ? { diagnostics: { kind: 'notRecorded' } }
        : {}),
      ...(Reflect.get(evidence, 'observedConversion') === undefined
        ? { observedConversion: { kind: 'notRecorded' } }
        : {}),
      ...(Reflect.get(evidence, 'fixedTimeConversion') === undefined
        ? { fixedTimeConversion: { kind: 'notRecorded' } }
        : {}),
      ...(Reflect.get(evidence, 'petoPeto') === undefined
        ? { petoPeto: { kind: 'notRecorded' } }
        : {}),
    }
    if (Object.keys(additions).length === 0) return value
    return {
      ...value,
      evidence: { ...(evidence as Record<string, unknown>), ...additions },
    }
  }
  if (kind === 'cox-regression-run') {
    // A Cox run recorded before the shared frailty existed carries no frailty field in its
    // right-censored configuration or in its evidence.
    const configuration = Reflect.get(value, 'configuration')
    const evidence = Reflect.get(value, 'evidence')
    const observation =
      typeof configuration === 'object' && configuration !== null
        ? Reflect.get(configuration, 'observation')
        : null
    const needsConfiguration =
      typeof observation === 'object' &&
      observation !== null &&
      Reflect.get(observation, 'kind') === 'right-censored' &&
      Reflect.get(observation, 'frailty') === undefined
    const needsEvidence =
      typeof evidence === 'object' &&
      evidence !== null &&
      Reflect.get(evidence, 'frailty') === undefined
    const needsFitting =
      typeof evidence === 'object' &&
      evidence !== null &&
      Reflect.get(evidence, 'fitting') === undefined
    const frailty =
      typeof evidence === 'object' && evidence !== null ? Reflect.get(evidence, 'frailty') : null
    const fitting = {
      kind:
        typeof frailty === 'object' && frailty !== null && Reflect.get(frailty, 'kind') === 'gamma'
          ? 'gammaFrailty'
          : 'efron',
    }
    if (!needsConfiguration && !needsEvidence && !needsFitting) return value
    return {
      ...value,
      ...(needsConfiguration
        ? {
            configuration: {
              ...(configuration as Record<string, unknown>),
              observation: {
                ...(observation as Record<string, unknown>),
                frailty: { kind: 'none' },
              },
            },
          }
        : {}),
      ...(needsEvidence || needsFitting
        ? {
            evidence: {
              ...(evidence as Record<string, unknown>),
              ...(needsEvidence ? { frailty: { kind: 'none' } } : {}),
              ...(needsFitting ? { fitting } : {}),
            },
          }
        : {}),
    }
  }
  if (kind !== 'multi-state-survival-run') return value
  const configuration = Reflect.get(value, 'configuration')
  const evidence = Reflect.get(value, 'evidence')
  if (
    typeof configuration !== 'object' ||
    configuration === null ||
    typeof evidence !== 'object' ||
    evidence === null
  )
    return value
  const preparedConfiguration =
    Reflect.get(configuration, 'input') === undefined
      ? {
          ...(configuration as Record<string, unknown>),
          input: {
            kind: 'prepared-transition-rows',
            start: Reflect.get(configuration, 'start'),
            stop: Reflect.get(configuration, 'stop'),
            event: Reflect.get(configuration, 'event'),
            from: Reflect.get(configuration, 'from'),
            to: Reflect.get(configuration, 'to'),
          },
        }
      : configuration
  const preparedEvidence =
    Reflect.get(evidence, 'preparation') === undefined
      ? { ...(evidence as Record<string, unknown>), preparation: { kind: 'preparedRows' } }
      : evidence
  return { ...value, configuration: preparedConfiguration, evidence: preparedEvidence }
}

export function parseSnapshotValue(value: unknown): Result<PersistedProject, SnapshotProblem> {
  const parsed = envelopeSchema.safeParse(value)
  if (!parsed.success)
    return err({ kind: 'invalid-snapshot', detail: z.prettifyError(parsed.error) })
  if (parsed.data.version !== 1)
    return err({ kind: 'unsupported-version', version: parsed.data.version })
  let source: SourceDescriptor | null = null
  if (parsed.data.source !== null) {
    const recipe = parseSourceRecipe(parsed.data.source.recipe)
    if (!recipe.ok) return err({ kind: 'invalid-snapshot', detail: recipe.error.detail })
    source = { ...parsed.data.source, recipe: recipe.value }
  }
  const prepared = upgradePreparedTransformRecord(parsed.data.prepared)
  if (!prepared.ok) return prepared
  if (parsed.data.timeSeriesRuns.length > 0) {
    const series = prepared.value as PreparedDatasetArtifact | null
    if (
      series === null ||
      !Array.isArray(series.columns) ||
      (series.kind !== 'prepared-time-series' &&
        parsed.data.timeSeriesRuns.some(
          (run) =>
            !(
              run.kind === 'panel-regression' &&
              run.specification.specification.kind === 'interactions'
            ) &&
            !(
              series.kind === 'prepared-panel' &&
              ['count-regression', 'panel-regression', 'bacon'].includes(run.kind)
            ),
        )) ||
      parsed.data.timeSeriesRuns.some((run) => !timeSeriesRunMatches(run, series))
    ) {
      return err({
        kind: 'invalid-snapshot',
        detail: 'A time-series run does not belong to the prepared time series in this project.',
      })
    }
  }
  const stationarity = upgradeStationarityTransformRecord(parsed.data.stationarity)
  if (!stationarity.ok) return stationarity
  let profile: DatasetProfile | null = null
  if (parsed.data.profile !== null) {
    const profileParsed = parseDatasetProfile(parsed.data.profile)
    if (!profileParsed.ok)
      return err({ kind: 'invalid-snapshot', detail: `profile: ${profileParsed.error.kind}` })
    profile = profileParsed.value
  }
  // Granger moved from the Discovery Lab to the data diagnostics; a run recorded there by an earlier build has no reader now.
  const discoveryRuns = parsed.data.discoveryRuns.filter(
    (run) => Reflect.get(run, 'kind') !== 'granger-ssr-f-run',
  )
  if (
    parsed.data.surrogateRuns.some(
      (run) => profile === null || !surrogateRunMatchesProfile(run, profile),
    )
  )
    return err({
      kind: 'invalid-snapshot',
      detail: 'A surrogate run does not match its source profile.',
    })
  const storedDraft = parsed.data.studyDraft as Partial<StudyDesignDraft>
  const studyDraft: StudyDesignDraft = { ...EMPTY_STUDY_DRAFT, ...storedDraft }
  const estimationRuns = parsed.data.estimationRuns.map((run) => upgradeEstimationRunRecord(run))
  for (const run of estimationRuns) {
    if (
      ['propensity-weighting-run', 'propensity-matching-run', 'doubly-robust-run'].includes(
        String(run.kind),
      )
    ) {
      const study = parsed.data.studies.find((s) => s.id === run.study)
      const target =
        typeof run.evidence === 'object' && run.evidence !== null
          ? Reflect.get(run.evidence, 'target')
          : null
      const estimand =
        typeof study?.estimand === 'object' && study.estimand !== null
          ? Reflect.get(study.estimand, 'kind')
          : null
      if (
        target !==
        (estimand === 'average-treatment-effect-on-treated'
          ? 'att'
          : estimand === 'average-treatment-effect'
            ? 'ate'
            : null)
      )
        return err({
          kind: 'invalid-snapshot',
          detail: 'The saved propensity result does not match its study target.',
        })
    }
    if (run.kind === 'ridge-augmented-synthetic-run') {
      const study = parsed.data.studies.find((s) => s.id === run.study),
        identification = parsed.data.identifications.find((i) => i.id === run.identification)
      if (
        !ridgeRecordMatches(run, study) ||
        identification?.study !== run.study ||
        prepared.value === null ||
        Reflect.get(prepared.value, 'id') !== run.preparedDataset
      )
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved ridge-augmented result does not match its ATT study, specification or prepared panel.',
        })
    }
    if (run.kind === 'predictor-synthetic-control-run') {
      const configuration = predictorSyntheticConfigurationSchema.safeParse(run.configuration)
      const evidence = predictorSyntheticEvidenceSchema.safeParse(run.evidence)
      const catalog = predictorSyntheticCatalogSchema.safeParse(run.catalog)
      const columns = z
        .array(
          z
            .object({
              column: z
                .string()
                .min(1)
                .transform((v) => brand<string, 'ColumnId'>(v)),
              name: z.string().min(1),
            })
            .strict(),
        )
        .min(1)
        .safeParse(run.columns)
      const studyRecord = parsed.data.studies.find((s) => s.id === run.study)
      const studyFields = z
        .object({
          estimand: z.object({ kind: z.literal('average-treatment-effect-on-treated') }),
          outcome: z.object({
            column: z
              .string()
              .min(1)
              .transform((v) => brand<string, 'ColumnId'>(v)),
          }),
        })
        .safeParse(studyRecord)
      const identification = parsed.data.identifications.find((i) => i.id === run.identification)
      if (
        !configuration.success ||
        !evidence.success ||
        !catalog.success ||
        !columns.success ||
        !studyFields.success ||
        identification?.study !== run.study ||
        prepared.value === null ||
        Reflect.get(prepared.value, 'id') !== run.preparedDataset ||
        !predictorSyntheticRecordMatches(
          configuration.data,
          evidence.data,
          catalog.data,
          columns.data,
          studyFields.data.outcome.column,
        ) ||
        !predictorSyntheticEstimateMatches(run.estimate, configuration.data, evidence.data) ||
        columns.data.some(
          (c) => !profile?.columns.some((p) => p.id === c.column && p.name === c.name),
        )
      )
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved predictor-based synthetic control does not match its treated-unit study, prepared panel or fitting specification.',
        })
    }
    if (run.kind === 'backdoor-linear-run') {
      const configuration = backdoorLinearConfigurationSchema.safeParse(run.configuration)
      const evidence = backdoorLinearEvidenceSchema.safeParse(run.evidence)
      if (!configuration.success || !evidence.success)
        return err({
          kind: 'invalid-snapshot',
          detail: 'The saved adjusted regression has an invalid configuration or result.',
        })
      const effects = configuration.data.fixedEffects
      const groupings = [
        ...(effects.kind === 'none' ? [] : [{ column: effects.column, name: effects.name }]),
        ...(effects.kind === 'unit-and-time'
          ? [{ column: effects.timeColumn, name: effects.timeName }]
          : []),
        ...(configuration.data.errors.kind === 'cluster' ? [configuration.data.errors] : []),
      ]
      if (
        groupings.some(
          (group) =>
            !profile?.columns.some(
              (column) => column.id === group.column && column.name === group.name,
            ),
        )
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail: 'A saved fixed-effect or cluster column does not match the dataset profile.',
        })
      }
      const expected = effects.kind === 'unit-and-time' ? 'unitAndTime' : effects.kind
      const errors = configuration.data.errors.kind
      if (
        evidence.data.fixedEffects.kind !== expected ||
        evidence.data.errorModel.kind !==
          (errors === 'classical' || errors === 'hac' ? 'neweyWest' : errors)
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved adjusted regression result does not match its fixed effects or uncertainty specification.',
        })
      }
    }
    if (
      run.kind === 'panel-intervention-run' &&
      ((typeof run.evidence === 'object' &&
        run.evidence !== null &&
        Reflect.get(run.evidence, 'kind') === 'sunAbraham') ||
        (typeof run.configuration === 'object' &&
          run.configuration !== null &&
          Reflect.get(run.configuration, 'primary') === 'sunAbraham'))
    ) {
      const study = parsed.data.studies.find((s) => s.id === run.study)
      const identification = parsed.data.identifications.find((i) => i.id === run.identification)
      if (
        !sunAbrahamRecordMatches(run, study) ||
        identification?.study !== run.study ||
        prepared.value === null ||
        Reflect.get(prepared.value, 'id') !== run.preparedDataset
      )
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved Sun–Abraham result does not match its ATT study, specification or prepared panel.',
        })
    }
    if (
      run.kind === 'panel-intervention-run' &&
      ((typeof run.evidence === 'object' &&
        run.evidence !== null &&
        Reflect.get(run.evidence, 'kind') === 'staggeredDid') ||
        (typeof run.configuration === 'object' &&
          run.configuration !== null &&
          Reflect.get(run.configuration, 'primary') === 'staggered'))
    ) {
      const study = parsed.data.studies.find((candidate) => candidate.id === run.study)
      const identification = parsed.data.identifications.find(
        (candidate) => candidate.id === run.identification,
      )
      if (
        !staggeredRecordMatches(run, study) ||
        identification?.study !== run.study ||
        prepared.value === null ||
        Reflect.get(prepared.value, 'id') !== run.preparedDataset
      )
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved staggered DiD result does not match its treated-group study, specification or prepared panel.',
        })
      const configuration = staggeredConfigurationSchema.safeParse(run.configuration)
      if (
        configuration.success &&
        'clustering' in configuration.data &&
        configuration.data.clustering.kind === 'column'
      ) {
        const column = configuration.data.clustering.column
        if (!profile?.columns.some((c) => c.id === column))
          return err({
            kind: 'invalid-snapshot',
            detail: 'The saved staggered DiD cluster column is not in the dataset profile.',
          })
      }
    }
    if (
      run.kind === 'panel-intervention-run' &&
      ((typeof run.evidence === 'object' &&
        run.evidence !== null &&
        Reflect.get(run.evidence, 'kind') === 'panelAdjusted') ||
        (typeof run.configuration === 'object' &&
          run.configuration !== null &&
          Reflect.get(run.configuration, 'primary') === 'adjusted'))
    ) {
      const study = parsed.data.studies.find((candidate) => candidate.id === run.study)
      const identification = parsed.data.identifications.find(
        (candidate) => candidate.id === run.identification,
      )
      if (
        !adjustedDidRecordMatches(run, study) ||
        identification?.study !== run.study ||
        prepared.value === null ||
        Reflect.get(prepared.value, 'id') !== run.preparedDataset
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved adjusted DiD result does not match its specification, ATT study or prepared panel.',
        })
      }
    }
    if (run.kind === 'sharp-rd-run') {
      const configuration = sharpRdConfigurationSchema.safeParse(run.configuration)
      const evidence = parseSharpRdEvidence(run.evidence)
      const study = parsed.data.studies.find((candidate) => candidate.id === run.study)
      const identification = parsed.data.identifications.find(
        (candidate) => candidate.id === run.identification,
      )
      if (
        !configuration.success ||
        !evidence.ok ||
        !sharpRdRecordMatches(run, study, identification, evidence.value)
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved sharp RD result does not match its cutoff-local study and robust inference.',
        })
      }
    }
    if (run.kind === 'causal-impact-run') {
      const start = z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('from-treatment') }).strict(),
        z.object({ kind: z.literal('row'), row: z.number().int().min(9) }).strict(),
      ])
      const window = z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('through-last-row') }).strict(),
        z.object({ kind: z.literal('to-row'), row: z.number().int().positive() }).strict(),
      ])
      const common = {
        kind: z.literal('causal-impact'),
        start,
        window,
        controls: z.array(z.string().min(1)),
      }
      const configuration = z
        .union([
          z.object({ ...common, maxIter: z.number().int().min(1).max(2000) }).strict(),
          z.object({ ...common, inference: bayesianImpactSettingsSchema }).strict(),
        ])
        .safeParse(run.configuration)
      const evidence = parseCausalImpactEvidence(run.evidence)
      if (
        !configuration.success ||
        !evidence.ok ||
        !impactInferenceMatches(
          {
            inference: 'inference' in configuration.data ? configuration.data.inference : undefined,
          },
          evidence.value,
        ) ||
        configuration.data.controls.length !== evidence.value.controls.length ||
        new Set(configuration.data.controls).size !== configuration.data.controls.length ||
        (configuration.data.start.kind === 'row' &&
          configuration.data.start.row - 1 !== evidence.value.nPre) ||
        (configuration.data.window.kind === 'to-row' &&
          configuration.data.window.row !== evidence.value.postEnd)
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail: 'The saved causal-impact result does not match its inference settings.',
        })
      }
    }
    if (run.kind === 'causal-forest-run') {
      const settings = causalForestConfigurationSchema.safeParse(run.configuration)
      const evidence = causalForestEvidenceSchema.safeParse(run.evidence)
      const study = parsed.data.studies.find((study) => study.id === run.study)
      const target =
        study === undefined
          ? null
          : causalForestTarget((study as unknown as StudySpecification).estimand)
      if (
        !settings.success ||
        !evidence.success ||
        target === null ||
        !sameCausalForestTarget(target, evidence.data.target) ||
        !causalForestSettingsMatch(settings.data, evidence.data)
      ) {
        return err({
          kind: 'invalid-snapshot',
          detail:
            'The saved causal forest result does not match its study target or forest settings.',
        })
      }
    }
    if (run.kind !== 't-learner-run') continue
    const evidence = z
      .discriminatedUnion('kind', [tLearnerEvidenceSchema, crossFittedTLearnerEvidenceSchema])
      .safeParse(run.evidence)
    const settings = tLearnerConfigurationSchema.safeParse(run.configuration)
    if (
      !evidence.success ||
      !settings.success ||
      !tLearnerRunMatches(settings.data, evidence.data)
    ) {
      return err({
        kind: 'invalid-snapshot',
        detail: 'The saved T-learner result does not match its outcome model settings.',
      })
    }
  }
  for (const record of parsed.data.sensitivityRuns) {
    if (record.kind === 'did-sensitivity-run') {
      const run = didSensitivityRunSchema.safeParse(record)
      if (
        !run.success ||
        !didSensitivityRunMatches(
          run.data,
          estimationRuns as unknown as PersistedProject['estimationRuns'],
        )
      )
        return err({
          kind: 'invalid-snapshot',
          detail: 'DiD sensitivity evidence does not match its saved two-period DR DiD source.',
        })
      continue
    }
    if (record.kind !== 'honest-did-run') continue
    const run = honestRunSchema.safeParse(record)
    if (
      !run.success ||
      !honestRunMatches(
        run.data,
        estimationRuns as unknown as PersistedProject['estimationRuns'],
        parsed.data.timeSeriesRuns,
      )
    )
      return err({
        kind: 'invalid-snapshot',
        detail: 'Parallel-trends sensitivity evidence does not match its saved event-study source.',
      })
  }
  const survivalRuns = parsed.data.survivalRuns.map((run) => upgradeSurvivalRunRecord(run))
  const identifications = parsed.data.identifications.map((identification) =>
    upgradeIdentificationRecord(identification),
  )
  const project: Project = {
    id: brand<string, 'ProjectId'>(parsed.data.project.id),
    name: brand<string, 'ProjectName'>(parsed.data.project.name),
    createdAt: parsed.data.project.createdAt,
  }
  return ok({
    ...(parsed.data as unknown as PersistedProject),
    version: 1,
    origin: parsed.data.origin as ProjectOrigin,
    project,
    source,
    profile,
    prepared: prepared.value as PreparedDatasetArtifact | null,
    stationarity: stationarity.value as StationarityEvidenceArtifact | null,
    studyDraft,
    discoveryRuns: discoveryRuns as unknown as PersistedProject['discoveryRuns'],
    identifications: identifications as unknown as PersistedProject['identifications'],
    estimationRuns: estimationRuns as unknown as PersistedProject['estimationRuns'],
    survivalRuns: survivalRuns as unknown as PersistedProject['survivalRuns'],
    dagDocuments: parsed.data.dagDocuments.map((record) =>
      upgradeDagDocumentRecord(record as Record<string, unknown>),
    ) as unknown as PersistedProject['dagDocuments'],
  })
}

export function describeSnapshotProblem(problem: SnapshotProblem): string {
  switch (problem.kind) {
    case 'not-json':
      return `The saved project could not be read: ${problem.detail}`
    case 'invalid-snapshot':
      return `The saved project is damaged: ${problem.detail}`
    case 'unsupported-version':
      return `The saved project was written by a newer version (${problem.version}).`
    default: {
      const exhaustive: never = problem
      return exhaustive
    }
  }
}
import { adjustedDidRecordMatches } from './adjustedDid'
import { calendarEdgeSchema } from './windowEvidence'
import { inspectDagStructure, type EditableDag } from './dag'
