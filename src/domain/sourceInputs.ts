import { z } from 'zod'
import { sourceFingerprint, type SourceFingerprint } from './dataset'
import { brand, err, ok, type Brand, type Result } from './dop'
import { columnDeclarationsSchema, fileReadingOf, type FileReading } from './fileReading'

/**
 * The files a derived source is built from, as every derivation records them: an alias the recipe
 * refers to, the file's name and size, its format, and its fingerprint, which a replay matches the
 * offered files against. The SQL statement and the pipeline both read from these aliases, and
 * neither owns them.
 */

export type SqlInputAlias = Brand<string, 'SqlInputAlias'>

interface InputFileDescriptor {
  readonly alias: SqlInputAlias
  readonly fileName: string
  readonly bytes: number
  readonly fingerprint: SourceFingerprint
}

export type SqlInputDescriptor = InputFileDescriptor &
  (FileReading | { readonly format: 'duckdb-export-file'; readonly path: string })

export type SqlPreparationInput = SqlInputDescriptor & { readonly file: File }

export type SqlAliasProblem =
  | { readonly kind: 'empty-alias' }
  | { readonly kind: 'reserved-alias'; readonly alias: string }
  | { readonly kind: 'duplicate-alias'; readonly alias: string }

/** Reserved by the SQL workspace; an input may not take it. */
export const PREPARED_VIEW = 'hirmos_prepared'

const normaliseAlias = (value: string): string =>
  value
    .normalize('NFKD')
    .replace(/[̀-ͯ]/g, '')
    .replace(/\.[^.]+$/, '')
    .replace(/[^A-Za-z0-9_]+/g, '_')
    .replace(/^_+|_+$/g, '')
    .toLowerCase()

export function sqlInputAlias(
  value: string,
  occupied: ReadonlySet<string>,
): Result<SqlInputAlias, SqlAliasProblem> {
  const alias = normaliseAlias(value)
  if (alias.length === 0) return err({ kind: 'empty-alias' })
  if (alias === PREPARED_VIEW) return err({ kind: 'reserved-alias', alias })
  if (occupied.has(alias)) return err({ kind: 'duplicate-alias', alias })
  return ok(brand<string, 'SqlInputAlias'>(alias))
}

export function uniqueSqlInputAlias(
  fileName: string,
  occupied: ReadonlySet<string>,
): SqlInputAlias {
  const initial = normaliseAlias(fileName) || 'input'
  const base = initial === PREPARED_VIEW ? 'input' : initial
  let candidate = base
  let suffix = 2
  while (occupied.has(candidate)) {
    candidate = `${base}_${suffix}`
    suffix += 1
  }
  return brand<string, 'SqlInputAlias'>(candidate)
}

export const inputDescriptor = ({
  file: _file,
  ...descriptor
}: SqlPreparationInput): SqlInputDescriptor => descriptor

const fileFields = {
  alias: z.string().min(1),
  fileName: z.string().min(1),
  bytes: z.number().int().positive(),
  fingerprint: z.string(),
}
export const inputDescriptorSchema = z.union([
  z
    .object({
      ...fileFields,
      format: z.enum(['csv', 'tsv', 'parquet']),
      declared: columnDeclarationsSchema.optional(),
    })
    .strict(),
  z
    .object({
      ...fileFields,
      bytes: z.number().int().nonnegative(),
      format: z.literal('duckdb-export-file'),
      path: z.string().min(1),
    })
    .strict(),
])

/** Export paths are local relative names, never URLs or parent-directory traversals. */
export function exportDirectory(
  paths: readonly string[],
): Result<string, { readonly detail: string }> {
  const roots = new Set<string>()
  const seen = new Set<string>()
  for (const path of paths) {
    const parts = path.split('/')
    if (
      parts.length < 2 ||
      /[\\\\:\u0000-\u001f]/.test(path) ||
      parts.some((part) => part === '' || part === '.' || part === '..')
    )
      return err({ detail: `Invalid export path: ${path}` })
    if (seen.has(path)) return err({ detail: `Duplicate export path: ${path}` })
    seen.add(path)
    roots.add(parts[0]!)
  }
  const root = [...roots][0]
  if (roots.size !== 1 || root === undefined)
    return err({ detail: 'Choose one DuckDB export folder.' })
  if (!seen.has(`${root}/schema.sql`) || !seen.has(`${root}/load.sql`))
    return err({ detail: 'The export folder must contain schema.sql and load.sql.' })
  return ok(root)
}

export function parseInputDescriptors(
  values: readonly z.infer<typeof inputDescriptorSchema>[],
): Result<readonly SqlInputDescriptor[], { readonly detail: string }> {
  const descriptors: SqlInputDescriptor[] = []
  const occupied = new Set<string>()
  for (const input of values) {
    const alias = sqlInputAlias(input.alias, occupied)
    if (!alias.ok) return err({ detail: `Invalid SQL input alias: ${input.alias}` })
    const fingerprint = sourceFingerprint(input.fingerprint)
    if (!fingerprint.ok) return err({ detail: `Invalid fingerprint for input ${input.alias}.` })
    occupied.add(alias.value)
    const identity = {
      alias: alias.value,
      fileName: input.fileName,
      bytes: input.bytes,
      fingerprint: fingerprint.value,
    }
    if (input.format === 'duckdb-export-file') {
      descriptors.push({ ...identity, format: input.format, path: input.path })
      continue
    }
    const reading = fileReadingOf(input.format, input.declared)
    if (!reading.ok)
      return err({
        detail: `Input ${input.alias} is a Parquet file, which stores its column types, but declares some.`,
      })
    descriptors.push({ ...identity, ...reading.value })
  }
  const exports = descriptors.filter((input) => input.format === 'duckdb-export-file')
  if (exports.length > 0) {
    const directory = exportDirectory(exports.map((input) => input.path))
    if (!directory.ok) return directory
  }
  return ok(descriptors)
}
