import { useState } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import { useJob } from '@/analysis/JobsProvider'
import { calculateRawBalance } from '@/analysis/client'
import { materialisePrepared, describePreparedMaterialisationProblem } from '@/data/prepared'
import { expandDesign, describeDesignExpansionProblem } from '@/domain/designMatrix'
import type { StudySpecification, StudyVariable } from '@/domain/study'
import type { ColumnId } from '@/domain/dataset'
import type { NonEmptyArray } from '@/domain/dop'
import type { RawBalance } from '@/domain/covariateBalance'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { RunActions } from '@/components/ui/RunActions'
import { JobNotice } from '@/components/ui/JobNotice'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { button, well } from '@/components/ui/recipes'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'

type LabeledRow = RawBalance['rows'][number] & { readonly name: string }
export function RawBalanceTable({
  study,
  covariates,
}: {
  readonly study: StudySpecification
  readonly covariates: readonly StudyVariable[]
}) {
  const workflow = useWorkflow((s) => s.workflow)
  const draft = useWorkflow((s) => s.estimationDraft)
  const change = useWorkflow((s) => s.changeEstimation)
  const job = useJob('raw-balance:' + study.id)
  const [completed, setCompleted] = useState<{ key: string; rows: readonly LabeledRow[] } | null>(
    null,
  )
  const selected = covariates
    .filter(
      (v) =>
        draft?.prepared === study.preparedDataset &&
        draft.draft.encodings[v.column]?.kind === 'categorical',
    )
    .map((v) => v.column)
  const key = JSON.stringify([
    study.id,
    study.preparedDataset,
    covariates.map((v) => v.column),
    selected,
  ])
  const rows =
    workflow.kind === 'profiled' &&
    workflow.prepared?.id === study.preparedDataset &&
    completed?.key === key
      ? completed.rows
      : null
  const available = workflow.kind === 'profiled' && workflow.prepared?.id === study.preparedDataset
  async function run() {
    if (workflow.kind !== 'profiled' || workflow.prepared?.id !== study.preparedDataset) return
    const id = job.start('analysis', 'Raw covariate balance')
    if (id === null) return
    try {
      const columns: NonEmptyArray<ColumnId> = [
        study.treatment.column,
        ...covariates.map((v) => v.column),
      ]
      const matrix = await materialisePrepared(
        workflow.source,
        workflow.profile,
        workflow.prepared,
        columns,
      )
      if (!job.current(id)) return
      if (!matrix.ok) {
        job.fail(id, describePreparedMaterialisationProblem(matrix.error))
        return
      }
      const design = expandDesign(
        matrix.value.values,
        matrix.value.rowCount,
        columns.length,
        columns.map((c, i) =>
          i > 0 && selected.includes(c) ? { kind: 'indicators' } : { kind: 'numeric' },
        ),
      )
      if (!design.ok) {
        job.fail(
          id,
          describeDesignExpansionProblem(design.error, [
            study.treatment.name,
            ...covariates.map((v) => v.name),
          ]),
        )
        return
      }
      const names = covariates.flatMap((v, i) =>
        design.value.levels[i + 1]!.length === 0
          ? [v.name]
          : design.value.levels[i + 1]!.map((level) => v.name + ' = ' + String(level)),
      )
      const result = await calculateRawBalance(
        design.value.values,
        matrix.value.rowCount,
        design.value.columnCount,
        study.estimand.kind === 'average-treatment-effect-on-treated',
      )
      if (!job.current(id)) return
      if (!result.ok) {
        job.fail(id, describeAnalysisWorkerProblem(result.error))
        return
      }
      setCompleted({ key, rows: result.value.rows.map((r) => ({ ...r, name: names[r.column]! })) })
      job.finish(id)
    } catch (e) {
      job.fail(id, e instanceof Error ? e.message : 'Raw balance could not be calculated.')
    }
  }
  if (covariates.length === 0) return null
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <DisclosureSummary icon="balance" className="cursor-pointer text-ink">
        Raw covariate balance
      </DisclosureSummary>
      <div className="mt-2 space-y-3">
        <ColumnChecklist
          title="Categorical covariates"
          help="Select covariates whose values represent categories. Each category is assessed separately. These declarations also apply in Estimation."
          columns={covariates.map((v) => ({ id: v.column, name: v.name }))}
          selected={selected}
          onChange={(next) => {
            for (const v of covariates)
              change(study.preparedDataset, {
                type: 'encoding-declared',
                column: v.column,
                encoding: { kind: next.includes(v.column) ? 'categorical' : 'numeric' },
              })
          }}
        />
        <RunActions
          running={job.job.kind === 'running'}
          onCancel={job.cancel}
          orbLabel="Calculating covariate balance"
        >
          <button
            type="button"
            className={button('outline', undefined, 'sm')}
            disabled={!available || job.blocked || job.job.kind === 'running'}
            onClick={() => void run()}
          >
            Calculate raw balance
          </button>
        </RunActions>
        <JobNotice job={job.job} />
        {rows !== null && (
          <EvidenceTable
            title="Raw covariate balance"
            help={
              'Standardised differences compare treated and untreated covariate means before weighting or matching, using ' +
              (study.estimand.kind === 'average-treatment-effect-on-treated'
                ? 'the treated-group'
                : 'the pooled') +
              ' standard deviation. Binary covariates use Bernoulli variance. Values near zero indicate similar means, not proof of causal identification. Do not remove a required adjustment variable solely because its means are similar.'
            }
            rows={rows}
            rowKey={(r) => String(r.column)}
            noun="covariate"
            empty="No covariates to assess."
            columns={[
              { id: 'name', header: 'Covariate', value: (r) => r.name },
              figureColumn<LabeledRow>('difference', 'SMD', (r) => r.difference),
              {
                id: 'spread',
                header: 'Reference spread',
                value: (r) =>
                  r.usedFullSampleSpread
                    ? 'Full sample (degenerate reference arm)'
                    : study.estimand.kind === 'average-treatment-effect-on-treated'
                      ? 'Treated group'
                      : 'Pooled',
              },
            ]}
          />
        )}
      </div>
    </details>
  )
}
