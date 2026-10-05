/** Why a durable project exists; an example stamp survives edits to that copy. */
export type ProjectOrigin =
  { readonly kind: 'user' } | { readonly kind: 'shipped-example'; readonly exportedAt: string }
