import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { scatterMatrixOption, type MatrixColumn } from '@/charts/data/scatterMatrix'
import { useChartTheme } from '@/charts/theme'
import { button } from '@/components/ui/recipes'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { mapNonEmpty } from '@/domain/dop'
import type { RootCauseGraph } from '@/domain/rootCause'
import type { SelectedSource } from '@/domain/workflow'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'

type Data = { readonly kind: 'idle' } | { readonly kind: 'loading' } | { readonly kind: 'failed'; readonly detail: string } | { readonly kind: 'ready'; readonly columns: readonly MatrixColumn[]; readonly rows: number }

export function RootCauseData({ graph, source, profile, prepared }: { readonly graph: RootCauseGraph; readonly source: SelectedSource; readonly profile: DatasetProfile; readonly prepared: PreparedDatasetArtifact }) {
  const [data, setData] = useState<Data>({ kind: 'idle' })
  const [selected, setSelected] = useState<readonly string[]>(graph.nodes.slice(0, 6).map((node) => node.name))
  const theme = useChartTheme()
  const option = useMemo(() => data.kind === 'ready' && selected.length > 0 ? scatterMatrixOption(data.columns.filter((column) => selected.includes(column.name)), theme) : null, [data, selected, theme])
  const load = async () => {
    setData({ kind: 'loading' })
    try {
      const { materialisePrepared, describePreparedMaterialisationProblem } = await import('@/data/prepared')
      const result = await materialisePrepared(source, profile, prepared, mapNonEmpty(graph.nodes, (node) => node.column))
      if (!result.ok) { setData({ kind: 'failed', detail: describePreparedMaterialisationProblem(result.error) }); return }
      const { rowCount, values } = result.value
      if (values.some((value) => !Number.isFinite(value))) { setData({ kind: 'failed', detail: 'Resolve missing or nonfinite values before plotting the baseline model data.' }); return }
      setData({ kind: 'ready', rows: rowCount, columns: graph.nodes.map((node, column) => ({ name: node.name, values: Array.from(values.subarray(column * rowCount, (column + 1) * rowCount)) })) })
    } catch (error: unknown) { setData({ kind: 'failed', detail: error instanceof Error ? error.message : String(error) }) }
  }
  return <details className="text-body"><summary className="cursor-pointer text-muted">Baseline data relationships</summary><div className="mt-3 space-y-3">
    {data.kind !== 'ready' && <button type="button" className={button('outline')} disabled={data.kind === 'loading'} onClick={() => void load()}>{data.kind === 'loading' ? 'Loading baseline data…' : 'Explore baseline data'}</button>}
    {data.kind === 'failed' && <p role="alert" className="text-warn">{data.detail}</p>}
    {data.kind === 'ready' && <>
      <p className="text-muted">Each off-diagonal panel plots two variables across all {data.rows.toLocaleString()} observations. Diagonal panels show each variable’s distribution. These patterns describe association, not causal direction.</p>
      <fieldset>
        <legend className="mb-2 font-medium">Variables to plot</legend>
        <div className="flex flex-wrap items-center gap-x-3 gap-y-2">
          <SelectionActions compact selectLabel="Select all variables" clearLabel="Clear selected variables" onSelectAll={() => setSelected(data.columns.map((column) => column.name))} onClear={() => setSelected([])} />
          {data.columns.map((column) => <label key={column.name} className="flex min-h-11 items-center gap-2"><input type="checkbox" checked={selected.includes(column.name)} onChange={() => setSelected((current) => current.includes(column.name) ? current.filter((name) => name !== column.name) : [...current, column.name])} />{column.name}</label>)}
        </div>
      </fieldset>
      {option !== null && <div className="overflow-x-auto"><div style={{ minWidth: Math.max(400, selected.length * 140) }}><ExpandableChart option={option} label="Baseline scatter matrix" testId="root-cause-scatter-matrix" style={{ height: Math.max(400, selected.length * 140) }} defaultWidth={1100} defaultHeight={850} /></div></div>}
    </>}
  </div></details>
}
