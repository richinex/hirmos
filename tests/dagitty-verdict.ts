import { expect, type Locator } from '@playwright/test'

/** What dagitty says about a fixture graph's back-door adjustment. */
export interface ExpectedVerdict {
  readonly backdoorOpen: boolean
  readonly msas: readonly (readonly string[])[]
  readonly canonical: readonly string[]
}

/**
 * Checks the Adjustment panel against dagitty by the case it shows and the variables it names,
 * never by its wording. Returns true when the graph needs an adjustment set and Hirmos named one.
 */
export async function expectVerdict(adjustment: Locator, expected: ExpectedVerdict): Promise<boolean> {
  const shown = adjustment.locator('[data-adjustment]')
  if (!expected.backdoorOpen) {
    await expect(shown).toHaveAttribute('data-adjustment', 'unnecessary')
    return false
  }
  if (expected.msas.length === 0) {
    await expect(shown).toHaveAttribute('data-adjustment', 'none')
    return false
  }
  await expect(shown).toHaveAttribute('data-adjustment', 'sufficient')
  const named = await shown.locator('[data-adjustment-variables]').innerText()
  expect(named.split(', ').sort()).toEqual([...expected.canonical].sort())
  return true
}
