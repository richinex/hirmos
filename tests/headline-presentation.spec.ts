import { expect, test } from '@playwright/test'

test('staggered subtitles describe the recorded average, including older event-time runs', async ({
  page,
}) => {
  await page.goto('/app')
  const lines = await page.evaluate(async () => {
    const { resultScaleLine } = await import(
      new URL('/src/domain/resultInterpretation.ts', location.href).href
    )
    const study = { treatment: { name: 'adoption' }, outcome: { name: 'employment' } }
    return ['dynamic', 'group', 'calendar', 'simple', undefined].map((headline) =>
      resultScaleLine(
        {
          kind: 'panel-intervention-run',
          evidence: { kind: 'staggeredDid' },
          configuration: {
            kind: 'panel-intervention',
            primary: 'staggered',
            ...(headline === undefined ? {} : { headline }),
          },
        },
        study,
        'year',
      ),
    )
  })
  expect(lines).toEqual([
    'The headline is the equal-weight average of supported post-adoption event-time effects.',
    "The headline averages each cohort's post-adoption effects, then weights the cohorts by size.",
    'The headline is the equal-weight average of the effects in each calendar period after the first adoption.',
    "The headline averages every post-adoption group-time effect, weighting each by its cohort's size.",
    'The headline is the equal-weight average of supported post-adoption event-time effects.',
  ])
})

test('a negative headline keeps visible signs and speaks all three negative values consistently', async ({
  page,
}) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const { default: React } = await import(new URL('/node_modules/.vite/deps/react.js', location.href).href)
    const { default: ReactDOM } = await import(
      new URL('/node_modules/.vite/deps/react-dom_client.js', location.href).href
    )
    const { IntervalFigure } = await import(
      new URL('/src/components/ui/figures.tsx', location.href).href
    )
    const host = document.createElement('div')
    document.body.append(host)
    ReactDOM.createRoot(host).render(
      React.createElement(IntervalFigure, {
        sentence: 'Average treatment effect',
        estimate: -2.3,
        lower: -2.94,
        upper: -1.66,
        type: { kind: 'confidence', level: 0.95 },
        scale: { kind: 'additive', unit: '' },
        sampleLine: '',
        scaleLine: '',
        testId: 'negative-headline',
      }),
    )
  })
  const headline = page.getByTestId('negative-headline')
  await expect(headline.locator('.sr-only').first()).toHaveText(
    'minus 2.30; 95% confidence interval minus 2.94 to minus 1.66',
  )
  await expect(headline.locator('span[aria-hidden="true"]').first()).toHaveText('−2.30')
  await expect(headline.locator('p[aria-hidden="true"]')).toContainText('[−2.94, −1.66] 95% CI')
})
