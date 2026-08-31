import { z } from 'zod'
import { dagCheckEvidenceSchema, type DagCheckEvidence } from '@/domain/dagValidation'
import { grangerSsrEvidenceSchema, parseGrangerSsrEvidence } from '@/domain/granger'
import type { GrangerSsrEvidence } from '@/domain/granger'
import { parseSeasonalAdjustedEvidence, seasonalAdjustedEvidenceSchema, type SeasonalAdjustedEvidence } from '@/domain/seasonal'
import { ardlEvidenceSchema, bayesianGaussianEvidenceSchema, binaryEttEvidenceSchema, discreteBnEvidenceSchema, doubleMlEvidenceSchema, negbinNutsEvidenceSchema, panelInterventionEvidenceSchema, syntheticControlEvidenceSchema, vecmEvidenceSchema, type ArdlEvidence, type BayesianGaussianEvidence, type BinaryEttEvidence, type DiscreteBnEvidence, type DoubleMlEvidence, type NegbinNutsEvidence, type PanelInterventionEvidence, type SyntheticControlEvidence, type VecmEvidence } from '@/domain/estimation'
import { dmlRefutationEvidenceSchema, parseDmlRefutationEvidence, type DmlRefutationEvidence } from '@/domain/sensitivity'
import { linearScmEvidenceSchema, type LinearScmEvidence } from '@/domain/counterfactual'
import { brand, err, ok, type Brand, type Result } from '@/domain/dop'
import {
  dynotearsEvidenceSchema,
  directLingamEvidenceSchema,
  lpcmciEvidenceSchema,
  ocseEvidenceSchema,
  parseDynotearsEvidence,
  parseDirectLingamEvidence,
  parseLpcmciEvidence,
  parseOcseEvidence,
  parsePcmciPlusEvidence,
  parseVarLingamEvidence,
  pcmciPlusEvidenceSchema,
  varLingamEvidenceSchema,
  type DynotearsEvidence,
  type DirectLingamEvidence,
  type LpcmciEvidence,
  type OcseEvidence,
  type PcmciPlusEvidence,
  type VarLingamEvidence,
} from '@/domain/discovery'
import {
  backdoorLinearEvidenceSchema,
  frontdoorTwoStageEvidenceSchema,
  causalEffectsEvidenceSchema,
  causalImpactEvidenceSchema,
  countGlmEvidenceSchema,
  parseBackdoorLinearEvidence,
  parseFrontdoorTwoStageEvidence,
  parseCausalEffectsEvidence,
  parseCausalImpactEvidence,
  parseCountGlmEvidence,
  type BackdoorLinearEvidence,
  type FrontdoorTwoStageEvidence,
  type CausalEffectsEvidence,
  type CausalImpactEvidence,
  type CountGlmEvidence,
} from '@/domain/estimation'
import {
  missingnessResolvedEvidenceSchema,
  parseMissingnessResolvedEvidence,
  type MissingnessResolvedEvidence,
} from '@/domain/missingness'
import {
  linearRefutationEvidenceSchema,
  parseLinearRefutationEvidence,
  parseSeriesStructureEvidence,
  parseUnobservedConfoundingEvidence,
  seriesStructureEvidenceSchema,
  unobservedConfoundingEvidenceSchema,
  type LinearRefutationEvidence,
  type SeriesStructureEvidence,
  type UnobservedConfoundingEvidence,
} from '@/domain/sensitivity'
import {
  parseStationarityBattery,
  stationarityBatterySchema,
  type StationarityBattery,
} from '@/domain/stationarity'
import {
  backdoorIdentificationEvidenceSchema,
  parseBackdoorIdentificationEvidence,
  type BackdoorIdentificationEvidence,
} from '@/domain/study'

export type WorkerRequestId = Brand<string, 'WorkerRequestId'>

export interface AnalysisProgress {
  readonly stage: string
  readonly completed: number
  readonly total: number
}

export type AnalysisWorkerCommand =
  | {
      readonly kind: 'stationarity-battery'
      readonly request: WorkerRequestId
      readonly values: Float64Array
    }
  | {
      readonly kind: 'pcmci-plus'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly tauMax: number
      readonly pcAlpha: number
    }
  | {
      readonly kind: 'lpcmci'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly tauMax: number
      readonly pcAlpha: number
    }
  | {
      readonly kind: 'dynotears'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly maxLag: number
      readonly lambdaW: number
      readonly lambdaA: number
    }
  | {
      readonly kind: 'direct-lingam'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
    }
  | {
      readonly kind: 'var-lingam'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly lags: number
      readonly prune: boolean
    }
  | {
      readonly kind: 'ocse'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly maxLag: number
      readonly alpha: number
      readonly nShuffles: number
      readonly method: 'gaussian' | 'knn'
      readonly k: number
    }
  | {
      readonly kind: 'granger-ssr-f'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly maxLag: number
    }
  | {
      readonly kind: 'backdoor-identify'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly nodes: number
      readonly names: readonly string[]
      readonly edges: readonly (readonly [number, number])[]
      readonly treatment: number
      readonly outcome: number
      readonly unobserved: readonly number[]
      readonly estimand: 'ate' | 'att'
    }
  | {
      readonly kind: 'dag-check'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly nodeColumns: readonly number[]
      readonly edges: readonly (readonly [number, number])[]
      readonly implications: readonly {
        readonly x: number
        readonly y: number
        readonly given: readonly number[]
      }[]
      readonly maximumObservations: number
      readonly permutations: number
      readonly significanceLevel: number
      readonly runFalsification: boolean
    }
  | {
      readonly kind: 'backdoor-linear'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly hacMaxLags: number | null
      readonly level: number
    }
  | {
      readonly kind: 'frontdoor-two-stage'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly mediator: number
      readonly outcome: number
      readonly firstStageAdjustment: readonly number[]
      readonly secondStageAdjustment: readonly number[]
      readonly controlValue: number
      readonly treatmentValue: number
      readonly uncertainty: {
        readonly kind: 'bootstrap'
        readonly simulations: number
        readonly sampleSizeFraction: number
        readonly confidenceLevel: number
        readonly seed: number
      }
    }
  | {
      readonly kind: 'count-glm'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly family: 'poisson' | 'negativeBinomial'
    }
  | {
      readonly kind: 'causal-effects-total'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly statLag: number
      readonly graph: readonly (readonly (readonly string[])[])[]
      readonly x: readonly (readonly [number, number])[]
      readonly y: readonly (readonly [number, number])[]
      readonly hidden: readonly (readonly [number, number])[]
      readonly estimator: { readonly kind: 'linear' } | { readonly kind: 'knn'; readonly k: number }
      readonly interventions: readonly [number, number]
    }
  | {
      readonly kind: 'causal-impact'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly outcome: number
      readonly controls: readonly number[]
      readonly nPre: number
      readonly maxIter: number
    }
  | {
      readonly kind: 'linear-refutation'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly simulations: number
      readonly subsetFraction: number
      readonly seed: number
      readonly ljungBoxLags: number
    }
  | {
      readonly kind: 'unobserved-confounding'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly seed: number
      readonly kappaT: readonly number[] | null
      readonly kappaY: readonly number[] | null
    }
  | {
      readonly kind: 'series-structure'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly period: number | null
      readonly robust: boolean
      readonly peltMinSize: number
      readonly peltJump: number
      readonly peltPenalty: number
    }
  | {
      readonly kind: 'double-ml'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly model: 'plr' | 'irm'
      readonly att: boolean
      readonly seed: number
    }
  | {
      readonly kind: 'dml-refutation-batch'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly model: 'plr' | 'irm'
      readonly att: boolean
      readonly seed: number
    }
  | {
      readonly kind: 'ardl-pss'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly maxLag: number
      readonly trend: 'c' | 'ct'
      readonly case: number
    }
  | {
      readonly kind: 'vecm'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly endogenous: readonly number[]
      readonly maxLags: number
      readonly deterministic: 'n' | 'co' | 'ci' | 'coli'
      readonly significance: number
      readonly breakIndex: number | null
    }
  | {
      readonly kind: 'synthetic-control'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treated: number
      readonly donors: readonly number[]
      readonly nPre: number
    }
  | {
      readonly kind: 'panel-intervention'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly units: readonly string[]
      readonly times: readonly number[]
    }
  | {
      readonly kind: 'negbin-nuts'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly confounder: number
      readonly warmup: number
      readonly samples: number
      readonly seed: number
    }
  | {
      readonly kind: 'bayesian-gaussian'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
      readonly warmup: number
      readonly samples: number
      readonly seed: number
    }
  | {
      readonly kind: 'discrete-bn-query'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly nodes: readonly number[]
      readonly names: readonly string[]
      readonly edges: readonly (readonly [number, number])[]
      readonly treatment: number
      readonly outcome: number
      readonly bins: number
      readonly equivalentSampleSize: number
    }
  | {
      readonly kind: 'binary-ett'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly observedNodes: readonly number[]
      readonly names: readonly string[]
      readonly edges: readonly (readonly [number, number])[]
      readonly treatment: number
      readonly outcome: number
      readonly unobserved: readonly number[]
    }
  | {
      readonly kind: 'linear-scm-counterfactual'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly nodes: readonly number[]
      readonly names: readonly string[]
      readonly edges: readonly (readonly [number, number])[]
      readonly treatment: number
      readonly outcome: number
      readonly interventions: readonly [number, number]
      readonly observationNoise: number | null
    }
  | {
      readonly kind: 'seasonal-adjust'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly period: number
      readonly robust: boolean
      readonly adjust: readonly number[]
    }
  | {
      readonly kind: 'resolve-missingness'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      /** One byte per cell, column-major: 1 observed, 0 missing. */
      readonly validity: Uint8Array
      readonly resolution:
        | { readonly kind: 'completeInterval' }
        | { readonly kind: 'imputation'; readonly method: 'linearInterior' | 'forwardFill' | 'structuralZero'; readonly maxGap: number; readonly confirmation: string | null }
    }

export type AnalysisWorkerProblem =
  | { readonly kind: 'kernel-refused'; readonly detail: string }
  | { readonly kind: 'wasm-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-unavailable'; readonly detail: string }
  | { readonly kind: 'worker-protocol-failed'; readonly detail: string }

export type AnalysisWorkerEvent =
  | {
      readonly kind: 'analysis-progress'
      readonly request: WorkerRequestId
      readonly progress: AnalysisProgress
    }
  | {
      readonly kind: 'stationarity-succeeded'
      readonly request: WorkerRequestId
      readonly result: StationarityBattery
    }
  | {
      readonly kind: 'pcmci-plus-succeeded'
      readonly request: WorkerRequestId
      readonly result: PcmciPlusEvidence
    }
  | {
      readonly kind: 'lpcmci-succeeded'
      readonly request: WorkerRequestId
      readonly result: LpcmciEvidence
    }
  | {
      readonly kind: 'dynotears-succeeded'
      readonly request: WorkerRequestId
      readonly result: DynotearsEvidence
    }
  | {
      readonly kind: 'direct-lingam-succeeded'
      readonly request: WorkerRequestId
      readonly result: DirectLingamEvidence
    }
  | {
      readonly kind: 'var-lingam-succeeded'
      readonly request: WorkerRequestId
      readonly result: VarLingamEvidence
    }
  | {
      readonly kind: 'ocse-succeeded'
      readonly request: WorkerRequestId
      readonly result: OcseEvidence
    }
  | {
      readonly kind: 'granger-succeeded'
      readonly request: WorkerRequestId
      readonly result: GrangerSsrEvidence
    }
  | {
      readonly kind: 'backdoor-identification-succeeded'
      readonly request: WorkerRequestId
      readonly result: BackdoorIdentificationEvidence
    }
  | { readonly kind: 'dag-check-succeeded'; readonly request: WorkerRequestId; readonly result: DagCheckEvidence }
  | {
      readonly kind: 'backdoor-linear-succeeded'
      readonly request: WorkerRequestId
      readonly result: BackdoorLinearEvidence
    }
  | { readonly kind: 'frontdoor-two-stage-succeeded'; readonly request: WorkerRequestId; readonly result: FrontdoorTwoStageEvidence }
  | { readonly kind: 'count-glm-succeeded'; readonly request: WorkerRequestId; readonly result: CountGlmEvidence }
  | { readonly kind: 'causal-effects-succeeded'; readonly request: WorkerRequestId; readonly result: CausalEffectsEvidence }
  | { readonly kind: 'causal-impact-succeeded'; readonly request: WorkerRequestId; readonly result: CausalImpactEvidence }
  | { readonly kind: 'linear-refutation-succeeded'; readonly request: WorkerRequestId; readonly result: LinearRefutationEvidence }
  | { readonly kind: 'unobserved-confounding-succeeded'; readonly request: WorkerRequestId; readonly result: UnobservedConfoundingEvidence }
  | { readonly kind: 'series-structure-succeeded'; readonly request: WorkerRequestId; readonly result: SeriesStructureEvidence }
  | { readonly kind: 'seasonal-adjusted'; readonly request: WorkerRequestId; readonly result: SeasonalAdjustedEvidence }
  | { readonly kind: 'double-ml-succeeded'; readonly request: WorkerRequestId; readonly result: DoubleMlEvidence }
  | { readonly kind: 'ardl-succeeded'; readonly request: WorkerRequestId; readonly result: ArdlEvidence }
  | { readonly kind: 'vecm-succeeded'; readonly request: WorkerRequestId; readonly result: VecmEvidence }
  | { readonly kind: 'synthetic-control-succeeded'; readonly request: WorkerRequestId; readonly result: SyntheticControlEvidence }
  | { readonly kind: 'panel-intervention-succeeded'; readonly request: WorkerRequestId; readonly result: PanelInterventionEvidence }
  | { readonly kind: 'negbin-nuts-succeeded'; readonly request: WorkerRequestId; readonly result: NegbinNutsEvidence }
  | { readonly kind: 'bayesian-gaussian-succeeded'; readonly request: WorkerRequestId; readonly result: BayesianGaussianEvidence }
  | { readonly kind: 'discrete-bn-succeeded'; readonly request: WorkerRequestId; readonly result: DiscreteBnEvidence }
  | { readonly kind: 'binary-ett-succeeded'; readonly request: WorkerRequestId; readonly result: BinaryEttEvidence }
  | { readonly kind: 'linear-scm-succeeded'; readonly request: WorkerRequestId; readonly result: LinearScmEvidence }
  | { readonly kind: 'dml-refutation-succeeded'; readonly request: WorkerRequestId; readonly result: DmlRefutationEvidence }
  | { readonly kind: 'missingness-resolved'; readonly request: WorkerRequestId; readonly result: MissingnessResolvedEvidence }
  | {
      readonly kind: 'analysis-failed'
      readonly request: WorkerRequestId
      readonly problem: AnalysisWorkerProblem
    }
  | { readonly kind: 'protocol-failed'; readonly detail: string }

export type AnalysisProtocolProblem =
  | { readonly kind: 'invalid-command'; readonly detail: string }
  | { readonly kind: 'invalid-event'; readonly detail: string }

const requestSchema = z.string().uuid()

const workerRequestId = (value: string): Result<WorkerRequestId, { readonly kind: 'invalid-worker-request-id' }> =>
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(value)
    ? ok(brand<string, 'WorkerRequestId'>(value))
    : err({ kind: 'invalid-worker-request-id' })

export const newWorkerRequestId = (): WorkerRequestId =>
  brand<string, 'WorkerRequestId'>(crypto.randomUUID())

const commandSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('stationarity-battery'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
  }).strict(),
  z.object({
    kind: z.literal('pcmci-plus'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(32),
    tauMax: z.number().int().min(1).max(20),
    pcAlpha: z.number().finite().positive().max(1),
  }).strict(),
  z.object({
    kind: z.literal('lpcmci'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(32),
    tauMax: z.number().int().min(1).max(20),
    pcAlpha: z.number().finite().positive().max(1),
  }).strict(),
  z.object({
    kind: z.literal('dynotears'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    maxLag: z.number().int().min(1).max(6),
    lambdaW: z.number().finite().nonnegative(),
    lambdaA: z.number().finite().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('direct-lingam'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
  }).strict(),
  z.object({
    kind: z.literal('var-lingam'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    lags: z.number().int().min(1).max(6),
    prune: z.boolean(),
  }).strict(),
  z.object({
    kind: z.literal('ocse'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    maxLag: z.number().int().min(1).max(8),
    alpha: z.number().finite().positive().max(1),
    nShuffles: z.number().int().min(20).max(2_000),
    method: z.enum(['gaussian', 'knn']),
    k: z.number().int().min(1).max(20),
  }).strict(),
  z.object({
    kind: z.literal('granger-ssr-f'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    maxLag: z.number().int().min(1).max(20),
  }).strict(),
  z.object({
    kind: z.literal('backdoor-identify'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    nodes: z.number().int().min(2).max(64),
    names: z.array(z.string().trim().min(1)).min(2).max(64),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    unobserved: z.array(z.number().int().nonnegative()),
    estimand: z.enum(['ate', 'att']),
  }).strict(),
  z.object({
    kind: z.literal('dag-check'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(4),
    columns: z.number().int().min(2).max(256),
    nodeColumns: z.array(z.number().int().nonnegative()).min(2).max(64),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    implications: z.array(z.object({
      x: z.number().int().nonnegative(),
      y: z.number().int().nonnegative(),
      given: z.array(z.number().int().nonnegative()),
    }).strict()).min(1),
    maximumObservations: z.number().int().min(4).max(2_000),
    permutations: z.number().int().min(20).max(2_000),
    significanceLevel: z.number().gt(0).lte(0.25),
    runFalsification: z.boolean(),
  }).strict(),
  z.object({
    kind: z.literal('backdoor-linear'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()),
    hacMaxLags: z.number().int().nonnegative().nullable(),
    level: z.number().gt(0.5).lt(1),
  }).strict(),
  z.object({
    kind: z.literal('frontdoor-two-stage'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(3).max(64),
    treatment: z.number().int().nonnegative(),
    mediator: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    firstStageAdjustment: z.array(z.number().int().nonnegative()),
    secondStageAdjustment: z.array(z.number().int().nonnegative()),
    controlValue: z.number().finite(),
    treatmentValue: z.number().finite(),
    uncertainty: z.object({
      kind: z.literal('bootstrap'),
      simulations: z.number().int().min(20).max(10_000),
      sampleSizeFraction: z.number().gt(0).max(2),
      confidenceLevel: z.number().gt(0.5).lt(1),
      seed: z.number().int().nonnegative(),
    }).strict(),
  }).strict(),
  z.object({
    kind: z.literal('count-glm'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()),
    family: z.enum(['poisson', 'negativeBinomial']),
  }).strict(),
  z.object({
    kind: z.literal('causal-effects-total'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    statLag: z.number().int().min(0).max(20),
    graph: z.array(z.array(z.array(z.string().max(3)))),
    x: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().max(0)])).min(1),
    y: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().max(0)])).length(1),
    hidden: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().max(0)])),
    estimator: z.discriminatedUnion('kind', [z.object({ kind: z.literal('linear') }).strict(), z.object({ kind: z.literal('knn'), k: z.number().int().min(1).max(100) }).strict()]),
    interventions: z.tuple([z.number().finite(), z.number().finite()]),
  }).strict(),
  z.object({
    kind: z.literal('causal-impact'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(1).max(64),
    outcome: z.number().int().nonnegative(),
    controls: z.array(z.number().int().nonnegative()),
    nPre: z.number().int().min(8),
    maxIter: z.number().int().min(1).max(2000),
  }).strict(),
  z.object({
    kind: z.literal('linear-refutation'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()),
    simulations: z.number().int().min(1).max(2000),
    subsetFraction: z.number().gt(0.1).lt(1),
    seed: z.number().int().min(0).max(4294967295),
    ljungBoxLags: z.number().int().min(1).max(200),
  }).strict(),
  z.object({
    kind: z.literal('unobserved-confounding'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(3).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()).min(1),
    seed: z.number().int().min(0).max(4294967295),
    kappaT: z.array(z.number().min(0).max(1)).min(1).max(200).nullable(),
    kappaY: z.array(z.number().finite()).min(1).max(200).nullable(),
  }).strict(),
  z.object({
    kind: z.literal('series-structure'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(1).max(64),
    period: z.number().int().min(2).max(400).nullable(),
    robust: z.boolean(),
    peltMinSize: z.number().int().min(1).max(1000),
    peltJump: z.number().int().min(1).max(100),
    peltPenalty: z.number().finite().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('double-ml'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()).min(1),
    model: z.enum(['plr', 'irm']),
    att: z.boolean(),
    seed: z.number().int().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('dml-refutation-batch'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()).min(1),
    model: z.enum(['plr', 'irm']),
    att: z.boolean(),
    seed: z.number().int().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('ardl-pss'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    maxLag: z.number().int().min(1).max(24),
    trend: z.enum(['c', 'ct']),
    case: z.number().int().min(2).max(5),
  }).strict(),
  z.object({
    kind: z.literal('vecm'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    endogenous: z.array(z.number().int().nonnegative()).min(2),
    maxLags: z.number().int().min(1).max(24),
    deterministic: z.enum(['n', 'co', 'ci', 'coli']),
    significance: z.number().int().min(0).max(2),
    breakIndex: z.number().int().nonnegative().nullable(),
  }).strict(),
  z.object({
    kind: z.literal('synthetic-control'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treated: z.number().int().nonnegative(),
    donors: z.array(z.number().int().nonnegative()).min(1),
    nPre: z.number().int().min(2),
  }).strict(),
  z.object({
    kind: z.literal('panel-intervention'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    units: z.array(z.string().min(1)).min(1),
    times: z.array(z.number().int().nonnegative()).min(1),
  }).strict(),
  z.object({
    kind: z.literal('negbin-nuts'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(3).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    confounder: z.number().int().nonnegative(),
    warmup: z.number().int().min(10).max(5000),
    samples: z.number().int().min(10).max(5000),
    seed: z.number().int().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('bayesian-gaussian'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()),
    warmup: z.number().int().min(10).max(5000),
    samples: z.number().int().min(10).max(5000),
    seed: z.number().int().nonnegative(),
  }).strict(),
  z.object({
    kind: z.literal('discrete-bn-query'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    nodes: z.array(z.number().int().nonnegative()).min(2),
    names: z.array(z.string().min(1)).min(2),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    bins: z.number().int().min(2).max(10),
    equivalentSampleSize: z.number().positive(),
  }).strict(),
  z.object({
    kind: z.literal('binary-ett'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    observedNodes: z.array(z.number().int().nonnegative()).min(2).max(64),
    names: z.array(z.string().trim().min(1)).min(2).max(64),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    unobserved: z.array(z.number().int().nonnegative()),
  }).strict(),
  z.object({
    kind: z.literal('linear-scm-counterfactual'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    nodes: z.array(z.number().int().nonnegative()).min(2),
    names: z.array(z.string().min(1)).min(2),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    interventions: z.tuple([z.number().finite(), z.number().finite()]),
    observationNoise: z.number().positive().nullable(),
  }).strict(),
  z.object({
    kind: z.literal('seasonal-adjust'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(1).max(64),
    period: z.number().int().min(2).max(400),
    robust: z.boolean(),
    adjust: z.array(z.number().int().nonnegative()).min(1),
  }).strict(),
  z.object({
    kind: z.literal('resolve-missingness'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(1).max(256),
    validity: z.instanceof(Uint8Array),
    resolution: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('completeInterval') }).strict(),
      z.object({ kind: z.literal('imputation'), method: z.enum(['linearInterior', 'forwardFill', 'structuralZero']), maxGap: z.number().int().min(1).max(100000), confirmation: z.string().nullable() }).strict(),
    ]),
  }).strict(),
])

const workerProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('kernel-refused'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('wasm-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
])

export const analysisProgressSchema = z.object({
  stage: z.string().trim().min(1),
  completed: z.number().int().nonnegative(),
  total: z.number().int().positive(),
}).strict().refine((progress) => progress.completed <= progress.total, {
  message: 'Completed progress cannot exceed its total.',
})

const eventSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('analysis-progress'),
    request: requestSchema,
    progress: analysisProgressSchema,
  }).strict(),
  z.object({
    kind: z.literal('stationarity-succeeded'),
    request: requestSchema,
    result: stationarityBatterySchema,
  }).strict(),
  z.object({
    kind: z.literal('pcmci-plus-succeeded'),
    request: requestSchema,
    result: pcmciPlusEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('lpcmci-succeeded'),
    request: requestSchema,
    result: lpcmciEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('dynotears-succeeded'),
    request: requestSchema,
    result: dynotearsEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('direct-lingam-succeeded'),
    request: requestSchema,
    result: directLingamEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('var-lingam-succeeded'),
    request: requestSchema,
    result: varLingamEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('ocse-succeeded'),
    request: requestSchema,
    result: ocseEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('granger-succeeded'),
    request: requestSchema,
    result: grangerSsrEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('backdoor-identification-succeeded'),
    request: requestSchema,
    result: backdoorIdentificationEvidenceSchema,
  }).strict(),
  z.object({ kind: z.literal('dag-check-succeeded'), request: requestSchema, result: dagCheckEvidenceSchema }).strict(),
  z.object({
    kind: z.literal('backdoor-linear-succeeded'),
    request: requestSchema,
    result: backdoorLinearEvidenceSchema,
  }).strict(),
  z.object({ kind: z.literal('frontdoor-two-stage-succeeded'), request: requestSchema, result: frontdoorTwoStageEvidenceSchema }).strict(),
  z.object({ kind: z.literal('count-glm-succeeded'), request: requestSchema, result: countGlmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('causal-effects-succeeded'), request: requestSchema, result: causalEffectsEvidenceSchema }).strict(),
  z.object({ kind: z.literal('causal-impact-succeeded'), request: requestSchema, result: causalImpactEvidenceSchema }).strict(),
  z.object({ kind: z.literal('linear-refutation-succeeded'), request: requestSchema, result: linearRefutationEvidenceSchema }).strict(),
  z.object({ kind: z.literal('unobserved-confounding-succeeded'), request: requestSchema, result: unobservedConfoundingEvidenceSchema }).strict(),
  z.object({ kind: z.literal('series-structure-succeeded'), request: requestSchema, result: seriesStructureEvidenceSchema }).strict(),
  z.object({ kind: z.literal('seasonal-adjusted'), request: requestSchema, result: seasonalAdjustedEvidenceSchema }).strict(),
  z.object({ kind: z.literal('double-ml-succeeded'), request: requestSchema, result: doubleMlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('ardl-succeeded'), request: requestSchema, result: ardlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('vecm-succeeded'), request: requestSchema, result: vecmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('synthetic-control-succeeded'), request: requestSchema, result: syntheticControlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('panel-intervention-succeeded'), request: requestSchema, result: panelInterventionEvidenceSchema }).strict(),
  z.object({ kind: z.literal('negbin-nuts-succeeded'), request: requestSchema, result: negbinNutsEvidenceSchema }).strict(),
  z.object({ kind: z.literal('bayesian-gaussian-succeeded'), request: requestSchema, result: bayesianGaussianEvidenceSchema }).strict(),
  z.object({ kind: z.literal('discrete-bn-succeeded'), request: requestSchema, result: discreteBnEvidenceSchema }).strict(),
  z.object({ kind: z.literal('binary-ett-succeeded'), request: requestSchema, result: binaryEttEvidenceSchema }).strict(),
  z.object({ kind: z.literal('linear-scm-succeeded'), request: requestSchema, result: linearScmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('dml-refutation-succeeded'), request: requestSchema, result: dmlRefutationEvidenceSchema }).strict(),
  z.object({ kind: z.literal('missingness-resolved'), request: requestSchema, result: missingnessResolvedEvidenceSchema }).strict(),
  z.object({
    kind: z.literal('analysis-failed'),
    request: requestSchema,
    problem: workerProblemSchema,
  }).strict(),
  z.object({ kind: z.literal('protocol-failed'), detail: z.string() }).strict(),
])

export function parseAnalysisWorkerCommand(value: unknown): Result<AnalysisWorkerCommand, AnalysisProtocolProblem> {
  const parsed = commandSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-command', detail: z.prettifyError(parsed.error) })
  const request = workerRequestId(parsed.data.request)
  if (!request.ok) return err({ kind: 'invalid-command', detail: 'The worker request identity is invalid.' })
  if ('columns' in parsed.data && parsed.data.values.length !== parsed.data.rows * parsed.data.columns) {
    return err({ kind: 'invalid-command', detail: 'The multivariate matrix dimensions do not match its numeric buffer.' })
  }
  if (parsed.data.kind === 'granger-ssr-f' && parsed.data.values.length !== parsed.data.rows * 2) {
    return err({ kind: 'invalid-command', detail: 'The Granger matrix must contain exactly two columns.' })
  }
  if (parsed.data.kind === 'panel-intervention' && (
    parsed.data.values.length !== parsed.data.rows * 2
    || parsed.data.units.length !== parsed.data.rows
    || parsed.data.times.length !== parsed.data.rows
  )) {
    return err({ kind: 'invalid-command', detail: 'Panel values, unit keys, and time keys must describe the same rows.' })
  }
  if (parsed.data.kind === 'backdoor-identify' && parsed.data.values.length !== 0) {
    return err({ kind: 'invalid-command', detail: 'Identification takes a graph, not data.' })
  }
  if (parsed.data.kind === 'backdoor-identify' && (
    parsed.data.names.length !== parsed.data.nodes
    || new Set(parsed.data.names).size !== parsed.data.names.length
  )) {
    return err({ kind: 'invalid-command', detail: 'Identification requires one distinct name for every graph node.' })
  }
  if (parsed.data.kind === 'resolve-missingness' && parsed.data.validity.length !== parsed.data.rows * parsed.data.columns) {
    return err({ kind: 'invalid-command', detail: 'The validity bytes do not match the matrix dimensions.' })
  }
  return ok({ ...parsed.data, request: request.value })
}

export function parseAnalysisWorkerEvent(value: unknown): Result<AnalysisWorkerEvent, AnalysisProtocolProblem> {
  const parsed = eventSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-event', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'protocol-failed') return ok(parsed.data)
  const request = workerRequestId(parsed.data.request)
  if (!request.ok) return err({ kind: 'invalid-event', detail: 'The worker request identity is invalid.' })
  if (parsed.data.kind === 'analysis-failed') return ok({ ...parsed.data, request: request.value })
  if (parsed.data.kind === 'analysis-progress') return ok({ ...parsed.data, request: request.value })
  if (parsed.data.kind === 'pcmci-plus-succeeded') {
    const result = parsePcmciPlusEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'pcmci-plus-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'lpcmci-succeeded') {
    const result = parseLpcmciEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'lpcmci-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'dynotears-succeeded') {
    const result = parseDynotearsEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'dynotears-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'direct-lingam-succeeded') {
    const result = parseDirectLingamEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'direct-lingam-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'var-lingam-succeeded') {
    const result = parseVarLingamEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'var-lingam-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'ocse-succeeded') {
    const result = parseOcseEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'ocse-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'granger-succeeded') {
    const result = parseGrangerSsrEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'granger-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'backdoor-identification-succeeded') {
    const result = parseBackdoorIdentificationEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'backdoor-identification-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'dag-check-succeeded') {
    const result = dagCheckEvidenceSchema.safeParse(parsed.data.result)
    return result.success
      ? ok({ kind: 'dag-check-succeeded', request: request.value, result: result.data })
      : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'backdoor-linear-succeeded') {
    const result = parseBackdoorLinearEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'backdoor-linear-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'frontdoor-two-stage-succeeded') {
    const result = parseFrontdoorTwoStageEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'frontdoor-two-stage-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'count-glm-succeeded') {
    const result = parseCountGlmEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'count-glm-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'causal-effects-succeeded') {
    const result = parseCausalEffectsEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'causal-effects-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'causal-impact-succeeded') {
    const result = parseCausalImpactEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'causal-impact-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'linear-refutation-succeeded') {
    const result = parseLinearRefutationEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'linear-refutation-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'unobserved-confounding-succeeded') {
    const result = parseUnobservedConfoundingEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'unobserved-confounding-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'series-structure-succeeded') {
    const result = parseSeriesStructureEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'series-structure-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'double-ml-succeeded') {
    const result = doubleMlEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'double-ml-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'ardl-succeeded') {
    const result = ardlEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'ardl-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'vecm-succeeded') {
    const result = vecmEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'vecm-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'synthetic-control-succeeded') {
    const result = syntheticControlEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'synthetic-control-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'panel-intervention-succeeded') {
    const result = panelInterventionEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'panel-intervention-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'negbin-nuts-succeeded') {
    const result = negbinNutsEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'negbin-nuts-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'bayesian-gaussian-succeeded') {
    const result = bayesianGaussianEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'bayesian-gaussian-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'discrete-bn-succeeded') {
    const result = discreteBnEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'discrete-bn-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'binary-ett-succeeded') {
    const result = binaryEttEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'binary-ett-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'linear-scm-succeeded') {
    const result = linearScmEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'linear-scm-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'dml-refutation-succeeded') {
    const result = parseDmlRefutationEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'dml-refutation-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'seasonal-adjusted') {
    const result = parseSeasonalAdjustedEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'seasonal-adjusted', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'missingness-resolved') {
    const result = parseMissingnessResolvedEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'missingness-resolved', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  const result = parseStationarityBattery(parsed.data.result)
  return result.ok
    ? ok({ kind: 'stationarity-succeeded', request: request.value, result: result.value })
    : err({ kind: 'invalid-event', detail: result.error.detail })
}
