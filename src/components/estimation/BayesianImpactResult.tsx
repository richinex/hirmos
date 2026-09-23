import { EvidenceTable } from '@/components/table/EvidenceTable'
import { formatPercent, formatStatistic } from '@/lib/format/number'
import type { CausalImpactEvidence } from '@/domain/estimation'
import { StructuralImpactResult } from './StructuralImpactResult'
import { assertNever } from '@/domain/dop'

type Evidence = Exclude<CausalImpactEvidence, { kind: 'causalImpact' }>
type ImpactSummary = Evidence['averageSummary']
type PosteriorQuantity = ImpactSummary['predicted']

const figure = (value: number) => formatStatistic('raw', value).text
const share = (value: number) => formatPercent(value).text
const spread = (quantity: PosteriorQuantity, print: (value: number) => string) => `${print(quantity.mean)} (${print(quantity.sd)})`
const bounds = (quantity: PosteriorQuantity, print: (value: number) => string) => `[${print(quantity.lower)}, ${print(quantity.upper)}]`

function specification(evidence: Evidence) {
  const p = evidence.controls.length
  const components = evidence.kind === 'structuralCausalImpact'
  const inclusion = p === 0 ? 'No controls' : share(Math.min((components ? 1 : 3) / p, 1))
  const trend = components ? { level:'Local level', linear:'Local linear trend', semilocal:'Semilocal linear trend' }[evidence.model.trend] : 'Local level'
  const seasonal = () => {
    if (!components) return 'None'
    const season = evidence.model.seasonality
    switch (season.kind) {
      case 'none': return 'None'
      case 'seasonal': return `${season.seasons} seasons, ${season.duration} observations per season`
      case 'harmonic': return `Trigonometric, period ${season.period}, ${season.pairs} frequencies`
      default: return assertNever(season)
    }
  }
  return [
    { name:'Model', value:components ? 'BSTS components' : 'CausalImpact specification' },
    { name:'Trend', value:trend },
    { name:'Seasonality', value:seasonal() },
    { name:'Level innovation variance prior', value:components
      ? 'SdPrior: sigma.guess = 0.01, sample.size = 0.01, upper.limit = 1'
      : `Inverse gamma: concentration = 16, scale = 16 × ${evidence.priorLevelSd}²` },
    { name:'Prior inclusion probability per control', value:inclusion },
  ]
}

export function BayesianImpactResult({ evidence, columnNames = [] }: { readonly evidence: Evidence; readonly columnNames?: readonly string[] }) {
  // The quantities and their names follow the CausalImpact summary this model was ported from,
  // so a reader who knows that output finds the same rows here.
  const quantity = (
    id: string, name: string,
    read: (summary: ImpactSummary) => PosteriorQuantity,
    print: (value: number) => string,
  ) => [
    { id, name, average: spread(read(evidence.averageSummary), print), cumulative: spread(read(evidence.cumulativeSummary), print) },
    { id: `${id}-interval`, name: '95% CI', average: bounds(read(evidence.averageSummary), print), cumulative: bounds(read(evidence.cumulativeSummary), print) },
  ]
  const rows = [
    { id: 'actual', name: 'Actual', average: figure(evidence.averageSummary.actual), cumulative: figure(evidence.cumulativeSummary.actual) },
    ...quantity('prediction', 'Prediction (s.d.)', (summary) => summary.predicted, figure),
    ...quantity('absolute', 'Absolute effect (s.d.)', (summary) => summary.absolute, figure),
    ...quantity('relative', 'Relative effect (s.d.)', (summary) => summary.relative, share),
  ]
  type Row = typeof rows[number]
  return <div className="mt-3 grid gap-3">
    <EvidenceTable title="Model specification" help="Prior scales refer to the outcome standardised using the pre-intervention mean and standard deviation. These are the fitted run's settings. Prior inclusion probability 100% keeps a control in every draw; lower probabilities allow spike-and-slab variable selection." rows={specification(evidence)} rowKey={row => row.name} noun="setting" empty="No model specification." columns={[
      { id:'name', header:'Setting', value:row => row.name },
      { id:'value', header:'Value', value:row => row.value },
    ]} />
    {evidence.controls.length > 0 && (evidence.controlInclusion === undefined
      ? <p className="m-0 text-body text-muted">Posterior inclusion probabilities were not recorded for this run.</p>
      : <EvidenceTable title="Posterior inclusion probabilities" help="Proportion of retained posterior draws in which each control is included. This is not the probability of a causal effect." rows={evidence.controlInclusion} rowKey={row => String(row.column)} noun="control" empty="No controls." columns={[
        { id:'control', header:'Control', value:row => columnNames[row.column] ?? `Column ${row.column + 1}` },
        { id:'probability', header:'Inclusion probability', align:'right', mono:true, value:row => share(row.probability) },
      ]} />)}
    {evidence.kind === 'structuralCausalImpact' && <StructuralImpactResult evidence={evidence} columnNames={columnNames} />}
    <EvidenceTable title="Posterior inference" help="Equal-tailed 95% posterior intervals. Cumulative bounds come from cumulative posterior paths, not sums of pointwise bounds. These are conditional on the model, controls and no competing intervention." rows={rows} rowKey={row => row.id} noun="row" empty="No posterior summary." columns={[
      { id: 'quantity', header: '', value: row => row.name },
      { id: 'average', header: 'Average', align: 'right', mono: true, value: row => row.average },
      { id: 'cumulative', header: 'Cumulative', align: 'right', mono: true, value: row => row.cumulative },
    ]} />
    <p className="m-0 text-body text-muted">Posterior tail-area probability p: {formatStatistic('raw', evidence.averageSummary.tailProbability).text}. Posterior prob. of a causal effect: {formatPercent(1 - evidence.averageSummary.tailProbability, { precision: 2 }).text}.</p>
  </div>
}
