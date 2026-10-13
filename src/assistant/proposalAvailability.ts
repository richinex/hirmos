export type ProposalAvailability =
  { readonly kind: 'ready' } | { readonly kind: 'blocked'; readonly reason: string }

export function proposalAvailability(running: boolean, disabled: boolean): ProposalAvailability {
  if (running)
    return {
      kind: 'blocked',
      reason: 'Wait for the answer to finish, or stop it before applying changes.',
    }
  if (disabled)
    return { kind: 'blocked', reason: 'Review the proposal requirements before applying changes.' }
  return { kind: 'ready' }
}
