import { EvidenceTable } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Metadata } from '@/components/ui/Metadata'
import { num, well } from '@/components/ui/recipes'
import { type EstimationRunArtifact } from '@/domain/estimation'
import { formatStatistic } from '@/lib/format/number'
import { AdjustedDidResult } from './AdjustedDidResult'
import { StaggeredDidResult } from './StaggeredDidResult'
import { SunAbrahamResult } from './SunAbrahamResult'

const panelWeight = (values: readonly number[], index: number): string =>
  formatStatistic('score', values[index] ?? Number.NaN).text

export function SyntheticControlEvidenceDetails({
  run,
}: {
  readonly run: Extract<EstimationRunArtifact, { readonly kind: 'synthetic-control-run' }>
}) {
  const { evidence } = run
  const donorNames = run.columns.slice(2).map((column) => column.name)
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <DisclosureSummary className="cursor-pointer text-ink">
        Synthetic-control inference
      </DisclosureSummary>
      <div className="mt-3 grid gap-4">
        {evidence.crossFit.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table
              className="w-full border-collapse text-table"
              aria-label="Cross-fitted synthetic-control folds"
            >
              <thead>
                <tr className="text-left">
                  <th className="border-b border-hair px-2 py-1.5 font-medium">Fold</th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                    Held-out rows
                  </th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">Bias</th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                    Effect
                  </th>
                </tr>
              </thead>
              <tbody>
                {evidence.crossFit.folds.map((fold, index) => (
                  <tr key={index}>
                    <th
                      scope="row"
                      className="border-b border-hair px-2 py-1.5 text-left font-normal"
                    >
                      {index + 1}
                    </th>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {fold.heldOut[0] + 1}–{(fold.heldOut.at(-1) ?? fold.heldOut[0]) + 1}
                    </td>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {formatStatistic('raw', fold.bias).text}
                    </td>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {formatStatistic('raw', fold.att).text}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p className="m-0 text-body text-muted">
            Cross-fitted inference unavailable: {evidence.crossFit.reason}
          </p>
        )}
        {evidence.donorPlacebo.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table
              className="w-full border-collapse text-table"
              aria-label="Donor-placebo MSPE ratios"
            >
              <thead>
                <tr className="text-left">
                  <th className="border-b border-hair px-2 py-1.5 font-medium">Series</th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                    Pre MSPE
                  </th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                    Post MSPE
                  </th>
                  <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                    Post/pre ratio
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th
                    scope="row"
                    className="border-b border-hair px-2 py-1.5 text-left font-medium"
                  >
                    Treated
                  </th>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {formatStatistic('raw', evidence.donorPlacebo.treatedPreMspe).text}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {formatStatistic('raw', evidence.donorPlacebo.treatedPostMspe).text}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {evidence.donorPlacebo.treatedMspeRatio === null
                      ? '∞'
                      : formatStatistic('raw', evidence.donorPlacebo.treatedMspeRatio).text}
                  </td>
                </tr>
                {evidence.donorPlacebo.placebos.map((placebo) => (
                  <tr key={placebo.donor}>
                    <th
                      scope="row"
                      className="border-b border-hair px-2 py-1.5 text-left font-normal"
                    >
                      {donorNames[placebo.donor] ?? `Donor ${placebo.donor + 1}`}
                    </th>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {formatStatistic('raw', placebo.preMspe).text}
                    </td>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {formatStatistic('raw', placebo.postMspe).text}
                    </td>
                    <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                      {placebo.mspeRatio === null
                        ? '∞'
                        : formatStatistic('raw', placebo.mspeRatio).text}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p className="m-0 text-body text-muted">
            Donor-placebo inference unavailable: {evidence.donorPlacebo.reason}
          </p>
        )}
        <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
          <dt>Conformal band</dt>
          <dd className="m-0">
            {evidence.conformalBand.kind === 'available'
              ? `half-width ${formatStatistic('raw', evidence.conformalBand.halfWidth).text}`
              : evidence.conformalBand.reason}
          </dd>
          <dt>Gaussian band</dt>
          <dd className="m-0">
            {evidence.gaussianBand.kind === 'available'
              ? `half-width ${formatStatistic('raw', evidence.gaussianBand.halfWidth).text}`
              : evidence.gaussianBand.reason}
          </dd>
        </dl>
      </div>
    </details>
  )
}

type PanelPeriodDisplay =
  | { readonly kind: 'source-labels'; readonly labels: readonly string[] }
  | { readonly kind: 'dense-codes'; readonly labels: readonly string[] }

function panelPeriodDisplay(
  run: Extract<EstimationRunArtifact, { readonly kind: 'panel-intervention-run' }>,
): PanelPeriodDisplay {
  const recorded: unknown = Reflect.get(run, 'timeLabels')
  if (
    Array.isArray(recorded) &&
    recorded.length === run.evidence.times.length &&
    recorded.every((label) => typeof label === 'string' && label.length > 0)
  ) {
    return { kind: 'source-labels', labels: recorded }
  }
  return { kind: 'dense-codes', labels: run.evidence.times.map(String) }
}

export function PanelEvidenceDetails({
  run,
}: {
  readonly run: Extract<EstimationRunArtifact, { readonly kind: 'panel-intervention-run' }>
}) {
  const { evidence } = run
  if (evidence.kind === 'sunAbraham')
    return <SunAbrahamResult evidence={evidence} sourcePeriods={run.sourcePeriods} />
  if (evidence.kind === 'staggeredDid')
    return (
      <StaggeredDidResult
        evidence={evidence}
        labels={panelPeriodDisplay(run).labels}
        sourcePeriods={run.sourcePeriods}
      />
    )
  const controls = evidence.units.slice(0, evidence.controlUnits)
  const periods = panelPeriodDisplay(run)
  const preLabels = periods.labels.slice(0, evidence.nPre)
  const postLabels = periods.labels.slice(evidence.nPre)
  if (evidence.kind === 'panelAdjusted')
    return (
      <AdjustedDidResult
        evidence={evidence}
        labels={periods.labels}
        covariates={run.columns.slice(2).map((column) => column.name)}
      />
    )
  if (evidence.kind === 'panelDid')
    return (
      <EvidenceTable
        title="Post-period differences"
        frame="none"
        rows={evidence.did.effectCurve.map((value, index) => ({
          key: String(index),
          period: postLabels[index] ?? String(index),
          value,
        }))}
        rowKey={(row) => row.key}
        noun="period"
        empty="No post-period differences."
        exportName="did-period-effects"
        columns={[
          { id: 'period', header: 'Period', value: (row) => row.period },
          {
            id: 'effect',
            header: 'Difference',
            value: (row) => formatStatistic('raw', row.value).text,
          },
        ]}
      />
    )
  const estimates = [
    ['Difference-in-differences', evidence.did, null, null],
    [
      'Synthetic control',
      evidence.syntheticControl,
      evidence.syntheticControlPlacebo,
      evidence.syntheticControlInTime,
    ],
    [
      'Synthetic difference-in-differences',
      evidence.syntheticDid,
      evidence.syntheticDidPlacebo,
      evidence.syntheticDidInTime,
    ],
  ] as const
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <DisclosureSummary className="cursor-pointer text-ink">
        Panel weights and period effects
      </DisclosureSummary>
      <div className="mt-3 grid gap-4">
        {periods.kind === 'dense-codes' && (
          <Alert tone="info" live={false}>
            <p className="m-0">
              This saved run does not contain source period labels. The period tables therefore show
              zero-based dense codes.
            </p>
          </Alert>
        )}
        <div className="figure-strip overflow-x-auto">
          <table
            className="w-full border-collapse text-table"
            aria-label="Panel estimator comparison"
          >
            <thead>
              <tr className="text-left">
                <th className="border-b border-hair px-2 py-1.5 font-medium">Estimator</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                  Estimate
                </th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                  Placebo SE
                </th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                  In-time placebo
                </th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">
                  Noise level
                </th>
              </tr>
            </thead>
            <tbody>
              {estimates.map(([name, estimate, placebo, inTime]) => (
                <tr key={name}>
                  <th
                    scope="row"
                    className="border-b border-hair px-2 py-1.5 text-left font-normal"
                  >
                    {name}
                    {name.startsWith('Synthetic difference') ? (
                      <Metadata>
                        <span></span>
                        <span>primary</span>
                      </Metadata>
                    ) : (
                      ''
                    )}
                  </th>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {formatStatistic('raw', estimate.estimate).text}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {placebo === null
                      ? '—'
                      : placebo.kind === 'available'
                        ? formatStatistic('raw', placebo.standardError).text
                        : 'Unavailable'}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {inTime === null
                      ? '—'
                      : inTime.kind === 'available'
                        ? formatStatistic('raw', inTime.estimate).text
                        : 'Unavailable'}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {formatStatistic('raw', estimate.noiseLevel).text}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        {[
          evidence.syntheticControlPlacebo,
          evidence.syntheticDidPlacebo,
          evidence.syntheticControlInTime,
          evidence.syntheticDidInTime,
        ].some((item) => item.kind === 'unavailable') ? (
          <ul className="m-0 grid gap-1 pl-5 text-body text-muted">
            {evidence.syntheticControlPlacebo.kind === 'unavailable' ? (
              <li>SC placebo SE: {evidence.syntheticControlPlacebo.reason}</li>
            ) : null}
            {evidence.syntheticDidPlacebo.kind === 'unavailable' ? (
              <li>SDID placebo SE: {evidence.syntheticDidPlacebo.reason}</li>
            ) : null}
            {evidence.syntheticControlInTime.kind === 'unavailable' ? (
              <li>SC in-time placebo: {evidence.syntheticControlInTime.reason}</li>
            ) : null}
            {evidence.syntheticDidInTime.kind === 'unavailable' ? (
              <li>SDID in-time placebo: {evidence.syntheticDidInTime.reason}</li>
            ) : null}
          </ul>
        ) : null}
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-table" aria-label="Panel unit weights">
            <thead>
              <tr className="text-left">
                <th className="border-b border-hair px-2 py-1.5 font-medium">Control unit</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th>
              </tr>
            </thead>
            <tbody>
              {controls.map((unit, index) => (
                <tr key={unit}>
                  <th
                    scope="row"
                    className="border-b border-hair px-2 py-1.5 text-left font-normal"
                  >
                    {unit}
                  </th>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.did.omega, index)}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.syntheticControl.omega, index)}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.syntheticDid.omega, index)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-table" aria-label="Panel time weights">
            <thead>
              <tr className="text-left">
                <th className="border-b border-hair px-2 py-1.5 font-medium">Pre-period</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th>
              </tr>
            </thead>
            <tbody>
              {preLabels.map((period, index) => (
                <tr key={`${period}-${index}`}>
                  <th
                    scope="row"
                    className="border-b border-hair px-2 py-1.5 text-left font-normal"
                  >
                    {period}
                  </th>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.did.lambda, index)}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.syntheticControl.lambda, index)}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {panelWeight(evidence.syntheticDid.lambda, index)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
          <table
            className="w-full border-collapse text-table"
            aria-label="Panel post-period effects"
          >
            <thead>
              <tr className="text-left">
                <th className="border-b border-hair px-2 py-1.5 font-medium">Post-period</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th>
                <th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th>
              </tr>
            </thead>
            <tbody>
              {postLabels.map((period, index) => (
                <tr key={`${period}-${index}`}>
                  <th
                    scope="row"
                    className="border-b border-hair px-2 py-1.5 text-left font-normal"
                  >
                    {period}
                  </th>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {formatStatistic('raw', evidence.did.effectCurve[index] ?? Number.NaN).text}
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {
                      formatStatistic(
                        'raw',
                        evidence.syntheticControl.effectCurve[index] ?? Number.NaN,
                      ).text
                    }
                  </td>
                  <td className={num('border-b border-hair px-2 py-1.5 text-right')}>
                    {
                      formatStatistic('raw', evidence.syntheticDid.effectCurve[index] ?? Number.NaN)
                        .text
                    }
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </details>
  )
}
