import { z } from 'zod'
import { sourceFingerprint, type SourceFingerprint } from './dataset'
import { brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'

export type SqlInputAlias = Brand<string, 'SqlInputAlias'>
export type SqlViewName = Brand<string, 'SqlViewName'>

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

export type SourceRecipe =
  | { readonly kind: 'uploaded-file' }
  | {
      readonly kind: 'sql-derived'
      readonly outputView: SqlViewName
      readonly statement: string
      readonly inputs: NonEmptyArray<SqlInputDescriptor>
    }

export type SqlAliasProblem =
  | { readonly kind: 'empty-alias' }
  | { readonly kind: 'reserved-alias'; readonly alias: string }
  | { readonly kind: 'duplicate-alias'; readonly alias: string }

export const PREPARED_VIEW = 'hirmos_prepared'

export function sqlViewName(value: string): Result<SqlViewName, { readonly kind: 'invalid-view-name' }> {
  return value.trim().length === 0
    ? err({ kind: 'invalid-view-name' })
    : ok(brand<string, 'SqlViewName'>(value))
}

const normaliseAlias = (value: string): string => value
  .normalize('NFKD')
  .replace(/[\u0300-\u036f]/g, '')
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

export function sqlDerivedRecipe(
  statement: string,
  outputView: SqlViewName,
  inputs: readonly SqlPreparationInput[],
): Result<Extract<SourceRecipe, { readonly kind: 'sql-derived' }>, { readonly kind: 'invalid-sql-derivation' }> {
  const canonical = statement.trim()
  if (canonical.length === 0 || !isNonEmpty(inputs)) return err({ kind: 'invalid-sql-derivation' })
  const [first, ...rest] = inputs
  const withoutFile = ({ file: _file, ...descriptor }: SqlPreparationInput): SqlInputDescriptor => descriptor
  return ok({
    kind: 'sql-derived',
    outputView,
    statement: canonical,
    inputs: [withoutFile(first), ...rest.map(withoutFile)],
  })
}

const sourceRecipeSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('uploaded-file') }).strict(),
  z.object({
    kind: z.literal('sql-derived'),
    outputView: z.string().min(1),
    statement: z.string().trim().min(1),
    inputs: z.array(z.object({
      alias: z.string().min(1),
      fileName: z.string().min(1),
      bytes: z.number().int().positive(),
      format: z.enum(['csv', 'tsv', 'parquet']),
      fingerprint: z.string(),
    }).strict()).min(1),
  }).strict(),
])

export function parseSourceRecipe(value: unknown): Result<SourceRecipe, { readonly kind: 'invalid-source-recipe'; readonly detail: string }> {
  const parsed = sourceRecipeSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-source-recipe', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'uploaded-file') return ok(parsed.data)
  const outputView = sqlViewName(parsed.data.outputView)
  if (!outputView.ok) return err({ kind: 'invalid-source-recipe', detail: 'The SQL output view name is invalid.' })
  const descriptors: SqlInputDescriptor[] = []
  const occupied = new Set<string>()
  for (const input of parsed.data.inputs) {
    const alias = sqlInputAlias(input.alias, occupied)
    if (!alias.ok) return err({ kind: 'invalid-source-recipe', detail: `Invalid SQL input alias: ${input.alias}` })
    const fingerprint = sourceFingerprint(input.fingerprint)
    if (!fingerprint.ok) return err({ kind: 'invalid-source-recipe', detail: `Invalid fingerprint for SQL input ${input.alias}.` })
    occupied.add(alias.value)
    descriptors.push({ ...input, alias: alias.value, fingerprint: fingerprint.value })
  }
  if (!isNonEmpty(descriptors)) return err({ kind: 'invalid-source-recipe', detail: 'The SQL source recipe has no inputs.' })
  return ok({ kind: 'sql-derived', outputView: outputView.value, statement: parsed.data.statement, inputs: descriptors })
}
