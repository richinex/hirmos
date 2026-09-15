import { memo } from 'react'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import type { RootCauseCheckRecord } from '@/domain/rootCauseAnalysis'
import { formatP, formatStatistic } from '@/lib/format/number'

const number = (value: number) => formatStatistic('raw', value).text
type Performance = RootCauseCheckRecord['evidence']['mechanisms'][number]
type NoiseCheck = RootCauseCheckRecord['evidence']['invertibility'][number]
const metric = (value: string | number) => typeof value === 'number' ? number(value) : value

export const RootCauseChecks = memo(function RootCauseChecks({ record }: { readonly record: RootCauseCheckRecord }) {
  const { evidence, model } = record
  const noiseColumns: readonly EvidenceColumn<NoiseCheck>[] = [
    { id: 'variable', header: 'Variable', value: row => model.names[row.node] },
    { id: 'result', header: 'Result', value: row => row.rejected ? 'Rejected' : 'Not rejected' },
    { id: 'p', header: 'p-value', align: 'right', value: row => row.pValue, format: (_value, row) => formatP(row.pValue).text },
  ]
  const columns: readonly EvidenceColumn<Performance>[] = [
    { id: 'variable', header: 'Variable', value: (row) => model.names[row.node] },
    { id: 'kl', header: 'KL divergence', align: 'right', value: (row) => row.kind === 'root' ? row.klDivergence : '—', format: metric },
    { id: 'crps', header: 'CRPS', align: 'right', value: (row) => row.kind === 'conditional' ? row.crps : '—', format: metric },
    { id: 'mse', header: 'MSE', align: 'right', value: (row) => row.kind === 'conditional' ? row.mse : '—', format: metric },
    { id: 'nmse', header: 'Normalised RMSE', align: 'right', value: (row) => row.kind === 'conditional' ? row.nmse : '—', format: metric },
    { id: 'r2', header: 'R²', align: 'right', value: (row) => row.kind === 'conditional' ? row.r2 : '—', format: metric },
  ]
  return <section aria-label="Fitted-model diagnostics" className="space-y-3">
    <EvidenceTable title="Prediction performance" help="A dash means the measure does not apply. Variables without parents use KL divergence; regression models report CRPS and prediction errors." rows={evidence.mechanisms} columns={columns} rowKey={(row) => String(row.node)} noun="variable" empty="No prediction diagnostics." frame="none" exportName={`model-performance-${record.id}`} />
    <MetricGrid label="Overall distribution check"><MetricTile label="Overall distribution difference" value={formatStatistic('raw', evidence.overallDivergence)} context="KL divergence" size="compact" /></MetricGrid>
    <EvidenceTable title="Noise independence" rows={evidence.invertibility} columns={noiseColumns} rowKey={row => String(row.node)} noun="variable" empty="No noise-independence checks were reported." frame="none" exportName={`noise-independence-${record.id}`} />
  </section>
})
