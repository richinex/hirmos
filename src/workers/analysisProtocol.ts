import { z } from 'zod'
import { brand, err, ok, type Brand, type Result } from '@/domain/dop'
import {
  grangerSsrEvidenceSchema,
  parseGrangerSsrEvidence,
  parsePcmciPlusEvidence,
  pcmciPlusEvidenceSchema,
  type GrangerSsrEvidence,
  type PcmciPlusEvidence,
} from '@/domain/discovery'
import {
  parseStationarityBattery,
  stationarityBatterySchema,
  type StationarityBattery,
} from '@/domain/stationarity'

export type WorkerRequestId = Brand<string, 'WorkerRequestId'>

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

const eventSchema = z.discriminatedUnion('kind', [
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
  if (parsed.data.kind === 'pcmci-plus' && parsed.data.values.length !== parsed.data.rows * parsed.data.columns) {
    return err({ kind: 'invalid-command', detail: 'The PCMCI+ matrix dimensions do not match its numeric buffer.' })
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
  if (parsed.data.kind === 'pcmci-plus-succeeded') {
    const result = parsePcmciPlusEvidence(parsed.data.result)
    return result.ok
      ? ok({ kind: 'pcmci-plus-succeeded', request: request.value, result: result.value })
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
