import { z } from 'zod'

/** What a window on the results desk shows: a saved record, or one of the desk's own views of them. */
export type DeskWindowRef =
  | { readonly kind: 'runs' }
  | { readonly kind: 'differences' }
  | { readonly kind: 'estimate'; readonly id: string }
  | { readonly kind: 'record'; readonly id: string }
  | { readonly kind: 'sensitivity'; readonly id: string }
  | { readonly kind: 'survival'; readonly id: string }
  | { readonly kind: 'time-series'; readonly id: string }
  | { readonly kind: 'count-series'; readonly id: string }
  | { readonly kind: 'surrogate'; readonly id: string }
  | { readonly kind: 'root-cause'; readonly id: string }
  | { readonly kind: 'gcm-effect'; readonly id: string }
  | { readonly kind: 'gcm-influence'; readonly id: string }

export type DeskWindowKind = DeskWindowRef['kind']

const recordKinds = [
  'estimate',
  'record',
  'sensitivity',
  'survival',
  'time-series',
  'count-series',
  'surrogate',
  'root-cause',
  'gcm-effect',
  'gcm-influence',
] as const

export const deskKey = (ref: DeskWindowRef): string =>
  ref.kind === 'runs' || ref.kind === 'differences' ? ref.kind : `${ref.kind}:${ref.id}`

export function parseDeskKey(key: string): DeskWindowRef | null {
  if (key === 'runs' || key === 'differences') return { kind: key }
  const at = key.indexOf(':')
  if (at < 0) return null
  const kind = key.slice(0, at)
  const id = key.slice(at + 1)
  if (id === '') return null
  return (recordKinds as readonly string[]).includes(kind) ? ({ kind, id } as DeskWindowRef) : null
}

export const DESK_VIEWS = ['desk', 'compare', 'evidence', 'focus'] as const
export type DeskViewName = (typeof DESK_VIEWS)[number]

/** Windows on one desk at most. Past it, the window longest out of the front leaves the desk. */
export const DESK_CAPACITY = 10

const rectSchema = z
  .object({
    left: z.number().finite(),
    top: z.number().finite(),
    width: z.number().finite().positive(),
    height: z.number().finite().positive(),
  })
  .strict()

const windowSchema = z
  .object({
    key: z.string().min(1),
    placed: rectSchema.nullable(),
    visible: z.boolean(),
    maximized: z.boolean(),
    rank: z.number().int().nonnegative(),
  })
  .strict()

export const deskStateSchema = z
  .object({
    version: z.literal(1),
    windows: z.array(windowSchema).max(DESK_CAPACITY),
    view: z.enum(DESK_VIEWS),
    ratios: z.record(z.string(), z.number().min(0.1).max(0.9)),
    order: z
      .object({
        compare: z.array(z.string()).max(DESK_CAPACITY).optional(),
        evidence: z.array(z.string()).max(DESK_CAPACITY).optional(),
      })
      .strict()
      .default({}),
    focused: z.string().nullable(),
    pair: z
      .object({ selected: z.string().nullable(), compareWith: z.string().nullable() })
      .strict(),
  })
  .strict()

export type DeskState = z.infer<typeof deskStateSchema>
export type DeskWindowState = DeskState['windows'][number]

export function parseDeskState(raw: unknown): DeskState | null {
  const parsed = deskStateSchema.safeParse(raw)
  if (!parsed.success) return null
  const keys = new Set<string>()
  for (const window of parsed.data.windows) {
    if (parseDeskKey(window.key) === null || keys.has(window.key)) return null
    keys.add(window.key)
  }
  return parsed.data
}
