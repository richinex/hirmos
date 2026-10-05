import { z } from 'zod'
import { brand } from './dop'
import type { SurrogateSelection, SurrogateSamples } from './surrogateSamples'
import type { SurrogateEvidence } from './surrogate'
import type { SurrogatePathEvidence, SurrogatePathRequest } from './surrogatePath'

const column = z
  .string()
  .min(1)
  .transform((v) => brand<string, 'ColumnId'>(v))
const label = z.string().trim().min(1)
export const noSurrogateHorizons = {
  observed: { kind: 'none' },
  windows: { kind: 'none' },
} as const
export const surrogateHorizonsSchema = z
  .object({
    observed: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z
        .object({
          kind: z.literal('periods'),
          periods: z.array(z.object({ column, label }).strict().readonly()).min(1).readonly(),
        })
        .strict(),
    ]),
    windows: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z
        .object({
          kind: z.literal('groups'),
          groups: z
            .array(
              z
                .object({ label, columns: z.array(column).min(1).readonly() })
                .strict()
                .readonly(),
            )
            .min(1)
            .readonly(),
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((h, ctx) => {
    if (
      h.observed.kind === 'periods' &&
      (new Set(h.observed.periods.map((p) => p.column)).size !== h.observed.periods.length ||
        new Set(h.observed.periods.map((p) => p.label)).size !== h.observed.periods.length)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'Observed periods need distinct columns and distinct labels.',
      })
    if (h.windows.kind === 'groups') {
      const columns = h.windows.groups.flatMap((g) => g.columns)
      if (
        new Set(columns).size !== columns.length ||
        new Set(h.windows.groups.map((g) => g.label)).size !== h.windows.groups.length
      )
        ctx.addIssue({
          code: 'custom',
          message:
            'Each surrogate must belong to one horizon group, with a distinct label for each group.',
        })
    }
  })
  .readonly()
export type SurrogateHorizons = z.infer<typeof surrogateHorizonsSchema>

export function surrogateWindowEnds(
  h: SurrogateHorizons,
): readonly { readonly label: string; readonly surrogateColumns: number }[] {
  if (h.windows.kind === 'none') return []
  let count = 0
  return h.windows.groups.map((g) => ({
    label: g.label,
    surrogateColumns: (count += g.columns.length),
  }))
}

/** Reorder predictors only, preserving the fixed outcome, participants and baseline. */
export function surrogateHorizonRequests(
  samples: SurrogateSamples,
  s: SurrogateSelection,
): readonly SurrogatePathRequest[] {
  const requests: SurrogatePathRequest[] = []
  if (samples.observedPath.kind !== 'none') requests.push(samples.observedPath)
  if (s.horizons.windows.kind === 'groups') {
    const order = s.horizons.windows.groups
      .flatMap((g) => g.columns)
      .map((id) => s.surrogates.indexOf(id))
    const reorder = (rows: readonly (readonly number[])[]) =>
      rows.map((row) => order.map((i) => row[i]!))
    requests.push({
      kind: 'surrogateWindows',
      model: {
        ...samples.request,
        experimental: {
          ...samples.request.experimental,
          surrogates: reorder(samples.request.experimental.surrogates),
        },
        observational: {
          ...samples.request.observational,
          surrogates: reorder(samples.request.observational.surrogates),
        },
      },
      windows: surrogateWindowEnds(s.horizons),
    })
  }
  return requests
}

/** Persistence validates meaning and provenance, not merely JSON shape. */
export function surrogateHorizonRecordsMatch(
  s: SurrogateSelection,
  e: SurrogateEvidence,
  paths: readonly SurrogatePathEvidence[],
): boolean {
  const observed = paths.filter((p) => p.kind === 'observedOutcomes'),
    windows = paths.filter((p) => p.kind === 'surrogateWindows')
  if (
    observed.length !== (s.horizons.observed.kind === 'none' ? 0 : 1) ||
    windows.length !== (s.horizons.windows.kind === 'none' ? 0 : 1)
  )
    return false
  if (s.horizons.observed.kind === 'periods') {
    const p = observed[0]!,
      periods = s.horizons.observed.periods
    const u = p.cumulativeUncertainty
    if (
      u.kind !== 'notRecorded' &&
      (s.uncertainty.kind === 'none'
        ? u.kind !== 'none'
        : u.kind !== 'bootstrapStandardErrors' ||
          u.seed !== s.uncertainty.seed ||
          u.repetitions !== s.uncertainty.repetitions)
    )
      return false
    if (
      p.experimentalRows !== e.experimentalRows ||
      p.periods.length !== periods.length ||
      p.periods.some((v, i) => v.label !== periods[i]!.label)
    )
      return false
  }
  if (s.horizons.windows.kind === 'groups') {
    const ends = surrogateWindowEnds(s.horizons),
      p = windows[0]!
    if (
      p.windows.length === 0 ||
      p.windows.length !== ends.length ||
      p.windows.some((w, i) => {
        const v = w.evidence,
          u = e.uncertainty
        return (
          w.label !== ends[i]!.label ||
          v.surrogateColumns !== ends[i]!.surrogateColumns ||
          v.estimator !== e.estimator ||
          v.experimentalRows !== e.experimentalRows ||
          v.observationalRows !== e.observationalRows ||
          v.baselineColumns !== e.baselineColumns ||
          (u.kind === 'none'
            ? v.uncertainty.kind !== 'none'
            : v.uncertainty.kind !== 'bootstrapStandardError' ||
              v.uncertainty.seed !== u.seed ||
              v.uncertainty.repetitions !== u.repetitions)
        )
      })
    )
      return false
    const last = p.windows.at(-1)!.evidence
    const close = (a: number, b: number) => Math.abs(a - b) <= 1e-8 * (1 + Math.abs(b))
    if (!close(last.estimate, e.estimate)) return false
    if (
      last.uncertainty.kind === 'bootstrapStandardError' &&
      e.uncertainty.kind === 'bootstrapStandardError' &&
      !close(last.uncertainty.standardError, e.uncertainty.standardError)
    )
      return false
  }
  return true
}

export function sameSurrogateHorizons(a: SurrogateHorizons, b: SurrogateHorizons): boolean {
  const observed =
    a.observed.kind === 'none'
      ? b.observed.kind === 'none'
      : b.observed.kind === 'periods' &&
        a.observed.periods.length === b.observed.periods.length &&
        a.observed.periods.every(
          (p, i) =>
            b.observed.kind === 'periods' &&
            p.column === b.observed.periods[i]!.column &&
            p.label === b.observed.periods[i]!.label,
        )
  const windows =
    a.windows.kind === 'none'
      ? b.windows.kind === 'none'
      : b.windows.kind === 'groups' &&
        a.windows.groups.length === b.windows.groups.length &&
        a.windows.groups.every((g, i) => {
          if (b.windows.kind !== 'groups') return false
          const other = b.windows.groups[i]!
          return (
            g.label === other.label &&
            g.columns.length === other.columns.length &&
            g.columns.every((id, j) => id === other.columns[j])
          )
        })
  return observed && windows
}
