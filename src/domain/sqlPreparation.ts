import { z } from 'zod'
import { brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { parsePipelineRecipe, pipelineRecipeSchema, type PipelineRecipe } from './pipeline'
import { inputDescriptor, inputDescriptorSchema, parseInputDescriptors, type SqlInputDescriptor, type SqlPreparationInput } from './sourceInputs'

export { PREPARED_VIEW, sqlInputAlias, uniqueSqlInputAlias } from './sourceInputs'
export type { SqlAliasProblem, SqlInputAlias, SqlInputDescriptor, SqlPreparationInput } from './sourceInputs'

export type SqlViewName = Brand<string, 'SqlViewName'>

export type SourceRecipe =
  | { readonly kind: 'uploaded-file' }
  | {
      readonly kind: 'sql-derived'
      readonly outputView: SqlViewName
      readonly statement: string
      readonly inputs: NonEmptyArray<SqlInputDescriptor>
    }
  | PipelineRecipe

export function sqlViewName(value: string): Result<SqlViewName, { readonly kind: 'invalid-view-name' }> {
  return value.trim().length === 0
    ? err({ kind: 'invalid-view-name' })
    : ok(brand<string, 'SqlViewName'>(value))
}

export function sqlDerivedRecipe(
  statement: string,
  outputView: SqlViewName,
  inputs: readonly SqlPreparationInput[],
): Result<Extract<SourceRecipe, { readonly kind: 'sql-derived' }>, { readonly kind: 'invalid-sql-derivation' }> {
  const canonical = statement.trim()
  if (canonical.length === 0 || !isNonEmpty(inputs)) return err({ kind: 'invalid-sql-derivation' })
  const [first, ...rest] = inputs
  return ok({
    kind: 'sql-derived',
    outputView,
    statement: canonical,
    inputs: [inputDescriptor(first), ...rest.map(inputDescriptor)],
  })
}

const sourceRecipeSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('uploaded-file') }).strict(),
  z.object({
    kind: z.literal('sql-derived'),
    outputView: z.string().min(1),
    statement: z.string().trim().min(1),
    inputs: z.array(inputDescriptorSchema).min(1),
  }).strict(),
  pipelineRecipeSchema,
])

export function parseSourceRecipe(value: unknown): Result<SourceRecipe, { readonly kind: 'invalid-source-recipe'; readonly detail: string }> {
  const parsed = sourceRecipeSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-source-recipe', detail: z.prettifyError(parsed.error) })
  if (parsed.data.kind === 'uploaded-file') return ok(parsed.data)
  if (parsed.data.kind === 'pipeline-derived') {
    const recipe = parsePipelineRecipe(parsed.data)
    return recipe.ok ? ok(recipe.value) : err({ kind: 'invalid-source-recipe', detail: recipe.error.detail })
  }
  const outputView = sqlViewName(parsed.data.outputView)
  if (!outputView.ok) return err({ kind: 'invalid-source-recipe', detail: 'The SQL output view name is invalid.' })
  const descriptors = parseInputDescriptors(parsed.data.inputs)
  if (!descriptors.ok) return err({ kind: 'invalid-source-recipe', detail: descriptors.error.detail })
  return ok({ kind: 'sql-derived', outputView: outputView.value, statement: parsed.data.statement, inputs: descriptors.value })
}
