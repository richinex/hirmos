import { expect, test } from '@playwright/test'

/**
 * The DESIGN.md §6.4 evidence rules, table-driven over hand-built batteries. Runs in the browser
 * against the real domain module, as the DAG flow corpus does.
 */
test('routes each ADF, KPSS and Zivot–Andrews combination to the documented assessment', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Domain table runs once')
  await page.goto('/app')
  const results: unknown = await page.evaluate(async () => {
    const module = await import(new URL('/src/domain/stationarityAssessment.ts', window.location.href).href)
    const unitRoot = { statistic: 0, pValue: 0.5, usedLag: 1, observations: 100, criticalValues: { onePercent: -3.5, fivePercent: -2.9, tenPercent: -2.6 } }
    const test = (pValue: number) => ({ ...unitRoot, pValue })
    const za = (pValue: number, breakIndex = 40) => ({ statistic: -4, pValue, criticalValues: { onePercent: -5.3, fivePercent: -4.8, tenPercent: -4.6 }, baseLags: 1, breakIndex })
    const battery = (spec: { adfC: number; kpssC: number; adfCt?: number; kpssCt?: number; za?: number }) => ({
      observations: 100,
      adf: { constant: test(spec.adfC), constantAndTrend: test(spec.adfCt ?? spec.adfC) },
      kpss: { constant: test(spec.kpssC), constantAndTrend: test(spec.kpssCt ?? spec.kpssC) },
      zivotAndrews: { level: za(spec.za ?? 0.5), trend: za(spec.za ?? 0.5), levelAndTrend: za(spec.za ?? 0.5) },
    })
    const stationary = battery({ adfC: 0.01, kpssC: 0.3 })
    const integrated = battery({ adfC: 0.6, kpssC: 0.01 })
    const cases = {
      levelStationary: module.assessStationarity(stationary, null),
      trendStationary: module.assessStationarity(battery({ adfC: 0.4, kpssC: 0.01, adfCt: 0.01, kpssCt: 0.2 }), null),
      breakStationary: module.assessStationarity(battery({ adfC: 0.2, kpssC: 0.2, za: 0.01 }), null),
      differenceStationary: module.assessStationarity(integrated, stationary),
      higherOrder: module.assessStationarity(integrated, integrated),
      differenceMissing: module.assessStationarity(integrated, null),
      bothReject: module.assessStationarity(battery({ adfC: 0.01, kpssC: 0.01 }), null),
      neitherRejects: module.assessStationarity(battery({ adfC: 0.3, kpssC: 0.3 }), null),
      breakAfterUnitRoot: module.assessStationarity(battery({ adfC: 0.6, kpssC: 0.01, za: 0.02 }), battery({ adfC: 0.3, kpssC: 0.3 })),
    }
    return Object.fromEntries(Object.entries(cases).map(([name, assessment]) => {
      const level = module.levelModelVerdict('x', assessment)
      return [name, { kind: assessment.kind, conflicts: assessment.conflicts?.map((conflict: { kind: string }) => conflict.kind), model: assessment.model, break: assessment.break, verdict: module.describeStationarityAssessment(assessment).verdict, level: level.kind, levelReason: level.reason }]
    }))
  })
  const r = results as Record<string, { kind: string; conflicts?: string[]; model?: string; break?: number; verdict: string; level: string; levelReason: string }>

  expect(r.levelStationary).toMatchObject({ kind: 'levelStationary', level: 'allowed' })
  expect(r.trendStationary).toMatchObject({ kind: 'trendStationary', level: 'allowed' })
  expect(r.breakStationary).toMatchObject({ kind: 'breakStationary', model: 'level', break: 40, verdict: 'break-stationary (level break at row 41)', level: 'unresolved', levelReason: 'x is stationary only around a break at row 41; the model does not include it.' })
  expect(r.differenceStationary).toMatchObject({ kind: 'differenceStationary', verdict: 'I(1), difference-stationary', level: 'refused' })
  expect(r.higherOrder).toMatchObject({ kind: 'higherOrderOrUnresolved', level: 'refused' })
  expect(r.differenceMissing).toMatchObject({ kind: 'inconclusive', conflicts: ['difference-battery-missing'], level: 'unresolved' })
  expect(r.bothReject).toMatchObject({ kind: 'inconclusive', conflicts: ['both-reject', 'both-reject'] })
  expect(r.neitherRejects).toMatchObject({ kind: 'inconclusive', conflicts: ['neither-rejects', 'neither-rejects'] })
  expect(r.breakAfterUnitRoot).toMatchObject({ kind: 'breakStationary' })
})
