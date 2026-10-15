import { useStore } from 'zustand'
import { createStore } from 'zustand/vanilla'
import { DEFAULT_LIMITS } from './limits'

interface ConnectionSettings {
  readonly apiKey: string
  readonly model: string
  readonly maxIters: string
  readonly maxRequests: string
  readonly maxTokens: string
}

// Credentials survive project remounts, but never leave tab memory through persistence.
const connection = createStore<
  ConnectionSettings & { readonly approvedProjects: ReadonlySet<string> }
>(() => ({
  apiKey: '',
  model: '',
  maxIters: String(DEFAULT_LIMITS.maxIters),
  maxRequests: String(DEFAULT_LIMITS.maxRequests),
  maxTokens: String(DEFAULT_LIMITS.maxTokens),
  approvedProjects: new Set(),
}))

export const useAssistantConnection = () => useStore(connection)
export const configureAssistantConnection = (settings: Partial<ConnectionSettings>) =>
  connection.setState(settings)
export const approveAssistantProject = (project: string) =>
  connection.setState((state) => ({
    approvedProjects: new Set([...state.approvedProjects, project]),
  }))
export const forgetAssistantConnection = () =>
  connection.setState({ apiKey: '', approvedProjects: new Set() })
