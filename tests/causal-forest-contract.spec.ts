import { expect, test } from '@playwright/test'

test('forest targets, unavailable estimates and intervals retain their meaning', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const domain = await import(new URL('/src/domain/causalForest.ts', location.href).href)
    const chart = await import(new URL('/src/charts/estimation/causalForest.ts', location.href).href)
    const themes = await import(new URL('/src/charts/theme.ts', location.href).href)
    const evidence = {
      target: { kind: 'binary-conditional' }, confidenceLevel: 0.95, observations: 3, fitted: domain.DEFAULT_CAUSAL_FOREST,
      predictions: [
        { kind: 'estimated', estimate: 2, uncertainty: { kind: 'estimated', standardError: 1, interval: { lower: 0.04, upper: 3.96 } } },
        { kind: 'unavailable', reason: 'No contributing trees.' },
        { kind: 'estimated', estimate: -1, uncertainty: { kind: 'unavailable', reason: 'Too few tree groups.' } },
      ],
      summary: { kind: 'not-requested' }, calibration: { kind: 'unavailable', reason: 'Singular calibration design.' }, variableImportance: [0.7, 0.3],
    }
    const valid = domain.parseCausalForestEvidence(evidence)
    const configurations = [domain.DEFAULT_CAUSAL_FOREST,
      { ...domain.DEFAULT_CAUSAL_FOREST, sampleFraction: 0.7 },
      { ...domain.DEFAULT_CAUSAL_FOREST, honesty: { kind: 'disabled', fraction: 0.5 } },
      { ...domain.DEFAULT_CAUSAL_FOREST, groupSize: 1 },
      { ...domain.DEFAULT_CAUSAL_FOREST, alpha: 0.3 },
    ].map(value => domain.causalForestConfigurationSchema.safeParse(value).success)
    const targets = [
      { kind: 'binary-average', population: 'treated' },
      { kind: 'continuous-average', population: 'treated' },
      { kind: 'continuous-average', population: 'all' },
      { kind: 'continuous-conditional', population: 'all' },
    ].map(value => domain.causalForestTargetSchema.safeParse(value).success)
    const invalid = [
      { ...evidence, predictions: evidence.predictions.slice(1) },
      { ...evidence, target: { kind: 'binary-average', population: 'all' } },
      { ...evidence, confidenceLevel: 1 },
      { ...evidence, features: [{kind:'numeric',column:'x',name:'X'}] },
      { ...evidence, features: [{kind:'indicator',column:'x',name:'X'}, {kind:'numeric',column:'y',name:'Y'}] },
      { ...evidence, predictions: [{ kind: 'estimated', estimate: NaN, uncertainty: { kind: 'unavailable', reason: 'test' } }, ...evidence.predictions.slice(1)] },
    ].map(value => domain.parseCausalForestEvidence(value).ok)
    const plots = ['light', 'dark'].map(name => {
      document.documentElement.dataset.theme = name
      const theme = themes.readChartTheme()
      const option = chart.causalForestPredictionsOption(evidence, '<outcome>', theme)
      return { animation: option.animation, points: option.series[1].data, intervals: option.series[0].data,
        connect: option.series[0].connectNulls, colour: option.series[1].itemStyle.color, expectedColour: theme.signal,
        zero: option.series[1].markLine.data, tooltip: option.tooltip.formatter({ value: [3, -1] }),
        description: option.aria.label.description, label: option.series[1].name }
    })
    return { valid: valid.ok, configurations, targets, invalid, plots,
      continuousLabel: domain.describeCausalForestTarget({ kind: 'continuous-average', population: 'all' }),
      conditionalLabel: domain.describeCausalForestTarget({ kind: 'binary-conditional' }) }
  })
  expect(result.valid).toBe(true)
  expect(result.configurations).toEqual([true, false, false, false, true])
  expect(result.targets).toEqual([true, false, true, false])
  expect(result.invalid).toEqual([false, false, false, false, false, false])
  expect(result.continuousLabel).toBe('Average partial effect')
  expect(result.conditionalLabel).toBe('Conditional average treatment effects')
  for (const plot of result.plots) {
    expect(plot.animation).toBe(false)
    expect(plot.points).toEqual([[1, 2], [3, -1]])
    expect(plot.intervals).toEqual([[1, 0.04], [1, 3.96], null])
    expect(plot.connect).toBe(false)
    expect(plot.colour).toBe(plot.expectedColour)
    expect(plot.zero).toEqual([{ yAxis: 0 }])
    expect(plot.tooltip).toContain('Interval unavailable: Too few tree groups.')
    expect(plot.description).toContain('1 unavailable predictions')
    expect(plot.description).toContain('pointwise, not simultaneous')
    expect(plot.label).toBe('Conditional average treatment effect')
  }
})
