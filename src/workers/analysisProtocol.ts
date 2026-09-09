import { z } from 'zod'
import { multicollinearityEvidenceSchema, parseMulticollinearityEvidence, type MulticollinearityEvidence } from '@/domain/multicollinearity'
import { countSeriesInterventionScanEvidenceSchema, parseCountSeriesInterventionScanEvidence, type CountSeriesInterventionScanEvidence } from '@/domain/countSeries'
import { dagCheckEvidenceSchema, type DagCheckEvidence } from '@/domain/dagValidation'
import { identifiedDiscreteQueryEvidenceSchema, type IdentifiedDiscreteQueryEvidence } from '@/domain/intervention'
import { grangerSsrEvidenceSchema, parseGrangerSsrEvidence } from '@/domain/granger'
import type { GrangerSsrEvidence } from '@/domain/granger'
import { parseSeasonalAdjustedEvidence, seasonalAdjustedEvidenceSchema, type SeasonalAdjustedEvidence } from '@/domain/seasonal'
import { pandasResamplingEvidenceSchema, parsePandasResamplingEvidence, type PandasResamplingEvidence, type ResamplingAggregation } from '@/domain/resampling'
import { ardlEvidenceSchema, bayesianGaussianEvidenceSchema, binaryEttEvidenceSchema, causalEffectsUncertaintySchema, discreteBnEvidenceSchema, doubleMlEvidenceSchema, ingarchInterventionScheduleSchema, negbinNutsEvidenceSchema, negativeBinomialIngarchEvidenceSchema, panelInterventionEvidenceSchema, syntheticControlEvidenceSchema, tLearnerEvidenceSchema, totalEffectEstimatorSchema, vecmEvidenceSchema, type ArdlEvidence, type BayesianGaussianEvidence, type BinaryEttEvidence, type CausalEffectsUncertainty, type DiscreteBnEvidence, type DoubleMlEvidence, type IngarchInterventionSchedule, type NegbinNutsEvidence, type NegativeBinomialIngarchEvidence, type PanelInterventionEvidence, type SyntheticControlEvidence, type TLearnerEvidence, type TotalEffectEstimator, type VecmEvidence } from '@/domain/estimation'
import { dmlRefutationEvidenceSchema, parseDmlRefutationEvidence, type DmlRefutationEvidence } from '@/domain/sensitivity'
import { dynamicCounterfactualUncertaintySchema, dynamicLinearScmEvidenceSchema, linearScmEvidenceSchema, type DynamicCounterfactualUncertainty, type DynamicInterventionTiming, type DynamicLinearScmEvidence, type LinearScmEvidence } from '@/domain/counterfactual'
import {
  comparisonSurvivalEvidenceSchema,
  flexSurvEvidenceSchema,
  nonparametricSurvivalEvidenceSchema,
  multiStateSurvivalEvidenceSchema,
  parametricSurvivalFamilySchema,
  proportionalHazardsFamilySchema,
  parseComparisonSurvivalEvidence,
  parseFlexSurvEvidence,
  parseNonparametricSurvivalEvidence,
  parseMultiStateSurvivalEvidence,
  type ComparisonSurvivalEvidence,
  type FlexSurvEvidence,
  type NonparametricSurvivalEvidence,
  type MultiStateSurvivalEvidence,
  type ParametricSurvivalFamily,
  type ProportionalHazardsFamily,
} from '@/domain/survival'

/** The groups a double machine learning run averages within, or none for the plain average. */
export const dmlGroupsRequestSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({ kind: z.literal('levels'), column: z.number().int().nonnegative() }).strict(),
  z.object({ kind: z.literal('quantiles'), column: z.number().int().nonnegative(), bins: z.number().int().min(2).max(10) }).strict(),
])
export type DmlGroupsRequest = z.infer<typeof dmlGroupsRequestSchema>
import { assertNever, brand, err, ok, type Brand, type Result } from '@/domain/dop'
import {
  dynotearsEvidenceSchema,
  directLingamEvidenceSchema,
  fciEvidenceSchema,
  pcStableEvidenceSchema,
  lpcmciEvidenceSchema,
  rpcmciEvidenceSchema,
  ocseEvidenceSchema,
  cmlpEvidenceSchema,
  clstmEvidenceSchema,
  cdnotsEvidenceSchema,
  cdnotsPlusEvidenceSchema,
  graceEvidenceSchema,
  jpcmciPlusEvidenceSchema,
  parseDynotearsEvidence,
  parseDirectLingamEvidence,
  parseFciEvidence,
  parsePcStableEvidence,
  parseLpcmciEvidence,
  parseRpcmciEvidence,
  parseOcseEvidence,
  parseCmlpEvidence,
  parseClstmEvidence,
  parseCdnotsResult,
  parseCdnotsPlusResult,
  parseGraceEvidence,
  parseJpcmciPlusEvidence,
  parsePcmciPlusEvidence,
  parseVarLingamEvidence,
  pcmciPlusEvidenceSchema,
  varLingamEvidenceSchema,
  type DynotearsEvidence,
  type DirectLingamEvidence,
  type FciEvidence,
  type PcStableEvidence,
  type ConstraintBackgroundKnowledge,
  type ConstraintCiTest,
  type LpcmciEvidence,
  type RpcmciEvidence,
  type OcseEvidence,
  type CmlpEvidence,
  type ClstmEvidence,
  type CdnotsEvidence,
  type CdnotsPlusEvidence,
  type GraceEvidence,
  type JpcmciPlusEvidence,
  type PcmciPlusEvidence,
  type VarLingamEvidence,
} from '@/domain/discovery'
import {
  backdoorLinearEvidenceSchema,
  frontdoorTwoStageEvidenceSchema,
  instrumentalVariableEvidenceSchema,
  causalEffectsEvidenceSchema,
  causalImpactEvidenceSchema,
  countGlmEvidenceSchema,
  parseBackdoorLinearEvidence,
  parseFrontdoorTwoStageEvidence,
  parseInstrumentalVariableEvidence,
  parseCausalEffectsEvidence,
  parseCausalImpactEvidence,
  parseCountGlmEvidence,
  parseNegativeBinomialIngarchEvidence,
  type BackdoorLinearEvidence,
  type FrontdoorTwoStageEvidence,
  type InstrumentalVariableEvidence,
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

export type TemporalSamples =
  | { readonly kind: 'dense' }
  | {
      readonly kind: 'role-aware'
      readonly validity: Uint8Array
      readonly analysisMask: Uint8Array
      readonly cutOff: 'methodDefault' | 'twoTauMax' | 'tauMax' | 'maxLag' | 'maxLagOrTauMax' | 'twoTauMaxFuture'
      readonly propagateThroughMaxLag: boolean
      readonly maskType: 'none' | 'x' | 'y' | 'z' | 'xy' | 'xz' | 'yz' | 'xyz'
    }

export type MultiStateWorkerInput =
  | { readonly kind: 'preparedRows'; readonly start: number; readonly stop: number; readonly event: number; readonly from: number; readonly to: number }
  | { readonly kind: 'longitudinalStates'; readonly subject: number; readonly time: number; readonly state: number; readonly allowed: readonly (readonly boolean[])[] }
  | {
      readonly kind: 'wideEvents'
      readonly states: readonly (
        | { readonly kind: 'notApplicable' }
        | { readonly kind: 'recorded'; readonly time: number; readonly status: number }
      )[]
      readonly transitions: readonly (readonly (number | null)[])[]
      readonly entry:
        | { readonly kind: 'shared'; readonly state: number; readonly time: number }
        | { readonly kind: 'columns'; readonly state: number; readonly time: number }
    }

export type AnalysisWorkerCommand =
  | {
      readonly kind: 'flexsurv'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly observation:
        | { readonly kind: 'rightCensored'; readonly duration: number; readonly event: number }
        | { readonly kind: 'startStop'; readonly start: number; readonly stop: number; readonly event: number }
      readonly rowFrequency:
        | { readonly kind: 'oneObservationPerRow' }
        | { readonly kind: 'frequencyColumn'; readonly column: number }
      readonly covariates: readonly number[]
      readonly family: ParametricSurvivalFamily
      readonly predictionTimes: readonly number[]
    }
  | {
      readonly kind: 'comparison-survival'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly duration: number
      readonly event: number
      readonly group: number
      readonly truncationTime: number
      readonly permutations: number
      readonly seed: number
    }
  | {
      readonly kind: 'nonparametric-survival'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly duration: number
      readonly event: number
      readonly rowFrequency:
        | { readonly kind: 'oneObservationPerRow' }
        | { readonly kind: 'frequencyColumn'; readonly column: number }
      readonly predictionTimes: readonly number[]
      readonly ties: 'discrete' | 'smoothed'
    }
  | {
      readonly kind: 'multi-state-survival'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly input: MultiStateWorkerInput
      readonly family: ProportionalHazardsFamily
      readonly predictionTimes: readonly number[]
    }
  | {
      readonly kind: 'stationarity-battery'
      readonly request: WorkerRequestId
      readonly values: Float64Array
    }
  | {
      readonly kind: 'multicollinearity'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly correlationThreshold: number
      readonly vifThreshold: number
    }
  | {
      readonly kind: 'pandas-resample-daily'
      readonly request: WorkerRequestId
      /** UTC millisecond timestamps followed by the column-major matrix. */
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly target: 'weekly' | 'monthly'
      readonly incompleteBins: 'keep' | 'drop'
      readonly aggregations: readonly ResamplingAggregation[]
      readonly imputedCells: readonly (readonly [number, number])[]
    }
  | {
      readonly kind: 'pcmci-plus'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly tauMax: number
      readonly pcAlpha: number
      readonly samples: TemporalSamples
    }
  | {
      readonly kind: 'jpcmci-plus'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly datasets: number
      readonly periods: number
      readonly observedColumns: number
      readonly classes: readonly ('system' | 'timeContext' | 'spaceContext')[]
      readonly timeDummy: boolean
      readonly spaceDummy: boolean
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
      readonly samples: TemporalSamples
    }
  | {
      readonly kind: 'rpcmci'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly numRegimes: number
      readonly maxTransitions: number
      readonly switchThres: number
      readonly numIterations: number
      readonly maxAnneal: number
      readonly tauMin: number
      readonly tauMax: number
      readonly pcAlpha: number
      readonly alphaLevel: number
      readonly seed: number
    }
  | {
      readonly kind: 'cdnots'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly validity: Uint8Array
      readonly rows: number
      readonly columns: number
      readonly maxLag: number
      readonly alpha: number
      readonly missing: 'pairwiseComplete' | 'varEm'
      readonly context: 'none' | 'linear' | 'linearSine' | 'linearExponential' | 'linearQuadratic' | 'step' | 'stepLinear'
    }
  | {
      readonly kind: 'cdnots-plus'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly validity: Uint8Array
      readonly rows: number
      readonly columns: number
      readonly maxLag: number
      readonly alpha: number
      readonly missing: 'pairwiseComplete' | 'varEm'
      readonly context: 'none' | 'linear' | 'linearSine' | 'linearExponential' | 'linearQuadratic' | 'step' | 'stepLinear'
    }
  | {
      readonly kind: 'grace'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly validity: Uint8Array
      readonly rows: number
      readonly columns: number
      readonly maxLag: number
      readonly alpha: number
      readonly context: 'none' | 'linear' | 'linearSine' | 'linearExponential' | 'linearQuadratic' | 'step' | 'stepLinear'
      readonly gateThreshold: number
      readonly epochs: number
      readonly patience: number
      readonly seed: number
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
      readonly kind: 'pc-stable'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly names: readonly string[]
      readonly alpha: number
      readonly maxDepth: number | null
      readonly ciTest: ConstraintCiTest
      readonly background: ConstraintBackgroundKnowledge
    }
  | {
      readonly kind: 'fci'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly names: readonly string[]
      readonly alpha: number
      readonly maxDepth: number | null
      readonly maxPathLength: number | null
      readonly ciTest: ConstraintCiTest
      readonly background: ConstraintBackgroundKnowledge
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
      readonly kind: 'cmlp'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly lag: number
      readonly hidden: readonly number[]
      readonly activation: 'sigmoid' | 'tanh' | 'relu' | 'leakyRelu' | 'identity'
      readonly penalty: 'groupLasso' | 'groupSparseGroupLasso' | 'hierarchical'
      readonly lambda: number
      readonly ridgeLambda: number
      readonly learningRate: number
      readonly maxIter: number
      readonly checkEvery: number
      readonly lookback: number
      readonly seed: number
    }
  | {
      readonly kind: 'clstm'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly context: number
      readonly hidden: number
      readonly lambda: number
      readonly ridgeLambda: number
      readonly learningRate: number
      readonly maxIter: number
      readonly checkEvery: number
      readonly lookback: number
      readonly seed: number
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
      readonly kind: 'instrumental-variable'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly instruments: readonly number[]
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
      readonly kind: 'negative-binomial-ingarch'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly outcome: number
      readonly link: 'identity' | 'log'
      readonly regressors: readonly number[]
      readonly pastObservationLags: readonly number[]
      readonly pastMeanLags: readonly number[]
      readonly externalRegressors: readonly boolean[]
      readonly horizon: number
      readonly baselineRegressors: readonly number[]
      readonly interventionRegressor: number
      readonly controlValue: number
      readonly treatmentValue: number
      readonly schedule: IngarchInterventionSchedule
    }
  | {
      readonly kind: 'count-series-intervention-scan'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly outcome: number
      readonly link: 'identity' | 'log'
      readonly pastObservationLags: readonly number[]
      readonly pastMeanLags: readonly number[]
      readonly candidateReferencePoints: readonly number[]
      readonly delta: number
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
      readonly estimator: TotalEffectEstimator
      readonly interventions: readonly [number, number]
      readonly uncertainty: CausalEffectsUncertainty
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
      readonly correlationMaxLag: number
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
      readonly groups: DmlGroupsRequest
    }
  | {
      readonly kind: 't-learner'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly treatment: number
      readonly outcome: number
      readonly adjustment: readonly number[]
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
      readonly crossFitFolds: number
      readonly alpha: number
    }
  | {
      readonly kind: 'panel-intervention'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly units: readonly string[]
      readonly times: readonly number[]
      readonly placeboReplications: number
      readonly seed: number
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
      readonly kind: 'identified-discrete-query'
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
      readonly bins: number
      readonly condition: { readonly variable: number; readonly state: number } | null
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
      readonly kind: 'dynamic-linear-scm-counterfactual'
      readonly request: WorkerRequestId
      readonly values: Float64Array
      readonly rows: number
      readonly columns: number
      readonly nodes: readonly number[]
      readonly statLag: number
      readonly graph: readonly (readonly (readonly string[])[])[]
      readonly treatment: number
      readonly outcome: number
      readonly timing: DynamicInterventionTiming
      readonly steps: number
      readonly interventions: readonly [number, number]
      readonly uncertainty: DynamicCounterfactualUncertainty
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
  | { readonly kind: 'analysis-cancelled'; readonly detail: string }
  | DiscreteStateRefusal

export type DiscreteStateRefusal =
  | {
      readonly kind: 'discreteStateRefused'
      readonly query: 'bayesianNetwork' | 'identifiedExpression'
      readonly node: number
      readonly name: string
      readonly problem:
        | { readonly kind: 'noFiniteObservations'; readonly observations: number }
        | { readonly kind: 'singleObservedState'; readonly value: number; readonly observations: number }
        | {
            readonly kind: 'quantileCollapse'
            readonly distinctValues: number
            readonly requestedStates: number
            readonly populatedStates: number
          }
    }

export const describeAnalysisWorkerProblem = (problem: AnalysisWorkerProblem): string => {
  switch (problem.kind) {
    case 'kernel-refused':
    case 'wasm-unavailable':
    case 'worker-unavailable':
    case 'worker-protocol-failed':
    case 'analysis-cancelled':
      return problem.detail
    case 'discreteStateRefused': {
      switch (problem.problem.kind) {
        case 'noFiniteObservations':
          return `${problem.name} has no finite observations in the prepared data. Resolve or remove its missing values before running this discrete query.`
        case 'singleObservedState':
          return `${problem.name} takes only one value (${problem.problem.value}) across ${problem.problem.observations} prepared rows. This query requires at least two observed states.`
        case 'quantileCollapse':
          return `${problem.name} has ${problem.problem.distinctValues} distinct values, but a state budget of ${problem.problem.requestedStates} produced only ${problem.problem.populatedStates} populated quantile state. Tied values made the quantile cut points coincide; change the preparation or use a method that does not require discrete states.`
        default:
          return assertNever(problem.problem)
      }
    }
    default:
      return assertNever(problem)
  }
}

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
  | { readonly kind: 'multicollinearity-succeeded'; readonly request: WorkerRequestId; readonly result: MulticollinearityEvidence }
  | { readonly kind: 'pandas-resampling-succeeded'; readonly request: WorkerRequestId; readonly result: PandasResamplingEvidence }
  | { readonly kind: 'flexsurv-succeeded'; readonly request: WorkerRequestId; readonly result: FlexSurvEvidence }
  | { readonly kind: 'nonparametric-survival-succeeded'; readonly request: WorkerRequestId; readonly result: NonparametricSurvivalEvidence }
  | { readonly kind: 'comparison-survival-succeeded'; readonly request: WorkerRequestId; readonly result: ComparisonSurvivalEvidence }
  | { readonly kind: 'multi-state-survival-succeeded'; readonly request: WorkerRequestId; readonly result: MultiStateSurvivalEvidence }
  | {
      readonly kind: 'pcmci-plus-succeeded'
      readonly request: WorkerRequestId
      readonly result: PcmciPlusEvidence
    }
  | {
      readonly kind: 'jpcmci-plus-succeeded'
      readonly request: WorkerRequestId
      readonly result: JpcmciPlusEvidence
    }
  | {
      readonly kind: 'lpcmci-succeeded'
      readonly request: WorkerRequestId
      readonly result: LpcmciEvidence
    }
  | {
      readonly kind: 'rpcmci-succeeded'
      readonly request: WorkerRequestId
      readonly result: RpcmciEvidence
    }
  | { readonly kind: 'cdnots-succeeded'; readonly request: WorkerRequestId; readonly result: CdnotsEvidence }
  | { readonly kind: 'cdnots-plus-succeeded'; readonly request: WorkerRequestId; readonly result: CdnotsPlusEvidence }
  | { readonly kind: 'grace-succeeded'; readonly request: WorkerRequestId; readonly result: GraceEvidence }
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
  | { readonly kind: 'pc-stable-succeeded'; readonly request: WorkerRequestId; readonly result: PcStableEvidence }
  | { readonly kind: 'fci-succeeded'; readonly request: WorkerRequestId; readonly result: FciEvidence }
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
  | { readonly kind: 'cmlp-succeeded'; readonly request: WorkerRequestId; readonly result: CmlpEvidence }
  | { readonly kind: 'clstm-succeeded'; readonly request: WorkerRequestId; readonly result: ClstmEvidence }
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
  | { readonly kind: 'instrumental-variable-succeeded'; readonly request: WorkerRequestId; readonly result: InstrumentalVariableEvidence }
  | { readonly kind: 'count-glm-succeeded'; readonly request: WorkerRequestId; readonly result: CountGlmEvidence }
  | { readonly kind: 'negative-binomial-ingarch-succeeded'; readonly request: WorkerRequestId; readonly result: NegativeBinomialIngarchEvidence }
  | { readonly kind: 'count-series-intervention-scan-succeeded'; readonly request: WorkerRequestId; readonly result: CountSeriesInterventionScanEvidence }
  | { readonly kind: 'causal-effects-succeeded'; readonly request: WorkerRequestId; readonly result: CausalEffectsEvidence }
  | { readonly kind: 'causal-impact-succeeded'; readonly request: WorkerRequestId; readonly result: CausalImpactEvidence }
  | { readonly kind: 'linear-refutation-succeeded'; readonly request: WorkerRequestId; readonly result: LinearRefutationEvidence }
  | { readonly kind: 'unobserved-confounding-succeeded'; readonly request: WorkerRequestId; readonly result: UnobservedConfoundingEvidence }
  | { readonly kind: 'series-structure-succeeded'; readonly request: WorkerRequestId; readonly result: SeriesStructureEvidence }
  | { readonly kind: 'seasonal-adjusted'; readonly request: WorkerRequestId; readonly result: SeasonalAdjustedEvidence }
  | { readonly kind: 'double-ml-succeeded'; readonly request: WorkerRequestId; readonly result: DoubleMlEvidence }
  | { readonly kind: 't-learner-succeeded'; readonly request: WorkerRequestId; readonly result: TLearnerEvidence }
  | { readonly kind: 'ardl-succeeded'; readonly request: WorkerRequestId; readonly result: ArdlEvidence }
  | { readonly kind: 'vecm-succeeded'; readonly request: WorkerRequestId; readonly result: VecmEvidence }
  | { readonly kind: 'synthetic-control-succeeded'; readonly request: WorkerRequestId; readonly result: SyntheticControlEvidence }
  | { readonly kind: 'panel-intervention-succeeded'; readonly request: WorkerRequestId; readonly result: PanelInterventionEvidence }
  | { readonly kind: 'negbin-nuts-succeeded'; readonly request: WorkerRequestId; readonly result: NegbinNutsEvidence }
  | { readonly kind: 'bayesian-gaussian-succeeded'; readonly request: WorkerRequestId; readonly result: BayesianGaussianEvidence }
  | { readonly kind: 'discrete-bn-succeeded'; readonly request: WorkerRequestId; readonly result: DiscreteBnEvidence }
  | { readonly kind: 'identified-discrete-query-succeeded'; readonly request: WorkerRequestId; readonly result: IdentifiedDiscreteQueryEvidence }
  | { readonly kind: 'binary-ett-succeeded'; readonly request: WorkerRequestId; readonly result: BinaryEttEvidence }
  | { readonly kind: 'linear-scm-succeeded'; readonly request: WorkerRequestId; readonly result: LinearScmEvidence }
  | { readonly kind: 'dynamic-linear-scm-succeeded'; readonly request: WorkerRequestId; readonly result: DynamicLinearScmEvidence }
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

const temporalSamplesSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('dense') }).strict(),
  z.object({
    kind: z.literal('role-aware'),
    validity: z.instanceof(Uint8Array),
    analysisMask: z.instanceof(Uint8Array),
    cutOff: z.enum(['methodDefault', 'twoTauMax', 'tauMax', 'maxLag', 'maxLagOrTauMax', 'twoTauMaxFuture']),
    propagateThroughMaxLag: z.boolean(),
    maskType: z.enum(['none', 'x', 'y', 'z', 'xy', 'xz', 'yz', 'xyz']),
  }).strict(),
])

const cdnotsContextCommandSchema = z.enum(['none', 'linear', 'linearSine', 'linearExponential', 'linearQuadratic', 'step', 'stepLinear'])

const cdnotsCommandBaseSchema = z.object({
  request: requestSchema,
  values: z.instanceof(Float64Array),
  validity: z.instanceof(Uint8Array),
  rows: z.number().int().positive(),
  columns: z.number().int().min(2).max(32),
  maxLag: z.number().int().min(1).max(20),
  alpha: z.number().finite().positive().max(1),
  missing: z.enum(['pairwiseComplete', 'varEm']),
  context: cdnotsContextCommandSchema,
})

const constraintPairSchema = z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])
const constraintPatternSchema = z.tuple([z.string(), z.string()])
const constraintBackgroundSchema = z.object({
  forbidden: z.array(constraintPairSchema),
  required: z.array(constraintPairSchema),
  forbiddenPatterns: z.array(constraintPatternSchema),
  requiredPatterns: z.array(constraintPatternSchema),
  tiers: z.array(z.number().int().nonnegative().nullable()),
  forbiddenWithinTiers: z.array(z.number().int().nonnegative()),
}).strict()

const constraintCommandBaseSchema = z.object({
  request: requestSchema,
  values: z.instanceof(Float64Array),
  rows: z.number().int().positive(),
  columns: z.number().int().min(2).max(32),
  names: z.array(z.string().trim().min(1)).min(2).max(32),
  alpha: z.number().finite().positive().max(1),
  maxDepth: z.number().int().nonnegative().nullable(),
  ciTest: z.enum(['fisherZ', 'kci']),
  background: constraintBackgroundSchema,
})

const commandSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('flexsurv'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(2),
    columns: z.number().int().min(2).max(256),
    observation: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('rightCensored'), duration: z.number().int().nonnegative(), event: z.number().int().nonnegative() }).strict(),
      z.object({ kind: z.literal('startStop'), start: z.number().int().nonnegative(), stop: z.number().int().nonnegative(), event: z.number().int().nonnegative() }).strict(),
    ]),
    rowFrequency: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('oneObservationPerRow') }).strict(),
      z.object({ kind: z.literal('frequencyColumn'), column: z.number().int().nonnegative() }).strict(),
    ]),
    covariates: z.array(z.number().int().nonnegative()),
    family: parametricSurvivalFamilySchema,
    predictionTimes: z.array(z.number().finite().nonnegative()).min(1).max(500),
  }).strict().superRefine((value, context) => {
    const roles = value.observation.kind === 'rightCensored'
      ? [value.observation.duration, value.observation.event]
      : [value.observation.start, value.observation.stop, value.observation.event]
    const frequency = value.rowFrequency.kind === 'frequencyColumn' ? [value.rowFrequency.column] : []
    const selected = [...roles, ...frequency, ...value.covariates]
    if (new Set(roles).size !== roles.length || new Set(selected).size !== selected.length || selected.some((column) => column >= value.columns)) {
      context.addIssue({ code: 'custom', message: 'Survival roles and covariates must be distinct columns inside the matrix.' })
    }
    if (value.observation.kind === 'startStop' && !proportionalHazardsFamilySchema.safeParse(value.family).success) {
      context.addIssue({ code: 'custom', message: 'Start-stop data requires a proportional-hazards family.' })
    }
  }),
  z.object({
    kind: z.literal('nonparametric-survival'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(2),
    columns: z.number().int().min(2).max(3),
    duration: z.number().int().nonnegative(),
    event: z.number().int().nonnegative(),
    rowFrequency: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('oneObservationPerRow') }).strict(),
      z.object({ kind: z.literal('frequencyColumn'), column: z.number().int().nonnegative() }).strict(),
    ]),
    predictionTimes: z.array(z.number().finite().nonnegative()).min(1).max(500),
    ties: z.enum(['discrete', 'smoothed']),
  }).strict().superRefine((value, context) => {
    const frequency = value.rowFrequency.kind === 'frequencyColumn' ? [value.rowFrequency.column] : []
    const roles = [value.duration, value.event, ...frequency]
    if (new Set(roles).size !== roles.length || roles.some((column) => column >= value.columns)) {
      context.addIssue({ code: 'custom', message: 'Nonparametric survival roles must be distinct columns inside the matrix.' })
    }
  }),
  z.object({
    kind: z.literal('comparison-survival'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(4),
    columns: z.number().int().min(3).max(256),
    duration: z.number().int().nonnegative(),
    event: z.number().int().nonnegative(),
    group: z.number().int().nonnegative(),
    truncationTime: z.number().finite().positive(),
    permutations: z.number().int().min(1).max(100_000),
    seed: z.number().int().nonnegative(),
  }).strict().superRefine((value, context) => {
    const selected = [value.duration, value.event, value.group]
    if (new Set(selected).size !== selected.length || selected.some((column) => column >= value.columns)) {
      context.addIssue({ code: 'custom', message: 'Duration, event, and group must be three distinct columns inside the matrix.' })
    }
  }),
  z.object({
    kind: z.literal('multi-state-survival'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(2),
    columns: z.number().int().min(2).max(256),
    input: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('preparedRows'), start: z.number().int().nonnegative(), stop: z.number().int().nonnegative(), event: z.number().int().nonnegative(), from: z.number().int().nonnegative(), to: z.number().int().nonnegative() }).strict(),
      z.object({ kind: z.literal('longitudinalStates'), subject: z.number().int().nonnegative(), time: z.number().int().nonnegative(), state: z.number().int().nonnegative(), allowed: z.array(z.array(z.boolean()).min(1)).min(2) }).strict(),
      z.object({
        kind: z.literal('wideEvents'),
        states: z.array(z.discriminatedUnion('kind', [
          z.object({ kind: z.literal('notApplicable') }).strict(),
          z.object({ kind: z.literal('recorded'), time: z.number().int().nonnegative(), status: z.number().int().nonnegative() }).strict(),
        ])).min(2),
        transitions: z.array(z.array(z.number().int().positive().nullable()).min(1)).min(2),
        entry: z.discriminatedUnion('kind', [
          z.object({ kind: z.literal('shared'), state: z.number().int().positive(), time: z.number().finite() }).strict(),
          z.object({ kind: z.literal('columns'), state: z.number().int().nonnegative(), time: z.number().int().nonnegative() }).strict(),
        ]),
      }).strict(),
    ]),
    family: proportionalHazardsFamilySchema,
    predictionTimes: z.array(z.number().finite().nonnegative()).min(1).max(500),
  }).strict().superRefine((value, context) => {
    const input = value.input
    const selected = input.kind === 'preparedRows'
      ? [input.start, input.stop, input.event, input.from, input.to]
      : input.kind === 'longitudinalStates'
        ? [input.subject, input.time, input.state]
        : [
            ...input.states.flatMap((state) => state.kind === 'recorded' ? [state.time, state.status] : []),
            ...(input.entry.kind === 'columns' ? [input.entry.state, input.entry.time] : []),
          ]
    const requiresDistinctRoles = input.kind !== 'wideEvents'
    if ((requiresDistinctRoles && new Set(selected).size !== selected.length) || selected.some((column) => column >= value.columns)) {
      context.addIssue({ code: 'custom', message: 'Multi-state input roles must be valid columns, and prepared or longitudinal roles must be distinct.' })
    }
    if (input.kind === 'wideEvents' && input.states.some((state) => state.kind === 'recorded' && state.time === state.status)) {
      context.addIssue({ code: 'custom', message: 'Each wide state needs different time and status columns.' })
    }
    if (input.kind === 'longitudinalStates' && input.allowed.some((row) => row.length !== input.allowed.length)) {
      context.addIssue({ code: 'custom', message: 'The allowed-transition matrix must be square.' })
    }
    if (input.kind === 'wideEvents' && (input.transitions.length !== input.states.length || input.transitions.some((row) => row.length !== input.states.length))) {
      context.addIssue({ code: 'custom', message: 'The numbered transition matrix must be square and match the number of states.' })
    }
    if (value.predictionTimes.some((time, index) => index > 0 && value.predictionTimes[index - 1]! > time)) {
      context.addIssue({ code: 'custom', message: 'Multi-state prediction times must be ordered.' })
    }
  }),
  z.object({
    kind: z.literal('stationarity-battery'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
  }).strict(),
  z.object({
    kind: z.literal('multicollinearity'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().min(3),
    columns: z.number().int().min(2).max(64),
    correlationThreshold: z.number().finite().gt(0).max(1),
    vifThreshold: z.number().finite().gt(1),
  }).strict(),
  z.object({
    kind: z.literal('pandas-resample-daily'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(1).max(256),
    target: z.enum(['weekly', 'monthly']),
    incompleteBins: z.enum(['keep', 'drop']),
    aggregations: z.array(z.enum(['mean', 'sum', 'median', 'minimum', 'maximum', 'first', 'last'])).min(1).max(256),
    imputedCells: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
  }).strict(),
  z.object({
    kind: z.literal('pcmci-plus'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(32),
    tauMax: z.number().int().min(1).max(20),
    pcAlpha: z.number().finite().positive().max(1),
    samples: temporalSamplesSchema,
  }).strict(),
  z.object({
    kind: z.literal('jpcmci-plus'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    datasets: z.number().int().min(2),
    periods: z.number().int().min(2),
    observedColumns: z.number().int().min(2).max(32),
    classes: z.array(z.enum(['system', 'timeContext', 'spaceContext'])).min(2).max(32),
    timeDummy: z.boolean(),
    spaceDummy: z.boolean(),
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
    samples: temporalSamplesSchema,
  }).strict(),
  z.object({
    kind: z.literal('rpcmci'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    numRegimes: z.number().int().min(2).max(6),
    maxTransitions: z.number().int().nonnegative(),
    switchThres: z.number().finite().min(0).max(1),
    numIterations: z.number().int().min(1).max(100),
    maxAnneal: z.number().int().min(1).max(50),
    tauMin: z.number().int().nonnegative().max(6),
    tauMax: z.number().int().nonnegative().max(6),
    pcAlpha: z.number().finite().positive().max(1),
    alphaLevel: z.number().finite().positive().max(1),
    seed: z.number().int().nonnegative(),
  }).strict(),
  cdnotsCommandBaseSchema.extend({ kind: z.literal('cdnots') }).strict(),
  cdnotsCommandBaseSchema.extend({ kind: z.literal('cdnots-plus') }).strict(),
  z.object({
    kind: z.literal('grace'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    validity: z.instanceof(Uint8Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(32),
    maxLag: z.number().int().min(1).max(20),
    alpha: z.number().finite().positive().max(1),
    context: cdnotsContextCommandSchema,
    gateThreshold: z.number().finite().min(0).max(1),
    epochs: z.number().int().min(1).max(2_000),
    patience: z.number().int().min(1).max(500),
    seed: z.number().int().nonnegative(),
  }).strict().refine((value) => value.patience <= value.epochs, { message: 'GRACE patience cannot exceed epochs.' }),
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
  constraintCommandBaseSchema.extend({ kind: z.literal('pc-stable') }).strict(),
  constraintCommandBaseSchema.extend({
    kind: z.literal('fci'),
    maxPathLength: z.number().int().nonnegative().nullable(),
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
    kind: z.literal('cmlp'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    lag: z.number().int().min(1).max(20),
    hidden: z.array(z.number().int().min(1).max(256)).min(1).max(4),
    activation: z.enum(['sigmoid', 'tanh', 'relu', 'leakyRelu', 'identity']),
    penalty: z.enum(['groupLasso', 'groupSparseGroupLasso', 'hierarchical']),
    lambda: z.number().finite().nonnegative(),
    ridgeLambda: z.number().finite().nonnegative(),
    learningRate: z.number().finite().positive(),
    maxIter: z.number().int().min(1).max(50_000),
    checkEvery: z.number().int().positive(),
    lookback: z.number().int().positive(),
    seed: z.number().int().nonnegative(),
  }).strict().refine((value) => value.checkEvery <= value.maxIter, { message: 'cMLP checkEvery cannot exceed maxIter.' }),
  z.object({
    kind: z.literal('clstm'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(12),
    context: z.number().int().min(1).max(100),
    hidden: z.number().int().min(1).max(256),
    lambda: z.number().finite().nonnegative(),
    ridgeLambda: z.number().finite().nonnegative(),
    learningRate: z.number().finite().positive(),
    maxIter: z.number().int().min(1).max(20_000),
    checkEvery: z.number().int().positive(),
    lookback: z.number().int().positive(),
    seed: z.number().int().nonnegative(),
  }).strict().refine((value) => value.checkEvery <= value.maxIter, { message: 'cLSTM checkEvery cannot exceed maxIter.' }),
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
    kind: z.literal('instrumental-variable'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(3).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    instruments: z.array(z.number().int().nonnegative()).min(1),
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
    kind: z.literal('negative-binomial-ingarch'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    outcome: z.number().int().nonnegative(),
    link: z.enum(['identity', 'log']),
    regressors: z.array(z.number().int().nonnegative()).min(1),
    pastObservationLags: z.array(z.number().int().min(1).max(24)).min(1),
    pastMeanLags: z.array(z.number().int().min(1).max(24)).min(1),
    externalRegressors: z.array(z.boolean()),
    horizon: z.number().int().min(1).max(240),
    baselineRegressors: z.array(z.number().finite()).min(1),
    interventionRegressor: z.number().int().nonnegative(),
    controlValue: z.number().finite(),
    treatmentValue: z.number().finite(),
    schedule: ingarchInterventionScheduleSchema,
  }).strict().superRefine((value, context) => {
    if (value.link !== 'identity') return
    if (value.controlValue < 0 || value.treatmentValue < 0 || value.baselineRegressors.some((entry) => entry < 0)) {
      context.addIssue({ code: 'custom', message: 'Identity-link INGARCH requires non-negative control, treatment, and future regressor values.' })
    }
    for (const column of value.regressors) {
      for (let row = 0; row < value.rows; row += 1) {
        if ((value.values[column * value.rows + row] ?? Number.NaN) < 0) {
          context.addIssue({ code: 'custom', message: 'Identity-link INGARCH requires non-negative historical regressors.' })
          return
        }
      }
    }
  }),
  z.object({
    kind: z.literal('count-series-intervention-scan'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().positive().max(64),
    outcome: z.number().int().nonnegative(),
    link: z.enum(['identity', 'log']),
    pastObservationLags: z.array(z.number().int().min(1).max(24)).min(1),
    pastMeanLags: z.array(z.number().int().min(1).max(24)).min(1),
    candidateReferencePoints: z.array(z.number().int().nonnegative()).min(1),
    delta: z.number().finite().min(0).max(1),
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
    estimator: totalEffectEstimatorSchema,
    interventions: z.tuple([z.number().finite(), z.number().finite()]),
    uncertainty: causalEffectsUncertaintySchema,
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
    correlationMaxLag: z.number().int().min(1).max(400),
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
    groups: dmlGroupsRequestSchema,
  }).strict(),
  z.object({
    kind: z.literal('t-learner'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(3).max(64),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    adjustment: z.array(z.number().int().nonnegative()).min(1),
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
    crossFitFolds: z.number().int().min(2).max(20),
    alpha: z.number().gt(0).lt(1),
  }).strict(),
  z.object({
    kind: z.literal('panel-intervention'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    units: z.array(z.string().min(1)).min(1),
    times: z.array(z.number().int().nonnegative()).min(1),
    placeboReplications: z.number().int().min(2).max(2000),
    seed: z.number().int().min(0).max(0xffff_ffff),
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
    kind: z.literal('identified-discrete-query'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    observedNodes: z.array(z.number().int().nonnegative()).min(2),
    names: z.array(z.string().trim().min(1)).min(2),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    unobserved: z.array(z.number().int().nonnegative()),
    bins: z.number().int().min(2).max(10),
    condition: z.object({ variable: z.number().int().nonnegative(), state: z.number().int().nonnegative() }).strict().nullable(),
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
    kind: z.literal('dynamic-linear-scm-counterfactual'),
    request: requestSchema,
    values: z.instanceof(Float64Array),
    rows: z.number().int().positive(),
    columns: z.number().int().min(2).max(64),
    nodes: z.array(z.number().int().nonnegative()).min(2),
    statLag: z.number().int().min(0).max(20),
    graph: z.array(z.array(z.array(z.string().max(3)))),
    treatment: z.number().int().nonnegative(),
    outcome: z.number().int().nonnegative(),
    timing: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('point'), time: z.number().int().nonnegative() }).strict(),
      z.object({ kind: z.literal('persistent'), start: z.number().int().nonnegative() }).strict(),
    ]),
    steps: z.number().int().positive(),
    interventions: z.tuple([z.number().finite(), z.number().finite()]),
    uncertainty: dynamicCounterfactualUncertaintySchema,
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

const discreteStateRefusalSchema = z.object({
  kind: z.literal('discreteStateRefused'),
  query: z.enum(['bayesianNetwork', 'identifiedExpression']),
  node: z.number().int().nonnegative(),
  name: z.string().trim().min(1),
  problem: z.discriminatedUnion('kind', [
    z.object({
      kind: z.literal('noFiniteObservations'),
      observations: z.number().int().nonnegative(),
    }).strict(),
    z.object({
      kind: z.literal('singleObservedState'),
      value: z.number().finite(),
      observations: z.number().int().positive(),
    }).strict(),
    z.object({
      kind: z.literal('quantileCollapse'),
      distinctValues: z.number().int().min(2),
      requestedStates: z.number().int().min(2),
      populatedStates: z.number().int().max(1),
    }).strict(),
  ]),
}).strict()

const workerProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('kernel-refused'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('wasm-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-unavailable'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('worker-protocol-failed'), detail: z.string() }).strict(),
  z.object({ kind: z.literal('analysis-cancelled'), detail: z.string() }).strict(),
  discreteStateRefusalSchema,
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
  z.object({ kind: z.literal('multicollinearity-succeeded'), request: requestSchema, result: multicollinearityEvidenceSchema }).strict(),
  z.object({ kind: z.literal('flexsurv-succeeded'), request: requestSchema, result: flexSurvEvidenceSchema }).strict(),
  z.object({ kind: z.literal('nonparametric-survival-succeeded'), request: requestSchema, result: nonparametricSurvivalEvidenceSchema }).strict(),
  z.object({ kind: z.literal('comparison-survival-succeeded'), request: requestSchema, result: comparisonSurvivalEvidenceSchema }).strict(),
  z.object({ kind: z.literal('multi-state-survival-succeeded'), request: requestSchema, result: multiStateSurvivalEvidenceSchema }).strict(),
  z.object({
    kind: z.literal('pandas-resampling-succeeded'),
    request: requestSchema,
    result: pandasResamplingEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('pcmci-plus-succeeded'),
    request: requestSchema,
    result: pcmciPlusEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('jpcmci-plus-succeeded'),
    request: requestSchema,
    result: jpcmciPlusEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('lpcmci-succeeded'),
    request: requestSchema,
    result: lpcmciEvidenceSchema,
  }).strict(),
  z.object({
    kind: z.literal('rpcmci-succeeded'),
    request: requestSchema,
    result: rpcmciEvidenceSchema,
  }).strict(),
  z.object({ kind: z.literal('cdnots-succeeded'), request: requestSchema, result: cdnotsEvidenceSchema }).strict(),
  z.object({ kind: z.literal('cdnots-plus-succeeded'), request: requestSchema, result: cdnotsPlusEvidenceSchema }).strict(),
  z.object({ kind: z.literal('grace-succeeded'), request: requestSchema, result: graceEvidenceSchema }).strict(),
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
  z.object({ kind: z.literal('pc-stable-succeeded'), request: requestSchema, result: pcStableEvidenceSchema }).strict(),
  z.object({ kind: z.literal('fci-succeeded'), request: requestSchema, result: fciEvidenceSchema }).strict(),
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
  z.object({ kind: z.literal('cmlp-succeeded'), request: requestSchema, result: cmlpEvidenceSchema }).strict(),
  z.object({ kind: z.literal('clstm-succeeded'), request: requestSchema, result: clstmEvidenceSchema }).strict(),
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
  z.object({ kind: z.literal('instrumental-variable-succeeded'), request: requestSchema, result: instrumentalVariableEvidenceSchema }).strict(),
  z.object({ kind: z.literal('count-glm-succeeded'), request: requestSchema, result: countGlmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('negative-binomial-ingarch-succeeded'), request: requestSchema, result: negativeBinomialIngarchEvidenceSchema }).strict(),
  z.object({ kind: z.literal('count-series-intervention-scan-succeeded'), request: requestSchema, result: countSeriesInterventionScanEvidenceSchema }).strict(),
  z.object({ kind: z.literal('causal-effects-succeeded'), request: requestSchema, result: causalEffectsEvidenceSchema }).strict(),
  z.object({ kind: z.literal('causal-impact-succeeded'), request: requestSchema, result: causalImpactEvidenceSchema }).strict(),
  z.object({ kind: z.literal('linear-refutation-succeeded'), request: requestSchema, result: linearRefutationEvidenceSchema }).strict(),
  z.object({ kind: z.literal('unobserved-confounding-succeeded'), request: requestSchema, result: unobservedConfoundingEvidenceSchema }).strict(),
  z.object({ kind: z.literal('series-structure-succeeded'), request: requestSchema, result: seriesStructureEvidenceSchema }).strict(),
  z.object({ kind: z.literal('seasonal-adjusted'), request: requestSchema, result: seasonalAdjustedEvidenceSchema }).strict(),
  z.object({ kind: z.literal('double-ml-succeeded'), request: requestSchema, result: doubleMlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('t-learner-succeeded'), request: requestSchema, result: tLearnerEvidenceSchema }).strict(),
  z.object({ kind: z.literal('ardl-succeeded'), request: requestSchema, result: ardlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('vecm-succeeded'), request: requestSchema, result: vecmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('synthetic-control-succeeded'), request: requestSchema, result: syntheticControlEvidenceSchema }).strict(),
  z.object({ kind: z.literal('panel-intervention-succeeded'), request: requestSchema, result: panelInterventionEvidenceSchema }).strict(),
  z.object({ kind: z.literal('negbin-nuts-succeeded'), request: requestSchema, result: negbinNutsEvidenceSchema }).strict(),
  z.object({ kind: z.literal('bayesian-gaussian-succeeded'), request: requestSchema, result: bayesianGaussianEvidenceSchema }).strict(),
  z.object({ kind: z.literal('discrete-bn-succeeded'), request: requestSchema, result: discreteBnEvidenceSchema }).strict(),
  z.object({ kind: z.literal('identified-discrete-query-succeeded'), request: requestSchema, result: identifiedDiscreteQueryEvidenceSchema }).strict(),
  z.object({ kind: z.literal('binary-ett-succeeded'), request: requestSchema, result: binaryEttEvidenceSchema }).strict(),
  z.object({ kind: z.literal('linear-scm-succeeded'), request: requestSchema, result: linearScmEvidenceSchema }).strict(),
  z.object({ kind: z.literal('dynamic-linear-scm-succeeded'), request: requestSchema, result: dynamicLinearScmEvidenceSchema }).strict(),
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
  if (parsed.data.kind === 'pandas-resample-daily') {
    const { rows, columns } = parsed.data
    if (parsed.data.values.length !== rows * (columns + 1)
      || parsed.data.aggregations.length !== columns
      || parsed.data.imputedCells.some(([row, column]) => row >= rows || column >= columns)) {
      return err({ kind: 'invalid-command', detail: 'The resampling timestamps, matrix, aggregation rules, or imputation evidence do not share one shape.' })
    }
  }
  if (parsed.data.kind !== 'pandas-resample-daily' && 'columns' in parsed.data && parsed.data.values.length !== parsed.data.rows * parsed.data.columns) {
    return err({ kind: 'invalid-command', detail: 'The multivariate matrix dimensions do not match its numeric buffer.' })
  }
  if (parsed.data.kind === 'jpcmci-plus' && (
    parsed.data.rows !== parsed.data.datasets * parsed.data.periods
    || parsed.data.values.length !== parsed.data.rows * parsed.data.observedColumns
    || parsed.data.classes.length !== parsed.data.observedColumns
    || parsed.data.classes.filter((role) => role === 'system').length < 2
  )) {
    return err({ kind: 'invalid-command', detail: 'J-PCMCI+ panel dimensions and observed-variable roles must describe one balanced joint matrix.' })
  }
  if ((parsed.data.kind === 'pcmci-plus' || parsed.data.kind === 'lpcmci') && parsed.data.samples.kind === 'role-aware') {
    const cells = parsed.data.rows * parsed.data.columns
    if (parsed.data.samples.validity.length !== cells || parsed.data.samples.analysisMask.length !== cells) {
      return err({ kind: 'invalid-command', detail: 'Role-aware validity and analysis-mask bytes must match the time-series matrix.' })
    }
    if (parsed.data.samples.validity.some((value) => value > 1) || parsed.data.samples.analysisMask.some((value) => value > 1)) {
      return err({ kind: 'invalid-command', detail: 'Role-aware validity and analysis-mask cells must be encoded as 0 or 1.' })
    }
  }
  if (parsed.data.kind === 'cdnots' || parsed.data.kind === 'cdnots-plus' || parsed.data.kind === 'grace') {
    const cells = parsed.data.rows * parsed.data.columns
    if (parsed.data.validity.length !== cells) {
      return err({ kind: 'invalid-command', detail: 'Causal-TS validity bytes must match the time-series matrix.' })
    }
    if (parsed.data.validity.some((value) => value > 1)) {
      return err({ kind: 'invalid-command', detail: 'Causal-TS validity cells must be encoded as 0 or 1.' })
    }
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

export function parseAnalysisRefusal(
  value: unknown,
): Result<DiscreteStateRefusal | null, AnalysisProtocolProblem> {
  if (typeof value !== 'object' || value === null || !('kind' in value) || value.kind !== 'discreteStateRefused') {
    return ok(null)
  }
  const parsed = discreteStateRefusalSchema.safeParse(value)
  return parsed.success
    ? ok(parsed.data)
    : err({ kind: 'invalid-event', detail: z.prettifyError(parsed.error) })
}

export function parseAnalysisWorkerEvent(value: unknown): Result<AnalysisWorkerEvent, AnalysisProtocolProblem> {
  const parsed = eventSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-event', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'protocol-failed') return ok(parsed.data)
  const request = workerRequestId(parsed.data.request)
  if (!request.ok) return err({ kind: 'invalid-event', detail: 'The worker request identity is invalid.' })
  if (parsed.data.kind === 'analysis-failed') return ok({ ...parsed.data, request: request.value })
  if (parsed.data.kind === 'analysis-progress') return ok({ ...parsed.data, request: request.value })
  if (parsed.data.kind === 'flexsurv-succeeded') {
    const result = parseFlexSurvEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'flexsurv-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'nonparametric-survival-succeeded') {
    const result = parseNonparametricSurvivalEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'nonparametric-survival-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'comparison-survival-succeeded') {
    const result = parseComparisonSurvivalEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'comparison-survival-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'multi-state-survival-succeeded') {
    const result = parseMultiStateSurvivalEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'multi-state-survival-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'multicollinearity-succeeded') {
    const result = parseMulticollinearityEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'multicollinearity-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.kind === 'invalid-evidence' ? result.error.detail : 'The multicollinearity result does not match its columns.' })
  }
  if (parsed.data.kind === 'pcmci-plus-succeeded') {
    const result = parsePcmciPlusEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'pcmci-plus-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'jpcmci-plus-succeeded') {
    const result = parseJpcmciPlusEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'jpcmci-plus-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'lpcmci-succeeded') {
    const result = parseLpcmciEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'lpcmci-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'rpcmci-succeeded') {
    const result = parseRpcmciEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'rpcmci-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'cdnots-succeeded') {
    const result = parseCdnotsResult(parsed.data.result)
    return result.ok
      ? ok({ kind: 'cdnots-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'cdnots-plus-succeeded') {
    const result = parseCdnotsPlusResult(parsed.data.result)
    return result.ok
      ? ok({ kind: 'cdnots-plus-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'grace-succeeded') {
    const result = parseGraceEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'grace-succeeded', request: request.value, result: result.value })
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
  if (parsed.data.kind === 'pc-stable-succeeded') {
    const result = parsePcStableEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'pc-stable-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'fci-succeeded') {
    const result = parseFciEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'fci-succeeded', request: request.value, result: result.value })
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
  if (parsed.data.kind === 'cmlp-succeeded') {
    const result = parseCmlpEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'cmlp-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'clstm-succeeded') {
    const result = parseClstmEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'clstm-succeeded', request: request.value, result: result.value })
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
  if (parsed.data.kind === 'instrumental-variable-succeeded') {
    const result = parseInstrumentalVariableEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'instrumental-variable-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'count-glm-succeeded') {
    const result = parseCountGlmEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'count-glm-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'negative-binomial-ingarch-succeeded') {
    const result = parseNegativeBinomialIngarchEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'negative-binomial-ingarch-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  if (parsed.data.kind === 'count-series-intervention-scan-succeeded') {
    const result = parseCountSeriesInterventionScanEvidence(parsed.data.result)
    return result.ok ? ok({ kind: 'count-series-intervention-scan-succeeded', request: request.value, result: result.value }) : err({ kind: 'invalid-event', detail: result.error.detail })
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
  if (parsed.data.kind === 't-learner-succeeded') {
    const result = tLearnerEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 't-learner-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
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
  if (parsed.data.kind === 'identified-discrete-query-succeeded') {
    const result = identifiedDiscreteQueryEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'identified-discrete-query-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'binary-ett-succeeded') {
    const result = binaryEttEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'binary-ett-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'linear-scm-succeeded') {
    const result = linearScmEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'linear-scm-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
  }
  if (parsed.data.kind === 'dynamic-linear-scm-succeeded') {
    const result = dynamicLinearScmEvidenceSchema.safeParse(parsed.data.result)
    return result.success ? ok({ kind: 'dynamic-linear-scm-succeeded', request: request.value, result: result.data }) : err({ kind: 'invalid-event', detail: z.prettifyError(result.error) })
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
  if (parsed.data.kind === 'pandas-resampling-succeeded') {
    const result = parsePandasResamplingEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'pandas-resampling-succeeded', request: request.value, result: result.value })
      : err({ kind: 'invalid-event', detail: result.error.detail })
  }
  const result = parseStationarityBattery(parsed.data.result)
  return result.ok
    ? ok({ kind: 'stationarity-succeeded', request: request.value, result: result.value })
    : err({ kind: 'invalid-event', detail: result.error.detail })
}
