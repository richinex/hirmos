import { memo, useMemo } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { causalForestPredictionsOption, causalForestTocOption } from '@/charts/estimation/causalForest'
import { useChartTheme } from '@/charts/theme'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { IntervalFigure } from '@/components/ui/figures'
import { fieldHint, stepTitle } from '@/components/ui/recipes'
import { CAUSAL_FOREST_COPY, describeCausalForestTarget, forestFeatureLabel, type CausalForestEvidence } from '@/domain/causalForest'
import { formatStatistic, formatP } from '@/lib/format/number'

const format = (value: number) => formatStatistic('raw', value).text
const formatCell = (value: string | number) => typeof value === 'number' ? format(value) : value
type CalibrationRow = { readonly term: string; readonly estimate: number; readonly standardError: number; readonly pValue: number }
type PredictionRow = CausalForestEvidence['predictions'][number] & { readonly row: number }
const predictionColumns: readonly EvidenceColumn<PredictionRow>[] = [
  { id: 'row', header: 'Prepared row', value: row => row.row },
  { id: 'estimate', header: 'Conditional estimate', align: 'right', value: row => row.kind === 'estimated' ? row.estimate : 'Unavailable', format: formatCell },
  { id: 'se', header: 'Standard error', align: 'right', value: row => row.kind === 'estimated' && row.uncertainty.kind === 'estimated' ? row.uncertainty.standardError : 'Unavailable', format: formatCell },
  { id: 'lower', header: 'Lower bound', align: 'right', value: row => row.kind === 'estimated' && row.uncertainty.kind === 'estimated' ? row.uncertainty.interval.lower : 'Unavailable', format: formatCell },
  { id: 'upper', header: 'Upper bound', align: 'right', value: row => row.kind === 'estimated' && row.uncertainty.kind === 'estimated' ? row.uncertainty.interval.upper : 'Unavailable', format: formatCell },
  { id: 'status', header: 'Availability', value: row => row.kind === 'unavailable' ? row.reason : row.uncertainty.kind === 'unavailable' ? row.uncertainty.reason : 'Estimate and interval available' },
]
const calibrationColumns: readonly EvidenceColumn<CalibrationRow>[] = [
  { id: 'term', header: 'Prediction term', value: row => row.term },
  { id: 'estimate', header: 'Coefficient', align: 'right', value: row => row.estimate, format: (_, row) => format(row.estimate) },
  { id: 'se', header: 'Standard error', align: 'right', value: row => row.standardError, format: (_, row) => format(row.standardError) },
  { id: 'p', header: 'One-sided p-value', align: 'right', value: row => row.pValue, format: (_, row) => formatP(row.pValue, { withLabel: false }).text },
]
const rankColumns: readonly EvidenceColumn<{ readonly term: string; readonly estimate: number; readonly standardError: number }>[] = [
  { id: 'term', header: 'Priority rule', value: row => row.term },
  { id: 'estimate', header: 'RATE estimate', value: row => row.estimate, format: formatCell, align: 'right' },
  { id: 'se', header: 'Standard error', value: row => row.standardError, format: formatCell, align: 'right' },
]

/** Evidence is immutable. Local chart interaction does not refit or remount the result. */
export const CausalForestResults = memo(function CausalForestResults({ evidence, outcome, columns = [] }: {
  readonly evidence: CausalForestEvidence
  readonly outcome: string
  readonly columns?: readonly { readonly column: string; readonly name: string }[]
}) {
  const theme = useChartTheme()
  const option = useMemo(() => causalForestPredictionsOption(evidence, outcome, theme), [evidence, outcome, theme])
  const toc = useMemo(() => causalForestTocOption(evidence, theme), [evidence, theme])
  const analysis = evidence.analysis
  const projection = analysis?.projection.kind === 'estimated' ? analysis.projection.result : null
  const projectionIds = analysis?.specification.projection.kind === 'linear' ? analysis.specification.projection.columns : []
  const labels = analysis?.specification.labels ?? columns
  const priorityIds = analysis?.specification.ranking.kind === 'external' ? analysis.specification.ranking.columns : []
  const projectionRows = projection === null ? [] : projection.estimates.map((estimate, index) => ({
    term: index === 0 ? 'Intercept' : labels.find(column => column.column === projectionIds[index - 1])?.name ?? `Covariate ${index}`, estimate,
    standardError: projection.standardErrors[index]!, pValue: projection.pValues[index]!,
  }))
  const ranking = analysis?.ranking.kind === 'estimated' ? analysis.ranking.result : null
  const rankedRows = ranking === null ? [] : [...ranking.rules.map((rule, index) => ({ term: labels.find(column => column.column === priorityIds[index])?.name ?? `Priority rule ${index + 1}`, ...rule })),
    ...(ranking.difference === null ? [] : [{ term: 'Rule 1 minus rule 2', ...ranking.difference }])]
  const predictions = useMemo(() => evidence.predictions.map((prediction, index): PredictionRow => ({ ...prediction, row: index + 1 })), [evidence.predictions])
  const calibration = useMemo(() => evidence.calibration.kind === 'estimated' ? [
    { term: 'Mean prediction', ...evidence.calibration.mean },
    { term: 'Differential prediction', ...evidence.calibration.differential },
  ] : [], [evidence.calibration])
  const binary = evidence.target.kind === 'binary-average' || evidence.target.kind === 'binary-conditional'
  const featureLabel = (index: number) => evidence.features !== undefined ? forestFeatureLabel(evidence.features[index]!)
    : columns.length - 2 === evidence.variableImportance.length ? columns[index+2]!.name : `Covariate ${index+1} (label not saved)`
  const refitRows = evidence.refit?.initialImportance.map((importance, index) => ({
    index, term: featureLabel(index),
    importance, retained: evidence.refit!.selected.includes(index) ? 'Retained' : 'Not retained',
  })) ?? []
  const importanceRows = evidence.variableImportance.flatMap((importance, index) =>
    evidence.refit !== undefined && !evidence.refit.selected.includes(index) ? [] : [{ index, term: featureLabel(index), importance }])
  const moderationRows = analysis?.moderation?.map((test, index) => ({ ...test, index,
    name: labels.find(c => c.column === test.column)?.name ?? test.column,
  })) ?? []
  const moderationColumns: readonly EvidenceColumn<(typeof moderationRows)[number]>[] = [
    { id: 'name', header: 'Characteristic', value: row => row.name },
    { id: 'method', header: 'Comparison', value: row => row.method },
    { id: 'estimate', header: 'High minus low', value: row => row.result.kind === 'estimated' && row.result.result.kind === 'contrast' ? row.result.result.estimate : 'Not applicable', format: formatCell },
    { id: 'se', header: 'Standard error', value: row => row.result.kind === 'estimated' && row.result.result.kind === 'contrast' ? row.result.result.standardError : 'Not applicable', format: formatCell },
    { id: 'lower', header: 'Interval lower', value: row => row.result.kind === 'estimated' && row.result.result.kind === 'contrast' ? row.result.result.lower : 'Not applicable', format: formatCell },
    { id: 'upper', header: 'Interval upper', value: row => row.result.kind === 'estimated' && row.result.result.kind === 'contrast' ? row.result.result.upper : 'Not applicable', format: formatCell },
    { id: 'statistic', header: 'Test statistic', value: row => row.result.kind === 'estimated' ? row.result.result.statistic : 'Unavailable', format: formatCell },
    { id: 'df', header: 'Degrees of freedom', value: row => row.result.kind === 'estimated' ? row.result.result.degreesOfFreedom : 'Unavailable', format: formatCell },
    { id: 'residual-df', header: 'Residual degrees of freedom', value: row => row.result.kind === 'estimated' && row.result.result.kind === 'omnibus' ? row.result.result.residualDegreesOfFreedom : 'Not applicable', format: formatCell },
    { id: 'p', header: 'Two-sided t / upper-tail F p-value', value: row => row.result.kind === 'estimated' ? row.result.result.pValue : 'Unavailable', format: v => typeof v === 'number' ? formatP(v, {withLabel:false}).text : String(v) },
    { id: 'status', header: 'Availability', value: row => row.result.kind === 'unavailable' ? row.result.reason : 'Estimated' },
  ]
  return <div data-testid="causal-forest-results" className="space-y-5">
    <h3 className={stepTitle}>{describeCausalForestTarget(evidence.target)}</h3>
    {analysis !== undefined && <p className={fieldHint}>{evidence.summary.kind === 'not-requested' ? 'No aggregate effect requested.' : `${analysis.specification.averageMethod.toUpperCase()} aggregate inference.`} {analysis.specification.sampling.kind === 'independent' ? 'Independent observations.' : analysis.specification.sampling.kind === 'equal-clusters' ? 'Clusters receive equal weight.' : 'Clustered sampling and inference.'}</p>}
    {evidence.summary.kind === 'estimated' && <IntervalFigure
      sentence={`${describeCausalForestTarget(evidence.target)} on ${outcome}`}
      estimate={evidence.summary.estimate} lower={evidence.summary.interval.lower} upper={evidence.summary.interval.upper}
      type={{ kind: 'confidence', level: evidence.confidenceLevel }} scale={{ kind: 'additive', unit: '' }}
      standardError={evidence.summary.standardError} sampleLine={`${evidence.observations} observations`}
      scaleLine={binary ? 'Contrast between treatment 1 and treatment 0.' : 'Conditional treatment slopes averaged over the recorded target population.'}
      testId="causal-forest-aggregate"
    />}
    {evidence.summary.kind === 'unavailable' && <p className={fieldHint}>Aggregate estimate unavailable: {evidence.summary.reason}</p>}
    {evidence.refit !== undefined && <div>
      <h4 className={stepTitle}>Importance-selected refit</h4>
      <p className={fieldHint}>The final forest retained {evidence.refit.selected.length} of {evidence.refit.initialImportance.length} encoded covariates. Outcome and treatment predictions from the all-covariate nuisance forests were reused unchanged. Selection does not change the recorded adjustment set.</p>
      <EvidenceTable title="Initial forest importance" rows={refitRows} columns={[
        { id:'term',header:'Covariate',value:row=>row.term }, {id:'importance',header:'Importance',value:row=>row.importance,format:formatCell}, {id:'retained',header:'Final forest',value:row=>row.retained},
      ]} rowKey={row=>String(row.index)} noun="covariate" empty="No initial importance is available." exportName="forest-initial-importance" />
      <p className={fieldHint}>{CAUSAL_FOREST_COPY.importance}</p>
    </div>}
    <div>
      <EvidenceTable title="Final forest importance" rows={importanceRows} columns={[
        { id: 'term', header: 'Covariate', value: row => row.term },
        { id: 'importance', header: 'Importance', value: row => row.importance, format: formatCell },
      ]} rowKey={row => String(row.index)} noun="covariate" empty="No importance is available." exportName="forest-final-importance" />
      <p className={fieldHint}>{CAUSAL_FOREST_COPY.importance}</p>
    </div>
    <div>
      <p className={fieldHint}>{binary ? CAUSAL_FOREST_COPY.conditional : CAUSAL_FOREST_COPY.partial}</p>
      {!binary && <p className={fieldHint}>{CAUSAL_FOREST_COPY.partialContrast}</p>}
      <ExpandableChart option={option} label="Conditional-effect estimates and pointwise intervals" className="h-[320px]" testId="causal-forest-predictions" />
      <p className={fieldHint}>{CAUSAL_FOREST_COPY.intervals} {CAUSAL_FOREST_COPY.outOfBag}</p>
      <EvidenceTable title="Conditional predictions" rows={predictions} columns={predictionColumns} rowKey={row => String(row.row)} noun="prediction" empty="No predictions are available." exportName="causal-forest-predictions" />
    </div>
    <div>
      <h4 className={stepTitle}>Calibration</h4>
      <p className={fieldHint}>{CAUSAL_FOREST_COPY.variation}</p>
      {evidence.calibration.kind === 'estimated' ? <>
        <EvidenceTable title="Forest calibration" rows={calibration} columns={calibrationColumns} rowKey={row => row.term} noun="coefficient" empty="No calibration coefficients are available." exportName="causal-forest-calibration" />
        <p className={fieldHint}>{CAUSAL_FOREST_COPY.calibration}</p>
        <p className={fieldHint}>{CAUSAL_FOREST_COPY.calibrationTest}</p>
      </> : <p className={fieldHint}>Calibration unavailable: {evidence.calibration.reason}</p>}
    </div>
    {analysis?.projection.kind === 'unavailable' && <p className={fieldHint}>Projection unavailable: {analysis.projection.reason}</p>}
    {projection !== null && <div>
      <h4 className={stepTitle}>Best linear projection</h4>
      <EvidenceTable title="Effect projection" rows={projectionRows} columns={calibrationColumns.map(column => column.id === 'p' ? { ...column, header: 'Two-sided p-value' } : column)} rowKey={row => row.term} noun="coefficient" empty="No projection coefficients are available." exportName="causal-forest-projection" />
      <p className={fieldHint}>Coefficients summarize variation in conditional treatment effects. They are not effects of intervening on the projection covariates.</p>
    </div>}
    {analysis?.ranking.kind === 'unavailable' && <p className={fieldHint}>Prioritization analysis unavailable: {analysis.ranking.reason}</p>}
    {moderationRows.length > 0 && <div>
      <h4 className={stepTitle}>Cluster-score moderation</h4>
      <EvidenceTable title="Cluster-score comparisons" rows={moderationRows} columns={moderationColumns} rowKey={row=>String(row.index)} noun="comparison" empty="No cluster comparisons are available." exportName="forest-cluster-moderation" />
      <p className={fieldHint}>Each cluster receives equal weight. These tests compare estimated treatment effects across characteristics; they do not estimate effects of changing those characteristics. P-values are not adjusted for multiple comparisons.</p>
    </div>}
    {ranking !== null && <div>
      <h4 className={stepTitle}>Treatment prioritization</h4>
      <ExpandableChart option={toc} label="Targeting operator characteristic" className="h-[320px]" testId="causal-forest-toc" />
      <p className={fieldHint}>The curve compares the effect among the highest-priority fraction with the overall average. It shows point estimates, not confidence bands.</p>
      <EvidenceTable title="Rank-weighted effects" rows={rankedRows} columns={rankColumns} rowKey={row => row.term} noun="rule" empty="No ranking estimates are available." exportName="causal-forest-rate" />
      <p className={fieldHint}>{analysis?.specification.ranking.kind === 'external' ? `${analysis.specification.ranking.target.toUpperCase()}; ${analysis.specification.ranking.replications} bootstrap replications. ${analysis.specification.ranking.rationale}` : ''}</p>
    </div>}
  </div>
})
