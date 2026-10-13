export function assistantFailureMessage(error: unknown): string {
  if (error instanceof Error && error.name === 'TimeoutError')
    return 'The request took too long to complete. Check the latest saved results before trying again.'
  return error instanceof Error ? error.message : 'The request could not finish.'
}
