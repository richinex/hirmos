/** Builds a value on the first call and returns that value after it, so a large schema costs nothing at import. */
export function onFirstUse<T extends object>(build: () => T): () => T {
  let value: T | undefined
  return () => (value ??= build())
}
