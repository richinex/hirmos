import type { Citations, RlmState } from './types'

export interface CodeExecResult {
  readonly stdout: string
  readonly stderr: string
  readonly exit_code: number
  readonly answer: { readonly content: unknown; readonly ready: boolean; readonly citations?: unknown[] }
  readonly locals: Record<string, unknown>
}

export interface CodeActExecutor {
  execute(code: string): Promise<CodeExecResult>
  resetAnswer(): void
}

export interface CodeActMessage {
  role: 'user' | 'assistant'
  content: unknown
}

export interface CodeActReply {
  text: string
  content: unknown[]
  stop_reason: string
  usage?: { output_tokens: number }
}

export type CodeActCitationValidation =
  | { readonly kind: 'uncited' }
  | { readonly kind: 'rejected'; readonly invalid: Citations }
  | { readonly kind: 'validationFailed'; readonly message: string }
  | { readonly kind: 'grounded'; readonly citations: Citations }

export interface CodeActLoopOptions {
  readonly system: string
  readonly maxIters?: number
  readonly maxTokens?: number
  readonly signal?: AbortSignal
  readonly onState?: (state: RlmState) => void
  readonly acquireExecutor: () => Promise<CodeActExecutor>
  readonly callModel: (request: {
    system: string
    messages: CodeActMessage[]
    maxTokens: number
    signal?: AbortSignal
    onText: (text: string) => void
  }) => Promise<CodeActReply>
  readonly validateCitations: (raw: unknown) => Promise<CodeActCitationValidation>
  readonly guidance: {
    finish: string
    uncited: string
    rejected: string
  }
}
