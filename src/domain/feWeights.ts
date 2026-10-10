import { z } from 'zod'

const finite = z.number().finite()
const count = z.number().int().nonnegative()
const share = finite.min(0).max(1 + 1e-12)

export const feWeightDiagnosticSchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('notRecorded') }).strict(),
    z.object({ kind: z.literal('notApplicable') }).strict(),
    z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
    z
      .object({
        kind: z.literal('available'),
        observations: count.positive(),
        groups: z
          .array(
            z
              .object({
                group: count,
                label: z.union([finite, z.string().min(1)]),
                observations: count.positive(),
                variance: finite.nonnegative(),
                sumSquares: finite.nonnegative(),
                weight: share,
                relativeWeight: finite.nonnegative(),
                rankShare: share,
                cumulativeWeight: share,
              })
              .strict(),
          )
          .min(2)
          .max(10_000),
        giniVariance: finite.min(-1e-12).max(1),
        maximumWeight: share,
        effectiveGroups: finite.positive(),
        topFiveShare: share,
        topTenShare: share,
        topDecileShare: share,
        bandwidth: finite.positive(),
        density: z.array(z.tuple([finite, finite.nonnegative()])).length(512),
        dropout: z
          .array(
            z
              .object({
                removedGroups: z.array(count),
                originalWeightRemoved: share,
                remainingObservations: count.positive(),
                fit: z.discriminatedUnion('kind', [
                  z.object({ kind: z.literal('estimated'), coefficient: finite }).strict(),
                  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
                ]),
              })
              .strict(),
          )
          .min(1)
          .max(7),
      })
      .strict(),
  ])
  .superRefine((value, context) => {
    if (value.kind !== 'available') return
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    const groups = value.groups
    if (new Set(groups.map((group) => group.group)).size !== groups.length)
      fail('Group identifiers must be distinct.')
    if (groups.reduce((sum, group) => sum + group.observations, 0) !== value.observations)
      fail('Group counts must match the fitted sample.')
    if (Math.abs(groups.reduce((sum, group) => sum + group.weight, 0) - 1) > 1e-8)
      fail('Identifying weights must sum to one.')
    if (groups.some((group, i) => i > 0 && group.weight > groups[i - 1]!.weight))
      fail('Groups must follow the original descending weight ranking.')
    if (value.dropout[0]?.removedGroups.length !== 0)
      fail('The leave-out sequence must begin with the original sample.')
    for (const point of value.dropout) {
      const removed = groups.slice(0, point.removedGroups.length)
      if (
        point.removedGroups.length > groups.length - 2 ||
        removed.some((group, i) => group.group !== point.removedGroups[i])
      )
        fail(
          'Each leave-out fit must remove a prefix of the original ranking and retain two groups.',
        )
      if (
        point.remainingObservations !==
        value.observations - removed.reduce((sum, group) => sum + group.observations, 0)
      )
        fail('Leave-out counts must match the retained groups.')
    }
  })

export type FeWeightDiagnostic = z.infer<typeof feWeightDiagnosticSchema>
