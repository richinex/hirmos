import { z } from 'zod'
import { brand, err, ok, type Brand, type Result } from '@/domain/dop'
import {
  dynotearsEvidenceSchema,
  grangerSsrEvidenceSchema,
  lpcmciEvidenceSchema,
  ocseEvidenceSchema,
  parseDynotearsEvidence,
  parseGrangerSsrEvidence,
  parseLpcmciEvidence,
  parseOcseEvidence,
  parsePcmciPlusEvidence,
  pcmciPlusEvidenceSchema,
  type DynotearsEvidence,
  type GrangerSsrEvidence,
  type LpcmciEvidence,
  type OcseEvidence,
  type PcmciPlusEvidence,
} from '@/domain/discovery'
import {
  parseStationarityBattery,
  stationarityBatterySchema,
  type StationarityBattery,
} from '@/domain/stationarity'

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
  const result = parseStationarityBattery(parsed.data.result)
  return result.ok
    ? ok({ kind: 'stationarity-succeeded', request: request.value, result: result.value })
    : err({ kind: 'invalid-event', detail: result.error.detail })
}
