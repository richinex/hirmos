/**
 * Whether the browser is still accepting writes. The project store swallows a failed write so the session
 * keeps working in memory, but it reports it here so the shell can say the project is not saved. A failed
 * save must never look saved (DESIGN.md §5.6).
 */

export interface StorageFailure {
  readonly store: string
  readonly reason: string
  readonly at: string
}

type Listener = (failure: StorageFailure | null) => void

const listeners = new Set<Listener>()
let last: StorageFailure | null = null

export const reportStorageFailure = (failure: StorageFailure): void => {
  last = failure
  for (const listener of listeners) listener(failure)
}

export const reportStorageRecovered = (): void => {
  if (last === null) return
  last = null
  for (const listener of listeners) listener(null)
}

export const lastStorageFailure = (): StorageFailure | null => last

export const subscribeStorageHealth = (listener: Listener): (() => void) => {
  listeners.add(listener)
  return () => {
    listeners.delete(listener)
  }
}
