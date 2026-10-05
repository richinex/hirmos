import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { fieldLabel } from '@/components/ui/recipes'
import { Alert } from '@/components/ui/Alert'
import {
  changeRegressionAnalysis,
  regressionAnalyses,
  regressionDesignInfo,
  selectRegressionDesign,
  type RegressionDesign,
  type RegressionOutcomeModel,
  type CountOutcomeReadiness,
} from '@/domain/regressionDesigns'

export function RegressionDesignControls({
  design,
  panel,
  busy,
  countReadiness,
  onChange,
}: {
  readonly design: RegressionDesign
  readonly panel: boolean
  readonly busy: boolean
  readonly countReadiness: CountOutcomeReadiness
  readonly onChange: (design: RegressionDesign) => void
}) {
  const info = regressionDesignInfo(design)
  const countReason =
    countReadiness.kind === 'refused'
      ? countReadiness.reason
      : countReadiness.kind === 'checking'
        ? 'Checking the prepared outcome values.'
        : undefined
  const modelOptions = (['linear', 'count'] as const).map((model) => {
    const choice = selectRegressionDesign(info.analysis, model)
    const reason = !choice.ok ? choice.error : model === 'count' ? countReason : undefined
    return {
      value: model,
      label: model === 'linear' ? 'Linear' : 'Count (negative binomial)',
      ariaLabel: model === 'linear' ? 'Linear' : 'Count (negative binomial)',
      disabled: reason !== undefined,
      title: reason,
    }
  })
  const changeModel = (model: RegressionOutcomeModel) => {
    const selected = selectRegressionDesign(info.analysis, model)
    if (selected.ok) onChange(selected.value)
  }
  return (
    <div className="grid gap-3">
      <SegmentedControl
        className="justify-self-start"
        ariaLabel="Regression analysis"
        value={info.analysis}
        disabled={busy}
        options={regressionAnalyses.map((analysis) => ({
          ...analysis,
          disabled: analysis.requiresPanel && !panel,
          title:
            analysis.requiresPanel && !panel
              ? 'Prepare panel data to use adoption timing.'
              : undefined,
        }))}
        onChange={(analysis) => {
          const selected = changeRegressionAnalysis(design, analysis)
          if (selected.ok) onChange(selected.value)
        }}
      />
      {info.model !== null && (
        <div>
          <span className={fieldLabel}>Outcome model</span>
          <SegmentedControl
            className="mt-1"
            ariaLabel="Outcome model"
            value={info.model}
            disabled={busy}
            options={modelOptions}
            onChange={changeModel}
          />
        </div>
      )}
      {info.model !== null && countReadiness.kind === 'refused' && (
        <Alert tone="info" live={false}>
          {countReadiness.reason}
        </Alert>
      )}
    </div>
  )
}

/** The side panel's head for the selected design: its name, outcome model and description, so the stage keeps only the choices. */
export function RegressionDesignSummary({ design }: { readonly design: RegressionDesign }) {
  const info = regressionDesignInfo(design)
  const analysis = regressionAnalyses.find((candidate) => candidate.value === info.analysis)
  return (
    <section aria-labelledby="regression-design-title">
      <h3 id="regression-design-title" className="m-0 mt-1 text-body font-medium text-ink">
        {analysis?.label}
      </h3>
      {info.model !== null && (
        <p className="mb-0 mt-1 text-label text-muted">
          Outcome model: {info.model === 'linear' ? 'Linear' : 'Count (negative binomial)'}
        </p>
      )}
      <p className="mb-0 mt-2 text-body text-muted">{info.description}</p>
      {info.analysis === 'cohort-summary' && (
        <p className="mb-0 mt-2 text-body text-muted">
          A linear cohort-summary specification is not implemented.
        </p>
      )}
    </section>
  )
}
