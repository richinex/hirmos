import { useMemo } from 'react'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldLabel, fieldRow, settingsStack } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { PanelBinding } from '@/domain/estimationDraft'
import type { PanelInterventionConfiguration } from '@/domain/estimation'
import {
  didNormalizationLabels,
  didCovariateRestrictions,
  describeDidCovariateRole,
} from '@/domain/adjustedDid'
import { useDidCovariates } from './useDidCovariates'
import { DidCovariateChecklist } from './DidCovariateChecklist'
type Configuration = Extract<PanelInterventionConfiguration, { primary: 'adjusted' }>
export function AdjustedDidControls({
  configuration,
  candidates,
  onChange,
  file,
  profile,
  binding,
}: {
  readonly configuration: Configuration
  readonly candidates: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly onChange: (value: Configuration) => void
  readonly file: File
  readonly profile: DatasetProfile
  readonly binding: PanelBinding | null
}) {
  const spec = configuration.specification
  const inspection = useDidCovariates({ file, profile, binding }, candidates)
  // A scan of the whole panel: once per read and specification, not on every keystroke.
  const restrictions = useMemo(
    () =>
      new Map(
        inspection.kind === 'ready' && inspection.layout !== null
          ? didCovariateRestrictions(inspection.matrix, inspection.layout, spec).map((item) => [
              item.column,
              describeDidCovariateRole(item.role),
            ])
          : [],
      ),
    [inspection, spec],
  )
  return (
    <div className={settingsStack}>
      <div className="max-w-3xl">
        <div className="flex items-center justify-between gap-2">
          <ParameterLabel
            className={fieldLabel}
            label={spec.kind === 'regression' ? 'Covariates' : 'Baseline covariates'}
            help={
              spec.kind === 'regression'
                ? 'Regression uses the selected measurements in each of the two periods. Avoid adjusting for variables caused by treatment.'
                : 'DR DiD uses only each unit’s pre-treatment measurements, with paired outcome differences. It fits linear outcome regression and unpenalized logistic propensity models.'
            }
          />
          <SelectionActions
            selectLabel="Select all DiD covariates"
            clearLabel="Clear DiD covariates"
            onSelectAll={() =>
              onChange({
                ...configuration,
                covariates: candidates.filter((c) => !restrictions.has(c.id)).map((c) => c.id),
              })
            }
            onClear={() => onChange({ ...configuration, covariates: [] })}
          />
        </div>
        <DidCovariateChecklist
          label="DiD covariates"
          candidates={candidates}
          selected={configuration.covariates}
          restrictions={restrictions}
          check={
            inspection.kind === 'pending'
              ? 'pending'
              : inspection.kind === 'ready' && inspection.layout !== null
                ? 'checked'
                : 'unchecked'
          }
          onChange={(covariates) => onChange({ ...configuration, covariates: [...covariates] })}
        />
      </div>
      {spec.kind === 'doublyRobust' && (
        <div className={fieldRow.two}>
          <label>
            <span className={fieldLabel}>Folds</span>
            <input
              aria-label="DiD folds"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={2}
              step={1}
              value={spec.folds}
              onChange={(e) =>
                onChange({
                  ...configuration,
                  specification: {
                    ...spec,
                    folds: Math.max(2, Math.trunc(Number(e.target.value) || 2)),
                  },
                })
              }
            />
          </label>
          <label>
            <span className={fieldLabel}>Seed</span>
            <input
              aria-label="DiD seed"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={0}
              max={4294967295}
              step={1}
              value={spec.seed}
              onChange={(e) =>
                onChange({
                  ...configuration,
                  specification: {
                    ...spec,
                    seed: Math.max(
                      0,
                      Math.min(4294967295, Math.trunc(Number(e.target.value) || 0)),
                    ),
                  },
                })
              }
            />
          </label>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Propensity trimming"
              help="Estimated probabilities are clipped to this threshold and one minus it. No units are deleted; clipping does not repair absent overlap."
            />
            <input
              aria-label="DiD trimming"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={0}
              max={0.5}
              step="any"
              value={spec.trimming}
              onChange={(e) => {
                const value = Number(e.target.value)
                if (value > 0 && value < 0.5)
                  onChange({ ...configuration, specification: { ...spec, trimming: value } })
              }}
            />
          </label>
          <div>
            <ParameterLabel
              className={fieldLabel}
              label="Normalization"
              help="Choose the DoubleML in-sample normalized weights or population normalization. This is part of the score specification."
            />
            <SegmentedControl
              ariaLabel="DiD normalization"
              className="mt-1"
              value={spec.normalization}
              onChange={(normalization) =>
                onChange({ ...configuration, specification: { ...spec, normalization } })
              }
              options={(['in-sample', 'population'] as const).map((value) => ({
                value,
                label: didNormalizationLabels[value],
              }))}
            />
          </div>
        </div>
      )}
    </div>
  )
}
