import { useMemo } from 'react'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import type { TLearnerEvidence } from '@/domain/estimation'
import { formatPercent, formatStatistic } from '@/lib/format/number'

interface Row {
  readonly row: number
  readonly effect: number
  readonly lower: number
  readonly upper: number
  readonly standardError: number
}
const columns: readonly EvidenceColumn<Row>[] = [
  { id: 'row', header: 'Row', value: (row) => row.row, align: 'right' },
  ...(['effect', 'lower', 'upper', 'standardError'] as const).map((id): EvidenceColumn<Row> => ({
    id,
    header: {
      effect: 'Effect',
      lower: 'Lower bound',
      upper: 'Upper bound',
      standardError: 'Standard error',
    }[id],
    value: (row) => row[id],
    align: 'right',
    format: (_, row) => formatStatistic('raw', row[id]).text,
  })),
]
export function TLearnerIntervals({ evidence }: { readonly evidence: TLearnerEvidence }) {
  const uncertainty = evidence.uncertainty
  const rows = useMemo(
    (): Row[] =>
      uncertainty.kind === 'none'
        ? []
        : evidence.effects.map((effect, i) => ({
            row: i + 1,
            effect,
            lower: uncertainty.intervals[i]![0],
            upper: uncertainty.intervals[i]![1],
            standardError: uncertainty.standardErrors[i]!,
          })),
    [evidence.effects, uncertainty],
  )
  if (uncertainty.kind === 'none') return null
  return (
    <EvidenceTable
      title="Row effects and uncertainty"
      help={`Each ${formatPercent(uncertainty.level, { precision: 0 }).text} interval describes uncertainty in that row's estimated effect. The intervals do not provide simultaneous coverage across all rows.`}
      rows={rows}
      columns={columns}
      rowKey={(row) => String(row.row)}
      noun="row"
      empty="No row effects."
      exportName="t-learner-row-effects"
      frame="none"
    />
  )
}
