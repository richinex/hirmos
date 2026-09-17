import { z } from 'zod'
import { sourceFingerprint, type SourceFingerprint } from './dataset'
import { brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'

/**
 * The files a derived source is built from, as every derivation records them: an alias the recipe
 * refers to, the file's name and size, its format, and its fingerprint, which a replay matches the
 * offered files against. The SQL statement and the pipeline both read from these aliases, and
 * neither owns them.
 */

export type SqlInputAlias = Brand<string, 'SqlInputAlias'>

export interface SqlInputDescriptor {
  readonly alias: SqlInputAlias
  readonly fileName: string
  readonly bytes: number
  readonly format: 'csv' | 'tsv' | 'parquet'
  readonly fingerprint: SourceFingerprint
}

export interface SqlPreparationInput extends SqlInputDescriptor {
  readonly file: File
}

/** Recovered files keep the aliases recorded in the recipe, not their current filenames. */
export function matchInputFiles(descriptors: NonEmptyArray<SqlInputDescriptor>, offered: readonly SqlPreparationInput[]): Result<NonEmptyArray<SqlPreparationInput>, string> {
  const inputs: SqlPreparationInput[] = []
  const missing: string[] = []
  for (const descriptor of descriptors) {
    const input = offered.find(candidate => candidate.fingerprint === descriptor.fingerprint)
    if (input === undefined) missing.push(descriptor.fileName)
    else inputs.push({ ...input, alias: descriptor.alias })
  }
  return missing.length === 0 && isNonEmpty(inputs)
    ? ok(inputs)
    : err(`The files chosen do not include ${missing.join(', ')}, unchanged.`)
}

export type SqlAliasProblem =
  | { readonly kind: 'empty-alias' }
  | { readonly kind: 'reserved-alias'; readonly alias: string }
  | { readonly kind: 'duplicate-alias'; readonly alias: string }

/** Reserved by the SQL workspace; an input may not take it. */
export const PREPARED_VIEW = 'hirmos_prepared'

const normaliseAlias = (value: string): string => value
  .normalize('NFKD')
  .replace(/[̀-ͯ]/g, '')
  .replace(/\.[^.]+$/, '')
  .replace(/[^A-Za-z0-9_]+/g, '_')
  .replace(/^_+|_+$/g, '')
  .toLowerCase()

export function sqlInputAlias(value: string, occupied: ReadonlySet<string>): Result<SqlInputAlias, SqlAliasProblem> {
  const alias = normaliseAlias(value)
  if (alias.length === 0) return err({ kind: 'empty-alias' })
  if (alias === PREPARED_VIEW) return err({ kind: 'reserved-alias', alias })
  if (occupied.has(alias)) return err({ kind: 'duplicate-alias', alias })
  return ok(brand<string, 'SqlInputAlias'>(alias))
}

export function uniqueSqlInputAlias(fileName: string, occupied: ReadonlySet<string>): SqlInputAlias {
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

export const inputDescriptor = ({ file: _file, ...descriptor }: SqlPreparationInput): SqlInputDescriptor => descriptor

export const inputDescriptorSchema = z.object({
  alias: z.string().min(1),
  fileName: z.string().min(1),
  bytes: z.number().int().positive(),
  format: z.enum(['csv', 'tsv', 'parquet']),
  fingerprint: z.string(),
}).strict()

export function parseInputDescriptors(
  values: readonly z.infer<typeof inputDescriptorSchema>[],
): Result<NonEmptyArray<SqlInputDescriptor>, { readonly detail: string }> {
  const descriptors: SqlInputDescriptor[] = []
  const occupied = new Set<string>()
  for (const input of values) {
    const alias = sqlInputAlias(input.alias, occupied)
    if (!alias.ok) return err({ detail: `Invalid SQL input alias: ${input.alias}` })
    const fingerprint = sourceFingerprint(input.fingerprint)
    if (!fingerprint.ok) return err({ detail: `Invalid fingerprint for input ${input.alias}.` })
    occupied.add(alias.value)
    descriptors.push({ ...input, alias: alias.value, fingerprint: fingerprint.value })
  }
  return isNonEmpty(descriptors) ? ok(descriptors) : err({ detail: 'The recipe has no inputs.' })
}
