import { z } from 'zod'
import { err, ok, type Result } from './dop'
import {
  parseSnapshotValue,
  taggedJsonReplacer,
  taggedJsonReviver,
  type PersistedProject,
  type SnapshotProblem,
} from './persistence'

/**
 * The export bundle (DESIGN.md §17): a versioned envelope around the project record, with the source
 * file included only when the user asked for it after seeing its size. Re-import parses at the boundary
 * and refuses a version it does not know rather than guessing at fields.
 */

export type BundleData =
  | { readonly kind: 'not-included' }
  | {
      readonly kind: 'source-file'
      readonly name: string
      readonly mediaType: string
      readonly lastModified: number
      readonly bytes: number
      readonly base64: string
    }

export interface ProjectBundle {
  readonly kind: 'hirmos-bundle'
  readonly version: 1
  readonly exportedAt: string
  readonly application: { readonly name: 'hirmos'; readonly build: string }
  readonly project: PersistedProject
  readonly data: BundleData
}

declare const __APP_VERSION__: string

export const buildBundle = (
  project: PersistedProject,
  data: BundleData,
  exportedAt: string,
): ProjectBundle => ({
  kind: 'hirmos-bundle',
  version: 1,
  exportedAt,
  application: {
    name: 'hirmos',
    build: typeof __APP_VERSION__ === 'string' ? __APP_VERSION__ : 'dev',
  },
  project,
  data,
})

export const serialiseBundle = (bundle: ProjectBundle): string =>
  JSON.stringify(bundle, taggedJsonReplacer, 2)

const safeName = (name: string): string =>
  name
    .replace(/[^\p{L}\p{N}]+/gu, '-')
    .replace(/^-|-$/g, '')
    .toLowerCase() || 'project'

export const bundleFileName = (bundle: ProjectBundle): string =>
  `hirmos-${safeName(bundle.project.project.name)}-${bundle.exportedAt.slice(0, 10)}.hirmos.json`

export type BundleProblem =
  | { readonly kind: 'not-json'; readonly detail: string }
  | { readonly kind: 'not-a-bundle'; readonly detail: string }
  | { readonly kind: 'snapshot-not-bundle' }
  | { readonly kind: 'unsupported-bundle-version'; readonly version: number }
  | { readonly kind: 'project-invalid'; readonly problem: SnapshotProblem }

const bundleSchema = z
  .object({
    kind: z.literal('hirmos-bundle'),
    version: z.number().int(),
    exportedAt: z.string().datetime({ offset: true }),
    application: z.object({ name: z.literal('hirmos'), build: z.string() }).strict(),
    project: z.unknown(),
    data: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('not-included') }).strict(),
      z
        .object({
          kind: z.literal('source-file'),
          name: z.string().min(1),
          mediaType: z.string(),
          lastModified: z.number(),
          bytes: z.number().int().nonnegative(),
          base64: z.string(),
        })
        .strict(),
    ]),
  })
  .strict()

export function parseBundle(raw: string): Result<ProjectBundle, BundleProblem> {
  let value: unknown
  try {
    value = JSON.parse(raw, taggedJsonReviver)
  } catch (cause) {
    return err({ kind: 'not-json', detail: cause instanceof Error ? cause.message : String(cause) })
  }
  const parsed = bundleSchema.safeParse(value)
  if (!parsed.success) {
    // A saved project is the app's own record, not the file Export writes.
    const snapshot =
      typeof value === 'object' && value !== null && Reflect.get(value, 'kind') === 'hirmos-project'
    return err(
      snapshot
        ? { kind: 'snapshot-not-bundle' }
        : { kind: 'not-a-bundle', detail: z.prettifyError(parsed.error) },
    )
  }
  if (parsed.data.version !== 1)
    return err({ kind: 'unsupported-bundle-version', version: parsed.data.version })
  const project = parseSnapshotValue(parsed.data.project)
  if (!project.ok) return err({ kind: 'project-invalid', problem: project.error })
  return ok({ ...parsed.data, version: 1, project: project.value })
}

export function describeBundleProblem(problem: BundleProblem): string {
  switch (problem.kind) {
    case 'not-json':
      return `The file is not JSON: ${problem.detail}`
    case 'not-a-bundle':
      return `The file is not a Hirmos project bundle: ${problem.detail}`
    case 'snapshot-not-bundle':
      return 'This looks like a saved project snapshot and not an export bundle'
    case 'unsupported-bundle-version':
      return `The bundle was written by a newer version (${problem.version}).`
    case 'project-invalid':
      return `The bundle's project record could not be read: ${problem.problem.kind}`
    default: {
      const exhaustive: never = problem
      return exhaustive
    }
  }
}
