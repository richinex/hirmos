import { assertNever } from './dop'
import { EXAMPLE_QUESTIONS, type ShippedExample } from './example'

export type ExampleSort = 'catalog' | 'name'

export interface ExampleFilter {
  readonly query: string
  readonly shape: string | null
  readonly sort: ExampleSort
}

/** Filtering never changes the catalog or the identity of a saved copy. */
export function browseExamples(
  examples: readonly ShippedExample[],
  filter: ExampleFilter,
): readonly ShippedExample[] {
  const query = filter.query.trim().toLocaleLowerCase('en')
  const selected = examples.filter((example) => {
    const question = EXAMPLE_QUESTIONS.find((entry) => entry.id === example.question)
    const text = [
      example.name,
      example.sourceName,
      example.approach,
      example.shape,
      question?.title ?? '',
    ]
      .join(' ')
      .toLocaleLowerCase('en')
    return (filter.shape === null || example.shape === filter.shape) && text.includes(query)
  })
  switch (filter.sort) {
    case 'catalog':
      return selected
    case 'name':
      return selected.sort((a, b) => a.name.localeCompare(b.name, 'en') || a.id.localeCompare(b.id))
    default:
      return assertNever(filter.sort)
  }
}
