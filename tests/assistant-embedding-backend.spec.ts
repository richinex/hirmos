import { expect, test } from '@playwright/test'

test('embedding backend requires an adapter, not just the WebGPU property', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { embeddingBackend } = await import(
      new URL('/src/assistant/memory/embeddingBackend.ts', location.href).href
    )
    return {
      absent: await embeddingBackend(undefined),
      unavailable: await embeddingBackend({ requestAdapter: async () => null }),
      denied: await embeddingBackend({
        requestAdapter: async () => {
          throw new Error('Adapter denied')
        },
      }),
      available: await embeddingBackend({ requestAdapter: async () => ({}) }),
    }
  })
  expect(result.absent).toEqual({ device: 'wasm', dtype: 'q8' })
  expect(result.unavailable).toEqual(result.absent)
  expect(result.denied).toEqual(result.absent)
  expect(result.available).toEqual({ device: 'webgpu', dtype: 'fp32' })
})
