declare const brandSymbol: unique symbol

export type Brand<Value, Name extends string> = Value & {
  readonly [brandSymbol]: Name
}

export type Result<Value, Problem> =
  | { readonly ok: true; readonly value: Value }
  | { readonly ok: false; readonly error: Problem }

export const ok = <Value>(value: Value): Result<Value, never> => ({ ok: true, value })
export const err = <Problem>(error: Problem): Result<never, Problem> => ({ ok: false, error })

export type NonEmptyArray<Value> = readonly [Value, ...Value[]]

export function isNonEmpty<Value>(values: readonly Value[]): values is NonEmptyArray<Value> {
  return values.length > 0
}

export function mapNonEmpty<Input, Output>(
  values: NonEmptyArray<Input>,
  transform: (value: Input, index: number) => Output,
): NonEmptyArray<Output> {
  const [first, ...rest] = values
  return [transform(first, 0), ...rest.map((value, index) => transform(value, index + 1))]
}

export function flattenNonEmpty<Value>(groups: NonEmptyArray<NonEmptyArray<Value>>): NonEmptyArray<Value> {
  const [[first, ...initial], ...remaining] = groups
  return [first, ...initial, ...remaining.flat()]
}

export function brand<Value, Name extends string>(value: Value): Brand<Value, Name> {
  return value as Brand<Value, Name>
}

export function assertNever(value: never): never {
  throw new Error(`Unreachable variant: ${JSON.stringify(value)}`)
}
