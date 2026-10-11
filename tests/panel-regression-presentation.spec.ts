import { expect, test } from '@playwright/test'

test('event-study presentation shows estimates before sample bookkeeping', async ({
  page,
}, info) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const { React, createRoot } = await import(
      new URL('/tests/support/reactRuntime.ts', location.href).href
    )
    const { PanelRegressionResult } = await import(
      new URL('/src/components/time-series/PanelRegressionResult.tsx', location.href).href
    )
    const terms = [-2, 0, 1, 2].map((period, index) => ({
      index,
      name: `Event ${period}`,
      estimate: index === 0 ? 0.1 : -0.3 * index,
      standardError: 0.1,
      degreesOfFreedom: 9,
      lower: (index === 0 ? 0.1 : -0.3 * index) - 0.22,
      upper: (index === 0 ? 0.1 : -0.3 * index) + 0.22,
      pValue: 0.05,
    }))
    const run = {
      kind: 'panel-regression',
      outcome: { name: 'Outcome' },
      evidence: {
        request: {
          confidence: 0.95,
          specification: {
            kind: 'eventStudy',
            window: { first: -2, last: 2, reference: -1, tails: 'bin' },
          },
        },
        observations: 1200,
        clusters: 10,
        terms,
        omitted: [],
        events: [-2, 0, 1, 2].map((period, index) => ({ index, period, support: 100 })),
        leadTest: {
          kind: 'recorded',
          statistic: 1,
          pValue: 0.343,
          numeratorDf: 1,
          denominatorDf: 9,
        },
      },
    }
    const host = document.createElement('main')
    host.style.cssText =
      'max-width: 960px; margin: auto; padding: 16px; height: 100vh; overflow: auto'
    document.body.replaceChildren(host)
    createRoot(host).render(React.createElement(PanelRegressionResult, { run }))
  })
  await expect(
    page.getByRole('heading', { name: 'Regression event study for Outcome' }),
  ).toBeVisible()
  for (const theme of ['light', 'dark']) {
    await page.evaluate((theme) => {
      document.documentElement.dataset.theme = theme
    }, theme)
    await page.screenshot({ path: info.outputPath(`event-study-${theme}.png`) })
  }
  await expect(page.getByLabel('Regression summary', { exact: true })).toHaveCount(0)
  await expect(page.getByLabel('Regression sample', { exact: true })).toContainText(
    '1,200 observations',
  )
  await expect(page.getByLabel('Regression sample', { exact: true })).toContainText('10 clusters')
  await expect(
    page.getByRole('table', { name: 'Regression coefficients', exact: true }),
  ).toContainText('Event 2')
})
