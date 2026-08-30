import { expect, test } from '@playwright/test'

test('eligibility preserves every pre-run evaluation and does not overstate discrete-network checks', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain contract runs once')
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const estimation = await import(new URL('/src/domain/estimation.ts', window.location.href).href)
    const methods = await import(new URL('/src/domain/methods.ts', window.location.href).href)
    const treatment = { node: 'treatment', column: 'treatment', name: 'Treatment' }
    const outcome = { node: 'outcome', column: 'outcome', name: 'Outcome' }
    const confounder = { node: 'confounder', column: 'confounder', name: 'Confounder' }
    const prepared = { kind: 'prepared-cross-section', observations: 614, columns: ['treatment', 'outcome', 'confounder'] }
    const study = {
      estimand: { kind: 'average-treatment-effect', scale: 'additive' },
      treatment,
      outcome,
      graph: { nodes: [treatment, outcome, confounder], laggedArrows: 0 },
    }
    const identification = { kind: 'identified', adjustment: { kind: 'canonical', variables: [confounder] } }
    const context = {
      identification,
      prepared,
      stationarity: null,
      document: null,
      study,
      panelPreflight: { kind: 'not-applicable' },
    }
    const evaluate = (estimator: 'negbin-nuts' | 'bayesian-gaussian' | 'discrete-bn-query', facts: { outcomeIsCount: boolean; treatmentIsBinary: boolean }) => {
      const definition = methods.methodDefinition(estimation.methodIdOf(estimator))
      if (!definition.ok) throw new Error(`missing method ${estimator}`)
      return estimation.evaluateEstimatorEligibility(definition.value, {
        ...context,
        ...facts,
        configuration: estimation.defaultConfiguration(estimator, prepared, study),
      })
    }
    return {
      count: evaluate('negbin-nuts', { outcomeIsCount: false, treatmentIsBinary: true }),
      gaussian: evaluate('bayesian-gaussian', { outcomeIsCount: false, treatmentIsBinary: false }),
      discrete: evaluate('discrete-bn-query', { outcomeIsCount: false, treatmentIsBinary: true }),
    }
  })

  expect(result.count.kind).toBe('refused')
  expect(result.count).toMatchObject({
    satisfied: expect.arrayContaining([expect.objectContaining({ caveat: expect.objectContaining({ id: 'nuts-model-shape' }) })]),
    unresolved: expect.arrayContaining([expect.objectContaining({ caveat: expect.objectContaining({ id: 'nuts-convergence' }) })]),
    violations: expect.arrayContaining([expect.objectContaining({ caveat: expect.objectContaining({ id: 'nuts-count-outcome' }) })]),
  })
  expect(result.gaussian.kind).toBe('refused')
  expect(result.gaussian.violations).toEqual(expect.arrayContaining([
    expect.objectContaining({ caveat: expect.objectContaining({ id: 'bayes-gaussian-binary-treatment' }) }),
  ]))
  expect(result.discrete.kind).toBe('caution')
  expect(result.discrete.unresolved.map((entry: { caveat: { id: string } }) => entry.caveat.id)).toEqual(expect.arrayContaining([
    'bn-sample-per-cell',
    'bn-independent-rows',
  ]))
})

test('estimator chips use the prepared treatment and outcome before a run', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Example workflow runs once')
  await page.goto('/app')
  const example = page.getByRole('list', { name: 'Projects' }).getByRole('listitem').filter({ hasText: 'Seat-belt law and road deaths' })
  await example.getByRole('button', { name: 'Open' }).click()
  await expect(page.getByRole('navigation', { name: 'Workspace chapters' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Estimation/ }).click()
  await expect(page.getByRole('button', { name: 'Checking treatment and outcome…' })).toBeHidden({ timeout: 30_000 })
  await expect(page.getByRole('radio', { name: /Bayesian negative binomial.*review/i })).toBeVisible()
  await expect(page.getByRole('radio', { name: /Bayesian Gaussian regression.*unavailable/i })).toBeVisible()
  await expect(page.getByRole('radio', { name: /Discrete Bayesian network do-query.*unavailable/i })).toBeVisible()
})
