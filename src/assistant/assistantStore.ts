import { useStore } from 'zustand'
import { createStore } from 'zustand/vanilla'

/** `n` changes on every request, so asking the same question twice still fills the composer. */
export interface AssistantSeed {
  readonly text: string
  readonly n: number
}

export type AssistantPlacement = 'docked' | 'floating'

interface AssistantState {
  readonly open: boolean
  readonly placement: AssistantPlacement
  /** On a phone, whether the assistant has the whole screen rather than the lower part. */
  readonly expanded: boolean
  readonly running: boolean
  readonly seed: AssistantSeed | null
}

const store = createStore<AssistantState>(() => ({
  open: false,
  placement: 'docked',
  expanded: false,
  running: false,
  seed: null,
}))

export const useAssistantOpen = (): boolean => useStore(store, (state) => state.open)
export const useAssistantPlacement = (): AssistantPlacement =>
  useStore(store, (state) => state.placement)
export const floatAssistant = (): void => store.setState({ placement: 'floating' })
export const dockAssistant = (): void => store.setState({ placement: 'docked' })
export const useAssistantExpanded = (): boolean => useStore(store, (state) => state.expanded)
export const toggleAssistantExpanded = (): void =>
  store.setState((state) => ({ expanded: !state.expanded }))
export const useAssistantRunning = (): boolean => useStore(store, (state) => state.running)
export const useAssistantSeed = (): AssistantSeed | null => useStore(store, (state) => state.seed)

export const toggleAssistant = (): void => store.setState((state) => ({ open: !state.open }))
export const closeAssistant = (): void => store.setState({ open: false })

/** Opens the panel with the question in the composer; the user still sends it. */
export const askAssistant = (text: string): void =>
  store.setState((state) => ({ open: true, seed: { text, n: (state.seed?.n ?? 0) + 1 } }))

export const clearAssistantSeed = (): void => store.setState({ seed: null })

export const setAssistantRunning = (running: boolean): void => store.setState({ running })
