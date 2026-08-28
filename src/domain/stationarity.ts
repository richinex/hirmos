import { z } from 'zod'
import { err, ok, type Result } from './dop'

const adfEvidenceSchema = z.object({
  statistic: z.number().finite(),
  pValue: z.number().min(0).max(1),
  usedLag: z.number().int().nonnegative(),
  observations: z.number().int().positive(),
  criticalValues: z.tuple([z.number(), z.number(), z.number()]),
}).strict()

const kpssEvidenceSchema = z.object({
  statistic: z.number().finite(),
  pValue: z.number().min(0).max(1),
  usedLag: z.number().int().nonnegative(),
  criticalValues: z.tuple([z.number(), z.number(), z.number(), z.number()]),
}).strict()

const zivotAndrewsEvidenceSchema = z.object({
  statistic: z.number().finite(),
  pValue: z.number().min(0).max(1),
  criticalValues: z.tuple([z.number(), z.number(), z.number()]),
  baseLags: z.number().int().nonnegative(),
  breakIndex: z.number().int().nonnegative(),
}).strict()

export const stationarityBatterySchema = z.object({
  kind: z.literal('stationarityBattery'),
  observations: z.number().int().positive(),
  adf: z.object({
    constant: adfEvidenceSchema,
    constantAndTrend: adfEvidenceSchema,
  }).strict(),
  kpss: z.object({
    constant: kpssEvidenceSchema,
    constantAndTrend: kpssEvidenceSchema,
  }).strict(),
  zivotAndrews: z.object({
    level: zivotAndrewsEvidenceSchema,
    trend: zivotAndrewsEvidenceSchema,
    levelAndTrend: zivotAndrewsEvidenceSchema,
  }).strict(),
}).strict()

export type StationarityBattery = z.infer<typeof stationarityBatterySchema>

export type StationarityBoundaryProblem = {
  readonly kind: 'invalid-stationarity-result'
  readonly detail: string
}

export function parseStationarityBattery(value: unknown): Result<StationarityBattery, StationarityBoundaryProblem> {
  const parsed = stationarityBatterySchema.safeParse(value)
  return parsed.success
    ? ok(parsed.data)
    : err({ kind: 'invalid-stationarity-result', detail: z.prettifyError(parsed.error) })
}
