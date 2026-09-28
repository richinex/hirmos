import { readFileSync } from 'node:fs'
import { expect, test } from '@playwright/test'
import katex from 'katex'
import { survivalDistributionEquation, survivalEquations, survivalEquationVariables } from '../src/domain/survivalEquations'
import type { ParametricSurvivalFamily, SurvivalRunArtifact } from '../src/domain/survival'
import { parseBundle } from '../src/domain/bundle'

const example = (name: string): SurvivalRunArtifact => {
  const parsed = parseBundle(readFileSync(new URL(`../public/examples/${name}.hirmos.json`, import.meta.url), 'utf8'))
  if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
  return parsed.value.project.survivalRuns[0]!
}
const breast = example('breast-cancer-survival')
const adoption = example('feature-adoption-survival')
const crossing = example('crossing-survival-curves')

// Partial records isolate the equation's inputs. Production supplies validated saved run artifacts.
const run = (fields: object): SurvivalRunArtifact => ({ ...breast, ...fields }) as SurvivalRunArtifact
const coefficient = (value: number) => ({ coefficient: value })
const names = [{ id: 'a', name: 'AI_% {use} \\href{bad}' }, { id: 'b', name: 'experience' }]
const cox = (varying = false, frailty = false): SurvivalRunArtifact => run({
  kind: 'cox-regression-run',
  configuration: { observation: { kind: varying ? 'start-stop' : 'right-censored' }, strata: { kind: 'column', column: { name: 'department' } }, covariates: names },
  evidence: { coefficients: [coefficient(-0.223), coefficient(1e-10)], covariateMeans: [0.5, 4], fitting: { kind: frailty ? 'gammaFrailty' : 'efron' }, frailty: frailty ? { kind: 'gamma', theta: 0.3 } : { kind: 'none' } },
})

const models: readonly [ParametricSurvivalFamily, readonly number[], number, string][] = [
  ['exponential', [0.23], 0, 'log'], ['weibull', [1.7, 4.2], 1, 'log'],
  ['weibullPh', [1.7, 0.12], 1, 'log'], ['logNormal', [1.1, 0.65], 0, 'identity'],
  ['gamma', [2.4, 0.7], 1, 'log'], ['gompertz', [0, 0.12], 1, 'log'],
  ['logLogistic', [1.8, 3.4], 1, 'log'], ['generalizedGamma', [1.1, 0.7, -0.45], 0, 'identity'],
  ['generalizedF', [1.1, 0.7, -0.35, 0.6], 0, 'identity'],
]

test('all nine distributions use the flexsurv location link and baseline order', () => {
  for (const [family, naturalBaseline, location, link] of models) {
    const spec = survivalDistributionEquation(family)
    expect(spec.location).toBe(location)
    expect(spec.link).toBe(link)
    expect(spec.parameters).toHaveLength(naturalBaseline.length)
    const result = survivalEquations(run({ configuration: { covariates: names }, evidence: { family, naturalBaseline, coefficients: [-0.4, 0.2] } }))
    expect(result.fitted[0]!.plain).toBe('eta = (-0.4) × x1 + (0.2) × x2')
    for (const f of [...result.general, ...result.fitted]) expect(() => katex.renderToString(f.tex, { throwOnError: true, strict: 'error' })).not.toThrow()
  }
  expect(survivalDistributionEquation('weibull').survival.plain).toContain('(t/s(x))^k')
  expect(survivalDistributionEquation('weibullPh').survival.plain).toContain('m(x)t^k')
  expect(survivalDistributionEquation('gompertz').definitions.join(' ')).toContain('when a is zero')
})

test('the Weibull equations reproduce all saved curve points at the recorded profile', () => {
  for (const saved of [breast, adoption]) {
    if (saved.kind !== 'right-censored-survival-run' && saved.kind !== 'start-stop-survival-run') throw new Error('Wrong example family')
    const e = saved.evidence
    const eta = e.coefficients.reduce((sum, value, i) => sum + value * e.profile[i]!, 0)
    const scale = e.naturalBaseline[1]! * Math.exp(eta)
    e.predictionTimes.forEach((time, i) => {
      const survival = e.family === 'weibull' ? Math.exp(-Math.pow(time / scale, e.naturalBaseline[0]!)) : Math.exp(-scale * Math.pow(time, e.naturalBaseline[0]!))
      expect(survival).toBeCloseTo(e.survival[i]!, 12)
    })
  }
})

test('the start-stop likelihood is typeset separately and conditions on survival to entry', () => {
  const detail = survivalEquations(adoption).definitions.find((definition) => typeof definition !== 'string')
  if (detail === undefined || typeof detail === 'string') throw new Error('Missing interval likelihood')
  expect(detail.formula.tex).toContain('\\frac{S(t_i\\mid x_i)}{S(a_i\\mid x_i)}')
  expect(() => katex.renderToString(detail.formula.tex, { throwOnError: true, strict: 'error' })).not.toThrow()
})

test('Cox uses saved centring, strata, time-varying values and tiny coefficients', () => {
  const result = survivalEquations(cox(true))
  expect(result.general[0]!.tex).toContain('h_{0,s}')
  expect(result.fitted[0]!.plain).toBe('eta(t) = (-0.223) × (x1(t) - 0.5) + (1e-10) × (x2(t) - 4)')
  expect(result.fitted[0]!.tex).toContain('10^{-10}')
  expect(survivalEquationVariables(cox())).toEqual(names.map((n) => n.name))
  expect(JSON.stringify(result)).not.toContain('\\href')
  const gamma = survivalEquations(cox(false, true))
  expect(gamma.general[0]!.tex).toContain('z_g')
  expect(gamma.definitions.join(' ')).toContain('leaves z_g unspecified')
})

test('Aalen substitutes cumulative curves, never weighted summary coefficients', () => {
  const result = survivalEquations(run({ kind: 'aalen-run', evidence: { lastTime: 5, coefficients: [[999], [888]], curves: [[[0, 0], [5, 0.2]], [[0, 0], [5, -0.3]]] } }))
  expect(result.fitted[0]!.plain).toBe('H(5 | x) = 0.2 + (-0.3) × x1')
  expect(JSON.stringify(result)).not.toContain('999')
})

test('both penalised AFT families exponentiate the ancillary coefficient', () => {
  for (const family of ['weibull', 'logLogistic']) {
    const result = survivalEquations(run({ kind: 'penalized-aft-run', evidence: { family, coefficients: [coefficient(-0.4)], intercept: coefficient(1.2), ancillary: coefficient(Math.log(2)) } }))
    expect(result.fitted[1]!.plain).toBe('log s(x) = 1.2 + eta')
    expect(result.fitted[2]!.plain).toBe('k = 2')
  }
})

test('nonparametric, forest and multi-state equations preserve their distinct meanings', () => {
  const discrete = survivalEquations(run({ kind: 'nonparametric-survival-run', configuration: { ties: 'discrete' } }))
  const smooth = survivalEquations(run({ kind: 'nonparametric-survival-run', configuration: { ties: 'smoothed' } }))
  expect(discrete.general[1]!.tex).toContain('d_i/n_i')
  expect(smooth.general[1]!.tex).toContain('(n_i-k)^{-1}')
  const forest = survivalEquations(run({ kind: 'survival-forest-run', evidence: { trees: 20, predictionRow: 3 } }))
  expect(forest.general[0]!.tex).toContain('B^{-1}')
  expect(forest.definitions[0]).toContain('row 4')
  const multi = survivalEquations(run({ kind: 'multi-state-survival-run', evidence: { family: 'weibullPh' } }))
  expect(multi.general[0]!.plain).toContain('P(t) Q(t)')
  expect(multi.fitted).toEqual([])
  for (const result of [discrete, smooth, forest, multi, survivalEquations(crossing), survivalEquations(cox(true)), survivalEquations(cox(false, true))]) {
    for (const f of [...result.general, ...result.fitted]) expect(() => katex.renderToString(f.tex, { throwOnError: true, strict: 'error' })).not.toThrow()
  }
})

test('fixed-time conversion differences keep their sign and probability scale', () => {
  const saved = structuredClone(crossing)
  if (saved.kind !== 'two-group-survival-run') throw new Error('Wrong example')
  const result = survivalEquations(run({ ...saved, evidence: { ...saved.evidence, fixedTimeConversion: { kind: 'recorded', result: { time: 7, difference: -0.09 } } } }))
  expect(result.fitted.at(-1)!.plain).toBe('estimated Delta_F = -0.09')
  expect(result.general.at(-1)!.plain).toContain('(1 − S1(t)) − (1 − S0(t))')
})

test('the equation expands in the existing result and remains readable on both screen sizes', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: /Open Breast cancer/i }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Survival analysis/ }).click()
  const equation = page.getByTestId('survival-equation').first()
  await expect(equation).not.toHaveAttribute('open', '')
  await equation.getByText('Model equation', { exact: true }).click()
  await expect(equation.getByText('Fitted model', { exact: true })).toBeVisible()
  await expect(equation.locator('.katex')).not.toHaveCount(0)
  await expect(equation.locator('.katex-error')).toHaveCount(0)
  expect(await page.locator('body').evaluate((el) => el.scrollWidth <= window.innerWidth + 1)).toBe(true)
  const layout = await equation.getByTestId('survival-equation-layout').boundingBox()
  const general = await equation.getByTestId('survival-equation-formulas').boundingBox()
  const fitted = await equation.getByTestId('survival-equation-fitted').boundingBox()
  if (!layout || !general || !fitted) throw new Error('Missing model layout')
  if (layout.width >= 704) {
    expect(Math.abs(general.y - fitted.y)).toBeLessThan(2)
    expect(fitted.x).toBeGreaterThan(general.x + general.width)
  } else {
    expect(fitted.y).toBeGreaterThanOrEqual(general.y + general.height)
  }
  expect(await equation.evaluate((el) => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
  await equation.screenshot({ path: info.outputPath('survival-equation.png') })
})

test('interval details remain collapsed until requested and typeset when opened', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: /Open Feature adoption/i }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Survival analysis/ }).click()
  const equation = page.getByTestId('survival-equation').first()
  await equation.getByText('Model equation', { exact: true }).click()
  const detail = equation.getByTestId('survival-equation-detail')
  await expect(detail).not.toHaveAttribute('open', '')
  await detail.locator('summary').click()
  await expect(detail.locator('.katex')).toHaveCount(1)
  await expect(detail.locator('.katex-error')).toHaveCount(0)
  expect(await equation.evaluate((el) => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
  await equation.screenshot({ path: info.outputPath('interval-equation.png') })
})
