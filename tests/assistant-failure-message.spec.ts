import { expect, test } from '@playwright/test'
import { assistantFailureMessage } from '../src/assistant/failureMessage'

test('an expired request explains the timeout without implying that saved work was undone', async () => {
  const signal = AbortSignal.timeout(1)
  await new Promise<void>((resolve) =>
    signal.addEventListener('abort', () => resolve(), { once: true }),
  )
  expect(assistantFailureMessage(signal.reason)).toBe(
    'The request took too long to complete. Check the latest saved results before trying again.',
  )
  expect(assistantFailureMessage(new Error('Invalid column.'))).toBe('Invalid column.')
})
