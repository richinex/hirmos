import { expect, test } from '@playwright/test'

test('preprocessing uses consistent variable colours without changing data or diagnostic marks', async ({ page }) => {
  await page.goto('/')
  const result = await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [theme, changes, overview, histogram, scatter, decomposition] = await Promise.all([
      load('/src/charts/theme.ts'), load('/src/charts/sensitivity/changePoints.ts'),
      load('/src/charts/data/seriesOverview.ts'), load('/src/charts/data/histogram.ts'),
      load('/src/charts/data/pairwiseScatter.ts'), load('/src/charts/data/decomposition.ts'),
    ])
    return ['light', 'dark'].map((name) => {
      document.documentElement.dataset.theme = name
      const palette = theme.readChartTheme()
      const variable = 'bugs_per_kloc'
      const prepared = changes.changePointsOption({ name: variable, values: [1, 2, 3], changePoints: [2], stepLabel: 'row', zoom: false }, palette).baseOption
      const source = overview.seriesOverviewOption({ name: variable, values: new Float64Array([1, NaN, 3]), stepLabel: 'row' }, palette).baseOption
      const distribution = histogram.histogramOption({ name: variable, bins: { edges: [1, 2, 3], counts: [2, 1] }, nullCount: 0 }, palette)
      const points = scatter.pairwiseScatterOption({ xName: 'x', yName: variable, x: [1, 2], y: [2, 3], correlation: 1 }, palette)
      const stl = decomposition.decompositionOption({ name: variable, time: [1, 2], calendar: false, observed: [1, 2], trend: [1, 2], seasonal: [0, 0], remainder: [0, 0] }, palette)
      return {
        expected: theme.variableColour(palette, variable),
        colours: [prepared.series[0].lineStyle.color, source.series[0].lineStyle.color, distribution.series[0].itemStyle.color, points.series[0].itemStyle.color, stl.series[0].lineStyle.color],
        variables: ['ai_active', 'legacy_bugs_per_kloc', variable].map((key) => theme.variableColour(palette, key)),
        reversed: [variable, 'legacy_bugs_per_kloc', 'ai_active'].map((key) => theme.variableColour(palette, key)),
        marker: prepared.series[0].markLine.lineStyle.color, signal: palette.signal,
        source: source.series[0].data, connectNulls: source.series[0].connectNulls,
        prepared: prepared.series[0].data, bins: distribution.series[0].data,
      }
    })
  })
  for (const item of result) {
    expect(item.colours).toEqual(Array(5).fill(item.expected))
    expect(new Set(item.variables).size).toBe(3)
    expect(item.reversed).toEqual([...item.variables].reverse())
    expect(item.marker).toBe(item.signal)
    expect(item.source).toEqual([[1, 1], [2, null], [3, 3]])
    expect(item.connectNulls).toBe(false)
    expect(item.prepared).toEqual([[1, 1], [2, 2], [3, 3]])
    expect(item.bins).toEqual([2, 1])
  }
  expect(result[0]!.expected).not.toBe(result[1]!.expected)
})

test('categorical chart colours follow the theme and retain term identity', async ({ page }) => {
  await page.goto('/')
  const result = await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [theme, regression, curves, intervention] = await Promise.all([
      load('/src/charts/theme.ts'), load('/src/charts/survival/regression.ts'), load('/src/charts/survival/curves.ts'), load('/src/charts/dag/interventionBars.ts'),
    ])
    return ['light', 'dark'].map((name) => {
      document.documentElement.dataset.theme = name
      const palette = theme.readChartTheme()
      const points = [[0, 0, 0, 0], [1, -1, -2, 1]]
      const aalen = regression.aalenCurveOption('term', points, palette, 5)
      const repeated = regression.aalenCurveOption('term', points, palette, 13)
      const groups = [{ name: 'A', points: [[0, 1], [1, 0.8]] }, { name: 'B', points: [[0, 1], [1, 0.6]] }]
      const survival = curves.survivalCurvesOption(groups, 'days', palette)
      const hazard = curves.comparisonMeasureOption(groups, 'cumulative hazard', palette, true)
      const restricted = curves.restrictedMeanOption(groups, 1, palette)
      const bars = intervention.interventionBarsOption({ set: 'X', read: 'Y', states: ['0', '1'], low: [0.8, 0.2], high: [0.6, 0.4] }, palette)
      const importance = regression.importanceOption([{ name: 'A', value: 2 }, { name: 'B', value: -1 }], palette)
      return { name, colours: palette.categorical, line: aalen.series[2].lineStyle.color, band: aalen.series[1].areaStyle.color, repeated: repeated.series[2].lineStyle.color,
        groups: [survival, hazard, restricted].map((o) => o.series.map((s: any) => s.lineStyle.color)),
        bars: bars.series.map((s: any) => s.itemStyle.color),
        importance: importance.series[0].data.map((d: any) => d.itemStyle.color),
        signedBand: aalen.series[1].stackStrategy, lower: aalen.series[0].data,
      }
    })
  })
  for (const item of result) {
    expect(item.line).toBe(item.colours[5])
    expect(item.band).toBe(item.line)
    expect(item.repeated).toBe(item.line)
    for (const group of item.groups) expect(group).toEqual(item.colours.slice(0, 2))
    expect(item.bars).toEqual(item.colours.slice(0, 2))
    expect(item.importance).toEqual([item.colours[1], item.colours[0]])
    expect(item.signedBand).toBe('all')
    expect(item.lower).toEqual([[0, 0], [1, -2]])
  }
  expect(result[0]!.colours).not.toEqual(result[1]!.colours)
})
