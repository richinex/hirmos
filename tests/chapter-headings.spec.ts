import { expect, test } from '@playwright/test'

const chapters = [
  { nav: 'Discovery lab', title: 'Causal discovery' },
  { nav: 'DAG workspace', title: 'DAG workspace' },
  { nav: 'Study design', title: 'Study design' },
  { nav: 'Estimation', title: 'Estimation' },
  { nav: 'Sensitivity', title: 'Sensitivity' },
  { nav: 'Counterfactuals', title: 'Counterfactuals' },
  { nav: 'Survival analysis', title: 'Survival analysis' },
  { nav: 'Results', title: 'Results' },
] as const

for (const example of ['A simulated process with a collider', 'Deploys and incidents at a lag']) {
  test(`consistent chapter headings: ${example}`, async ({ page }, info) => {
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${example}`, exact: true }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
    const targets = example.startsWith('Deploys')
      ? [{ nav: 'Time-series analysis', title: 'Time-series analysis' }]
      : chapters
    for (const chapter of targets) {
      if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
      await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: new RegExp(chapter.nav) }).click()
      const heading = page.getByRole('heading', { level: 2, name: chapter.title, exact: true })
      await expect(heading).toBeVisible()
      if (chapter.nav === 'Study design') {
        await expect(page.getByText('A causal question specifies the treatment, outcome, intervention contrast, effect measure, and target population. In this section, you represent the data-generating process with a DAG, enumerate measured back-door adjustment sets, and run the ID algorithm to determine whether the interventional distribution can be written using observed probabilities.', { exact: true })).toBeVisible()
      }
      if (chapter.nav === 'Estimation') {
        await expect(page.getByText('Identification determines how to express the causal question using observed data. Estimation applies a statistical method to that expression. In this section, you choose a compatible estimator and examine the effect estimate, its uncertainty, and the method-specific diagnostics.', { exact: true })).toBeVisible()
      }
      await expect(page.getByText(/In this chapter/)).toHaveCount(0)
      await expect(heading.locator('..')).not.toContainText(/\d{2} · /)
      expect(await heading.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
      for (const theme of ['light', 'dark']) {
        await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
        await page.screenshot({ path: info.outputPath(`${chapter.nav}-${theme}.png`) })
      }
    }
  })
}
