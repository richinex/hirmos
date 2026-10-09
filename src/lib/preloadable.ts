type Settled<T> = Promise<T> & { status?: 'fulfilled'; value?: T }

/** Marks the promise fulfilled on resolve, so use() reads it without suspending. */
export function preloadable<T>(load: () => Promise<T>): () => Promise<T> {
  let promise: Settled<T> | undefined
  return () =>
    (promise ??= load().then((value) => {
      promise!.status = 'fulfilled'
      promise!.value = value
      return value
    }))
}
