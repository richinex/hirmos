import { NumberInput } from '@/components/ui/NumberInput'
import { memo } from 'react'
import { usePanelCatalog } from './usePanelCatalog'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Icon } from '@/components/Icon'
import { button, field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import {
  predictorSummarySchema,
  type PredictorSyntheticCatalog,
  type PredictorSyntheticConfiguration,
} from '@/domain/predictorSyntheticControl'
import type { DatasetProfile, ColumnId } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'

const summaries = predictorSummarySchema.options
const summaryLabels = {
  mean: 'Mean',
  median: 'Median',
  minimum: 'Minimum',
  maximum: 'Maximum',
  sum: 'Sum',
  variance: 'Sample variance',
  'standard-deviation': 'Standard deviation',
}

function Periods({
  title,
  help,
  periods,
  selected,
  onChange,
}: {
  readonly title: string
  readonly help: string
  readonly periods: PredictorSyntheticCatalog['periods']
  readonly selected: readonly number[]
  readonly onChange: (periods: readonly number[]) => void
}) {
  return (
    <div className="max-h-48 overflow-y-auto rounded-md bg-well p-3">
      <ColumnChecklist
        title={title}
        help={help}
        columns={periods.map((period) => ({ id: String(period.code), name: period.label }))}
        selected={selected.map(String)}
        onChange={(values) => onChange(values.map(Number).sort((a, b) => a - b))}
      />
    </div>
  )
}

export const PredictorSyntheticControls = memo(function PredictorSyntheticControls({
  configuration,
  onChange,
  source,
  profile,
  prepared,
  treatment,
}: {
  readonly configuration: PredictorSyntheticConfiguration
  readonly onChange: (configuration: PredictorSyntheticConfiguration) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly treatment: ColumnId | null
}) {
  const job = usePanelCatalog(source, profile, prepared)
  if (job.kind !== 'ready')
    return (
      <p className={fieldHint}>
        {job.kind === 'loading' ? 'Reading panel units and periods…' : job.detail}
      </p>
    )
  const catalog = job.catalog
  const columns = profile.columns
    .filter((column) => prepared.columns.includes(column.id) && column.id !== treatment)
    .map((column) => ({ id: column.id, name: column.name }))
  const before = catalog.periods.filter(
    (period) =>
      configuration.interventionPeriod !== null && period.code < configuration.interventionPeriod,
  )
  const unitOptions = catalog.units.map((unit) => ({ id: unit.label, name: unit.label }))
  const changeSpecial = (index: number, next: PredictorSyntheticConfiguration['special'][number]) =>
    onChange({
      ...configuration,
      special: configuration.special.map((value, i) => (i === index ? next : value)),
    })
  const predictorCount = configuration.predictors.length + configuration.special.length
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Define the comparison">
        <div className={fieldRow.two}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Treated unit"
              help="Choose the unit exposed to the intervention."
            />
            <Select
              className={field('text', 'mt-1 w-full')}
              aria-label="Synthetic treated unit"
              value={configuration.treatedUnit ?? ''}
              onChange={(event) =>
                onChange({
                  ...configuration,
                  treatedUnit: event.target.value || null,
                  donorUnits: configuration.donorUnits.filter(
                    (unit) => unit !== event.target.value,
                  ),
                })
              }
            >
              <option value="">Choose a unit</option>
              {catalog.units.map((unit) => (
                <option key={unit.label} value={unit.label}>
                  {unit.label}
                </option>
              ))}
            </Select>
          </label>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Intervention period"
              help="The gap is summarized over selected plot periods at or after this period."
            />
            <Select
              className={field('text', 'mt-1 w-full')}
              aria-label="Synthetic intervention period"
              value={
                configuration.interventionPeriod === null
                  ? ''
                  : String(configuration.interventionPeriod)
              }
              onChange={(event) => {
                const period = catalog.periods.find(
                  (period) => String(period.code) === event.target.value,
                )
                if (period === undefined) return
                const pre = catalog.periods.filter((p) => p.code < period.code).map((p) => p.code)
                onChange({
                  ...configuration,
                  interventionPeriod: period.code,
                  predictorPeriods: pre,
                  fitPeriods: pre,
                  plotPeriods: catalog.periods.map((p) => p.code),
                })
              }}
            >
              <option value="">Choose a period</option>
              {catalog.periods.map((period) => (
                <option key={period.code} value={period.code}>
                  {period.label}
                </option>
              ))}
            </Select>
          </label>
        </div>
        <div className="max-h-56 overflow-y-auto rounded-md bg-well p-3">
          <ColumnChecklist
            title="Donor units"
            help="Choose untreated units to form the synthetic control. Their weights are non-negative and sum to one."
            columns={unitOptions}
            selected={configuration.donorUnits}
            reserved={configuration.treatedUnit === null ? [] : [configuration.treatedUnit]}
            onChange={(donorUnits) => onChange({ ...configuration, donorUnits })}
          />
        </div>
      </SettingsStep>
      <SettingsStep number={2} title="Summarize predictors">
        <ColumnChecklist
          title="Ordinary predictors"
          help="Summarize these predictors over the predictor periods. The outcome may also be used as a pre-intervention predictor."
          columns={columns}
          selected={configuration.predictors}
          onChange={(predictors) => onChange({ ...configuration, predictors })}
        />
        <label className="block max-w-xs">
          <ParameterLabel
            className={fieldLabel}
            label="Predictor summary"
            help="This operation summarizes the treated unit. Ordinary donor predictors use the mean."
          />
          <Select
            aria-label="Predictor summary"
            className={field('text', 'mt-1 w-full')}
            value={configuration.summary}
            onChange={(event) => {
              const summary = summaries.find((value) => value === event.target.value)
              if (summary !== undefined) onChange({ ...configuration, summary })
            }}
          >
            {summaries.map((summary) => (
              <option key={summary} value={summary}>
                {summaryLabels[summary]}
              </option>
            ))}
          </Select>
        </label>
        <Periods
          title="Predictor periods"
          help="Summarize ordinary predictors over these pre-intervention periods."
          periods={before}
          selected={configuration.predictorPeriods}
          onChange={(predictorPeriods) => onChange({ ...configuration, predictorPeriods })}
        />
        {configuration.special.map((predictor, index) => (
          <div className="grid gap-3 rounded-md bg-well p-3" key={index}>
            <div className={fieldRow.two}>
              <label>
                <span className={fieldLabel}>Period-specific predictor {index + 1}</span>
                <Select
                  aria-label={`Period-specific predictor ${index + 1}`}
                  className={field('text', 'mt-1 w-full')}
                  value={predictor.column}
                  onChange={(event) => {
                    const column = columns.find((c) => c.id === event.target.value)
                    if (column !== undefined)
                      changeSpecial(index, { ...predictor, column: column.id })
                  }}
                >
                  {columns.map((column) => (
                    <option key={column.id} value={column.id}>
                      {column.name}
                    </option>
                  ))}
                </Select>
              </label>
              <label>
                <span className={fieldLabel}>Summary operation</span>
                <Select
                  aria-label={`Special predictor ${index + 1} summary`}
                  className={field('text', 'mt-1 w-full')}
                  value={predictor.summary}
                  onChange={(event) => {
                    const summary = summaries.find((value) => value === event.target.value)
                    if (summary !== undefined) changeSpecial(index, { ...predictor, summary })
                  }}
                >
                  {summaries.map((summary) => (
                    <option key={summary} value={summary}>
                      {summaryLabels[summary]}
                    </option>
                  ))}
                </Select>
              </label>
            </div>
            <Periods
              title={`Special predictor ${index + 1} periods`}
              help="This predictor uses its own periods and summary operation."
              periods={before}
              selected={predictor.periods}
              onChange={(periods) => changeSpecial(index, { ...predictor, periods })}
            />
            <button
              type="button"
              className={button('quiet', 'w-full')}
              onClick={() =>
                onChange({
                  ...configuration,
                  special: configuration.special.filter((_, i) => i !== index),
                })
              }
            >
              <Icon name="delete" size={16} />
              Remove predictor
            </button>
          </div>
        ))}
        <button
          type="button"
          className={button('quiet', 'justify-self-start')}
          disabled={columns.length === 0 || before.length === 0}
          onClick={() => {
            const column = columns[0]
            if (column !== undefined)
              onChange({
                ...configuration,
                special: [
                  ...configuration.special,
                  { column: column.id, periods: before.map((p) => p.code), summary: 'mean' },
                ],
              })
          }}
        >
          <Icon name="add" size={16} />
          Add period-specific predictor
        </button>
      </SettingsStep>
      <SettingsStep number={3} title="Choose fitting and plot periods">
        <Periods
          title="Fitting periods"
          help="Choose the pre-intervention periods used to minimize outcome prediction error."
          periods={before}
          selected={configuration.fitPeriods}
          onChange={(fitPeriods) => onChange({ ...configuration, fitPeriods })}
        />
        <Periods
          title="Plot periods"
          help="Choose the periods shown for the treated outcome, synthetic control and gap."
          periods={catalog.periods}
          selected={configuration.plotPeriods}
          onChange={(plotPeriods) => onChange({ ...configuration, plotPeriods })}
        />
      </SettingsStep>
      <SettingsStep number={4} title="Choose predictor weights">
        <SegmentedControl
          fill
          ariaLabel="Predictor weights"
          value={configuration.selection.kind}
          options={[
            { value: 'automatic', label: 'Estimate' },
            { value: 'supplied', label: 'Specify' },
          ]}
          onChange={(kind) =>
            onChange({
              ...configuration,
              selection:
                kind === 'automatic'
                  ? { kind }
                  : { kind, weights: Array.from({ length: predictorCount }, () => 1) },
            })
          }
        />
        <p className={fieldHint}>
          Estimated predictor weights minimize the mean squared prediction error of the outcome over
          the fitting periods. Specified weights are normalized before fitting donor weights.
        </p>
        {configuration.selection.kind === 'supplied' && (
          <div className={fieldRow.two}>
            {Array.from({ length: predictorCount }, (_, index) => (
              <label key={index}>
                <span className={fieldLabel}>Predictor weight {index + 1}</span>
                <NumberInput
                  min={0}
                  step="any"
                  aria-label={`Predictor weight ${index + 1}`}
                  className={field('text', 'mt-1 w-full')}
                  value={
                    configuration.selection.kind === 'supplied'
                      ? (configuration.selection.weights[index] ?? 0)
                      : 0
                  }
                  onChange={(event) => {
                    if (configuration.selection.kind !== 'supplied') return
                    const value = Number(event.target.value)
                    if (!Number.isFinite(value) || value < 0) return
                    onChange({
                      ...configuration,
                      selection: {
                        kind: 'supplied',
                        weights: Array.from({ length: predictorCount }, (_, i) =>
                          i === index
                            ? value
                            : configuration.selection.kind === 'supplied'
                              ? (configuration.selection.weights[i] ?? 0)
                              : 0,
                        ),
                      },
                    })
                  }}
                />
              </label>
            ))}
          </div>
        )}
      </SettingsStep>
    </div>
  )
})
