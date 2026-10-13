export const EMBED_DIM = 384
export interface Embedder {
  readonly dim: number
  embedMany(texts: string[]): Promise<Float32Array[]>
}
export type EmbedStatus =
  | { readonly kind: 'progress'; readonly done: number; readonly total: number }
  | { readonly kind: 'error'; readonly message: string }
  | { readonly kind: 'done'; readonly count: number }
