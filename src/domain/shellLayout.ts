import { z } from 'zod'

/**
 * Shell layout is chrome state: not part of the workflow reducer, not part of a project, never part of a
 * run manifest. It is boundary-parsed on read so a stale or hand-edited entry falls back to the default.
 */
export const shellLayoutSchema = z
  .object({
    version: z.literal(1),
    /** Kept so entries written before the rail replaced the list still parse; nothing reads it. */
    chapterNav: z.enum(['auto', 'expanded', 'collapsed']),
    /** Serialized pane layouts keyed by group id, in the resizable-panel library's own format. */
    panes: z.record(z.string(), z.string()),
    /** Row height for every data table: 24px compact or 32px comfortable. */
    tableDensity: z.enum(['compact', 'comfortable']).default('comfortable'),
  })
  .strict()

export type TableDensity = ShellLayout['tableDensity']

export type ShellLayout = z.infer<typeof shellLayoutSchema>

export const DEFAULT_SHELL_LAYOUT: ShellLayout = {
  version: 1,
  chapterNav: 'auto',
  panes: {},
  tableDensity: 'comfortable',
}

export function parseShellLayout(raw: string | null): ShellLayout {
  if (raw === null) return DEFAULT_SHELL_LAYOUT
  try {
    const parsed = shellLayoutSchema.safeParse(JSON.parse(raw))
    return parsed.success ? parsed.data : DEFAULT_SHELL_LAYOUT
  } catch {
    return DEFAULT_SHELL_LAYOUT
  }
}
