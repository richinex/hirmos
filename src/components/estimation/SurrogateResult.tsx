import { SurrogatePathResult } from './SurrogatePathResult'
import type { SurrogateBiasRestriction } from '@/domain/surrogateDiagnostics'
import type { DatasetProfile } from '@/domain/dataset'
import type { SurrogateRun } from '@/domain/surrogateRun'
import { formatStatistic, formatCount, formatWords } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { RunDetails } from '@/components/ui/RunDetails'
import { RunMeta } from '@/components/ui/RunMeta'
import { button, fieldHint, label, num, resultSurface } from '@/components/ui/recipes'

export const surrogateLabels = {
  index: 'Surrogate index',
  score: 'Surrogate score',
  influenceFunction: 'Influence-function estimator',
} as const

export function SurrogateResult({
  run,
  profile,
  onRestore,
  current = false,
}: {
  readonly run: SurrogateRun
  readonly profile: DatasetProfile
  readonly onRestore?: () => void
  readonly current?: boolean
}) {
  const name = (id: string) => profile.columns.find((c) => c.id === id)?.name ?? id
  const e = run.evidence,
    s = run.selection
  const validation = run.diagnostics.flatMap((d) =>
    d.analysis.kind === 'validation' ? [d.analysis] : [],
  )
  const bounds = run.diagnostics.flatMap((d) =>
    d.analysis.kind === 'biasBounds' ? [d.analysis] : [],
  )
  const validationRows = validation.flatMap((v) => [
    { name: 'Treatment (surrogacy)', value: v.surrogacy },
    { name: 'Experimental sample (comparability)', value: v.comparability },
  ])
  return (
    <section className={resultSurface('min-w-0 space-y-4')} aria-label="Surrogate estimate">
      <div>
        <div className="flex flex-wrap items-baseline justify-between gap-2">
          {current ? (
            <span className={label('text-signal-text')}>Current estimate</span>
          ) : (
            <span className={label('text-muted')}>{surrogateLabels[e.estimator]}</span>
          )}
          <span className={num('text-micro text-faint')}>
            <RunMeta>
              {[
                ...(current ? [surrogateLabels[e.estimator]] : []),
                `${formatCount(e.surrogateColumns).text} surrogates`,
                formatTime(run.createdAt),
              ]}
            </RunMeta>
          </span>
        </div>
        <h3 className="m-0 mt-2 font-sans text-title font-normal leading-7 text-ink">
          Estimated average effect on {plainName(name(s.outcome))} in the experimental population,
          setting treatment to 1 rather than 0.
        </h3>
      </div>
      <MetricGrid label="Surrogate estimate" testId="surrogate-estimate">
        <MetricTile
          size="compact"
          frame="cell"
          label="Effect estimate"
          value={formatStatistic('raw', e.estimate)}
        />
        {e.uncertainty.kind === 'bootstrapStandardError' && (
          <MetricTile
            size="compact"
            frame="cell"
            label="Bootstrap standard error"
            value={formatStatistic('raw', e.uncertainty.standardError)}
            context={`${formatCount(e.uncertainty.repetitions).text} repetitions`}
            help="Treatment arms are resampled separately in the experimental sample and rows in the observational sample."
          />
        )}
        {bounds.map((b) => (
          <MetricTile
            key="bounds"
            size="compact"
            frame="cell"
            label="Bias bounds"
            value={formatWords(
              `${formatStatistic('raw', b.lower).text} to ${formatStatistic('raw', b.upper).text}`,
            )}
            context={boundDescription(b.restriction)}
            help="Bounds on the true effect minus the surrogate estimand. These are not confidence intervals and do not include uncertainty from estimating the nuisance models."
          />
        ))}
      </MetricGrid>
      <p className="m-0 text-body text-muted">
        Experimental sample: {formatCount(e.experimentalRows).text} rows. Observational sample:{' '}
        {formatCount(e.observationalRows).text} rows. Outside both samples:{' '}
        {formatCount(run.rows.excluded.length).text} rows.
      </p>
      {validationRows.length > 0 && (
        <section aria-label="Surrogate validation regressions" className="space-y-2">
          <EvidenceTable
            frame="none"
            title="Validation regressions"
            help="These linear-regression coefficients assess conditional mean restrictions within the fitted specification. They do not test every implication of conditional independence, and non-rejection does not establish the identifying assumptions."
            rows={validationRows}
            rowKey={(r) => r.name}
            noun="regression"
            empty="No validation regressions."
            columns={[
              { id: 'indicator', header: 'Added indicator', value: (r) => r.name },
              {
                id: 'estimate',
                header: 'Coefficient',
                align: 'right',
                value: (r) => r.value.estimate,
                format: (v) => coefficient(Number(v)),
              },
              {
                id: 'se',
                header: 'Standard error',
                align: 'right',
                value: (r) =>
                  r.value.kind === 'estimated' ? r.value.standardError : 'Not applicable',
                format: (v) => (typeof v === 'number' ? coefficient(v) : String(v)),
              },
              {
                id: 't',
                header: 't statistic',
                align: 'right',
                value: (r) =>
                  r.value.kind === 'estimated' ? r.value.tStatistic : 'Not applicable',
                format: (v) => (typeof v === 'number' ? formatStatistic('raw', v).text : String(v)),
              },
              {
                id: 'df',
                header: 'Residual df',
                align: 'right',
                value: (r) => r.value.residualDegreesOfFreedom,
                format: (v) => formatCount(Number(v)).text,
              },
            ]}
          />
          {validation.some(
            (v) => v.surrogacy.kind === 'perfectFit' || v.comparability.kind === 'perfectFit',
          ) && (
            <p className={`${fieldHint} max-w-[75ch]`}>
              Validation is uninformative for a numerically perfect fit: the predictors reproduce
              the outcome to numerical precision. Standard errors and t statistics are not reported
              for that regression. This does not establish surrogacy or comparability.
            </p>
          )}
        </section>
      )}
      {run.paths.results.map((path) => (
        <SurrogatePathResult key={path.kind} path={path} />
      ))}
      <RunDetails label="Specification and recorded assumptions">
        <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
          <dt className="text-faint">Sample membership</dt>
          <dd className="m-0 text-ink">
            {name(s.sample.column)}:{' '}
            {s.sample.kind === 'numeric'
              ? `${s.sample.experimental} experimental, ${s.sample.observational} observational`
              : `${s.sample.experimental.join(', ')} experimental; ${s.sample.observational.join(', ')} observational`}
          </dd>
          <dt className="text-faint">Treatment</dt>
          <dd className="m-0 text-ink">{name(s.treatment)}</dd>
          <dt className="text-faint">Surrogates</dt>
          <dd className="m-0 text-ink">{s.surrogates.map(name).join(', ')}</dd>
          <dt className="text-faint">Baseline covariates</dt>
          <dd className="m-0 text-ink">
            {s.adjustment.kind === 'none' ? 'None' : s.adjustment.columns.map(name).join(', ')}
          </dd>
          {s.uncertainty.kind === 'bootstrap' && (
            <>
              <dt className="text-faint">Bootstrap seed</dt>
              <dd className="m-0 text-ink">{s.uncertainty.seed}</dd>
            </>
          )}
          <dt className="text-faint">Rationale</dt>
          <dd className="m-0 whitespace-pre-wrap text-ink">{run.rationale}</dd>
        </dl>
        {onRestore !== undefined && (
          <button type="button" className={button('quiet', 'mt-3', 'sm')} onClick={onRestore}>
            Restore this specification
          </button>
        )}
      </RunDetails>
    </section>
  )
}

/** As in result headlines elsewhere: a column name reads as words in a sentence. */
const plainName = (value: string): string => value.replaceAll('_', ' ')

/** Three significant figures, switching to exponent form where fixed decimals would show only zeros. */
function coefficient(value: number): string {
  return value !== 0 && Math.abs(value) < 1e-4
    ? value.toExponential(2).replaceAll('-', '−')
    : formatStatistic('raw', value).text
}

function boundDescription(r: SurrogateBiasRestriction): string {
  switch (r.kind) {
    case 'binaryWithoutSurrogacy':
      return 'For a binary outcome, surrogacy is relaxed and comparability is retained.'
    case 'binaryWithoutComparability':
      return 'For a binary outcome, comparability is relaxed and surrogacy is retained.'
    case 'boundedDirectEffect':
      return (
        'The direct treatment effect, conditional on surrogates and baseline covariates, is bounded in absolute value by ' +
        r.maximum +
        '. Comparability is retained.'
      )
    case 'boundedSampleDifference':
      return (
        'The conditional outcome-mean difference between samples is bounded in absolute value by ' +
        r.maximum +
        '. Surrogacy is retained.'
      )
  }
}
