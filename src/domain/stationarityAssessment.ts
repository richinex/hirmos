import { assertNever, type NonEmptyArray } from './dop'
import type { StationarityBattery } from './stationarity'

/**
 * The interpreted stationarity route per DESIGN.md §6.4: evidence rules over ADF and KPSS in `c` and
 * `ct`, Zivot–Andrews for a one-time break, and the same battery on the first difference before a
 * series is called I(1). The assessment recommends a route; it never applies one.
 */

export type DeterministicSpecification = 'c' | 'ct'

export interface StationarityTestRef {
  readonly test: 'adf' | 'kpss' | 'zivot-andrews'
  readonly specification: DeterministicSpecification | 'level' | 'trend' | 'levelAndTrend'
  readonly series: 'levels' | 'first-difference'
  readonly pValue: number
}

export type StationarityConflict =
  | { readonly kind: 'both-reject'; readonly specification: DeterministicSpecification }
  | { readonly kind: 'neither-rejects'; readonly specification: DeterministicSpecification }
  | { readonly kind: 'difference-battery-missing' }

export type StationarityAssessment =
  | { readonly kind: 'levelStationary'; readonly evidence: NonEmptyArray<StationarityTestRef> }
  | { readonly kind: 'trendStationary'; readonly evidence: NonEmptyArray<StationarityTestRef> }
  | { readonly kind: 'breakStationary'; readonly break: number; readonly model: 'level' | 'trend' | 'levelAndTrend'; readonly evidence: NonEmptyArray<StationarityTestRef> }
  | { readonly kind: 'differenceStationary'; readonly order: 1; readonly evidence: NonEmptyArray<StationarityTestRef> }
  | { readonly kind: 'higherOrderOrUnresolved'; readonly minimumSuspectedOrder: 2; readonly evidence: NonEmptyArray<StationarityTestRef> }
  | { readonly kind: 'inconclusive'; readonly conflicts: NonEmptyArray<StationarityConflict>; readonly evidence: NonEmptyArray<StationarityTestRef> }

type Reading = 'stationary' | 'unit-root' | 'both-reject' | 'neither-rejects'

/** The four ADF and KPSS combinations under one deterministic specification. */
const read = (battery: StationarityBattery, specification: DeterministicSpecification, alpha: number): { readonly reading: Reading; readonly refs: readonly StationarityTestRef[] } => {
  const adf = specification === 'c' ? battery.adf.constant : battery.adf.constantAndTrend
  const kpss = specification === 'c' ? battery.kpss.constant : battery.kpss.constantAndTrend
  const adfRejects = adf.pValue < alpha
  const kpssRejects = kpss.pValue < alpha
  const reading: Reading = adfRejects && !kpssRejects ? 'stationary' : !adfRejects && kpssRejects ? 'unit-root' : adfRejects && kpssRejects ? 'both-reject' : 'neither-rejects'
  return {
    reading,
    refs: [
      { test: 'adf', specification, series: 'levels', pValue: adf.pValue },
      { test: 'kpss', specification, series: 'levels', pValue: kpss.pValue },
    ],
  }
}

export function assessStationarity(levels: StationarityBattery, differenced: StationarityBattery | null, alpha = 0.05): StationarityAssessment {
  const constant = read(levels, 'c', alpha)
  const trend = read(levels, 'ct', alpha)
  const evidence: StationarityTestRef[] = [...constant.refs, ...trend.refs]
  const nonEmpty = (refs: readonly StationarityTestRef[]): NonEmptyArray<StationarityTestRef> => refs as unknown as NonEmptyArray<StationarityTestRef>

  if (constant.reading === 'stationary') return { kind: 'levelStationary', evidence: nonEmpty(constant.refs) }
  if (trend.reading === 'stationary') return { kind: 'trendStationary', evidence: nonEmpty(evidence) }

  // A one-time break can rescue a series the ordinary tests refuse or contradict.
  const breaks = ([
    ['level', levels.zivotAndrews.level],
    ['trend', levels.zivotAndrews.trend],
    ['levelAndTrend', levels.zivotAndrews.levelAndTrend],
  ] as const).filter(([, result]) => result.pValue < alpha)
  const breakRefs: StationarityTestRef[] = (['level', 'trend', 'levelAndTrend'] as const).map((model) => ({ test: 'zivot-andrews', specification: model, series: 'levels', pValue: levels.zivotAndrews[model].pValue }))
  if (breaks.length > 0 && constant.reading !== 'unit-root') {
    const [model, result] = breaks.reduce((best, candidate) => (candidate[1].pValue < best[1].pValue ? candidate : best))
    return { kind: 'breakStationary', break: result.breakIndex, model, evidence: nonEmpty([...evidence, ...breakRefs]) }
  }

  if (constant.reading === 'unit-root' || trend.reading === 'unit-root') {
    if (differenced === null) {
      return { kind: 'inconclusive', conflicts: [{ kind: 'difference-battery-missing' }], evidence: nonEmpty(evidence) }
    }
    const differenceReading = read(differenced, 'c', alpha)
    const differenceRefs = differenceReading.refs.map((ref) => ({ ...ref, series: 'first-difference' as const }))
    if (differenceReading.reading === 'stationary') return { kind: 'differenceStationary', order: 1, evidence: nonEmpty([...evidence, ...differenceRefs]) }
    if (breaks.length > 0) {
      const [model, result] = breaks.reduce((best, candidate) => (candidate[1].pValue < best[1].pValue ? candidate : best))
      return { kind: 'breakStationary', break: result.breakIndex, model, evidence: nonEmpty([...evidence, ...breakRefs, ...differenceRefs]) }
    }
    return { kind: 'higherOrderOrUnresolved', minimumSuspectedOrder: 2, evidence: nonEmpty([...evidence, ...differenceRefs]) }
  }

  const conflicts: StationarityConflict[] = []
  for (const [specification, reading] of [['c', constant.reading], ['ct', trend.reading]] as const) {
    if (reading === 'both-reject') conflicts.push({ kind: 'both-reject', specification })
    if (reading === 'neither-rejects') conflicts.push({ kind: 'neither-rejects', specification })
  }
  return { kind: 'inconclusive', conflicts: conflicts as unknown as NonEmptyArray<StationarityConflict>, evidence: nonEmpty([...evidence, ...breakRefs]) }
}

export function describeStationarityAssessment(assessment: StationarityAssessment): { readonly verdict: string; readonly route: string; readonly tone: 'ok' | 'warn' | 'danger' | 'muted' } {
  switch (assessment.kind) {
    case 'levelStationary': return { verdict: 'I(0), level-stationary', route: 'keep levels', tone: 'ok' }
    case 'trendStationary': return { verdict: 'trend-stationary', route: 'keep levels and include the trend, or detrend explicitly', tone: 'ok' }
    case 'breakStationary': return { verdict: `break-stationary (${assessment.model} break at ${assessment.break + 1})`, route: 'keep levels and model the break or regime', tone: 'warn' }
    case 'differenceStationary': return { verdict: 'I(1), difference-stationary', route: 'difference for short-run analysis, or keep levels with cointegration', tone: 'warn' }
    case 'higherOrderOrUnresolved': return { verdict: 'I(2) or unresolved', route: 'refuse I(0)/I(1)-only methods until the order is settled', tone: 'danger' }
    case 'inconclusive': return { verdict: 'inconclusive', route: 'do not choose a transformation silently; inspect trend and breaks', tone: 'muted' }
    default: return assertNever(assessment)
  }
}

/** The tests that decided the route: the levels reading under `c`, plus whichever specification settled it. */
export function decisiveEvidence(assessment: StationarityAssessment): readonly StationarityTestRef[] {
  return assessment.evidence.filter((ref) => {
    if (ref.series === 'first-difference') return true
    if (ref.test === 'zivot-andrews') return assessment.kind === 'breakStationary' && ref.specification === assessment.model
    if (ref.specification === 'ct') return assessment.kind === 'trendStationary' || assessment.kind === 'inconclusive'
    return true
  })
}

export function describeStationarityConflict(conflict: StationarityConflict): string {
  switch (conflict.kind) {
    case 'both-reject': return `ADF and KPSS both reject under ${conflict.specification}: conflicting evidence.`
    case 'neither-rejects': return `Neither ADF nor KPSS rejects under ${conflict.specification}: the sample cannot tell.`
    case 'difference-battery-missing': return 'The first difference was not tested, so the series cannot be called I(1).'
    default: return assertNever(conflict)
  }
}

/** Whether a level model (a regression in levels) may run on a series with this assessment. */
export type LevelModelVerdict =
  | { readonly kind: 'allowed'; readonly reason: string }
  | { readonly kind: 'unresolved'; readonly reason: string }
  | { readonly kind: 'refused'; readonly reason: string }

export function levelModelVerdict(name: string, assessment: StationarityAssessment | null): LevelModelVerdict {
  if (assessment === null) return { kind: 'unresolved', reason: `Run stationarity tests for ${name} on this prepared dataset version.` }
  switch (assessment.kind) {
    case 'levelStationary': return { kind: 'allowed', reason: `${name} is level-stationary.` }
    case 'trendStationary': return { kind: 'allowed', reason: `${name} is trend-stationary; a trend term or detrending is still owed.` }
    case 'breakStationary': return { kind: 'unresolved', reason: `${name} is stationary only around a break at ${assessment.break + 1}; the model does not include it.` }
    case 'differenceStationary': return { kind: 'refused', reason: `${name} is I(1): a level regression risks a spurious relation. Differencing in Data studio or a cointegration method might be needed.` }
    case 'higherOrderOrUnresolved': return { kind: 'refused', reason: `${name} is I(2) or unresolved; no I(0)/I(1) method applies until its order is settled.` }
    case 'inconclusive': return { kind: 'unresolved', reason: `${name}'s stationarity evidence is inconclusive: ${assessment.conflicts.map(describeStationarityConflict).join(' ')}` }
    default: return assertNever(assessment)
  }
}
