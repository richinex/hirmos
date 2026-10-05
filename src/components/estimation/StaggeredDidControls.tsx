import { useMemo } from 'react'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SettingsDisclosure } from '@/components/ui/SettingsDisclosure'
import { Select } from '@/components/ui/Select'
import { field, fieldLabel, fieldRow, settingsStack } from '@/components/ui/recipes'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { PanelBinding } from '@/domain/estimationDraft'
import { describeDidCovariateRole, staggeredCovariateRestrictions } from '@/domain/adjustedDid'
import { useDidCovariates } from './useDidCovariates'
import { DidCovariateChecklist } from './DidCovariateChecklist'
import {
  recordedStaggeredAdjustment,
  recordedStaggeredHeadline,
  staggeredAdjustmentDescriptions,
  staggeredHeadlineDescriptions,
  type StaggeredConfiguration,
} from '@/domain/staggeredDid'

export function StaggeredDidControls({
  section,
  configuration,
  candidates,
  clusterCandidates,
  onChange,
  inspect,
}: {
  readonly configuration: StaggeredConfiguration
  readonly candidates: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly clusterCandidates: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly onChange: (value: StaggeredConfiguration) => void
  readonly section: 'comparison' | 'reporting'
  /** Given only where the covariate list is shown, so the panel is read once. */
  readonly inspect?: {
    readonly file: File
    readonly profile: DatasetProfile
    readonly binding: PanelBinding | null
  }
}) {
  const inspection = useDidCovariates(inspect ?? null, candidates)
  // A scan of the whole panel: once per read, not on every keystroke.
  const restrictions = useMemo(
    () =>
      new Map(
        inspection.kind === 'ready'
          ? staggeredCovariateRestrictions(inspection.matrix).map((item) => [
              item.column,
              describeDidCovariateRole(item.role),
            ])
          : [],
      ),
    [inspection],
  )
  const spec = configuration.specification
  const adjustment = recordedStaggeredAdjustment(spec)
  const headline = recordedStaggeredHeadline(configuration)
  const update = (value: Partial<typeof spec>) =>
    onChange({ ...configuration, specification: { ...spec, ...value } })
  // Two settings steps, so the method step does not stand alone above a column of empty space.
  if (section === 'comparison')
    return (
      <div className={settingsStack} data-testid="staggered-did-comparison">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Adjustment method"
            help={staggeredAdjustmentDescriptions[adjustment.kind].description}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Staggered adjustment method"
            value={adjustment.kind}
            onChange={(kind) => update({ adjustment: { kind } })}
            options={[
              { value: 'doublyRobust', label: 'Doubly robust' },
              { value: 'outcomeRegression', label: 'Outcome regression' },
              { value: 'inverseProbability', label: 'Inverse probability weighting' },
            ]}
          />
        </div>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Comparison group"
            help="Never-treated units do not adopt in the observed panel. Not-yet-treated comparisons use units still untreated beyond the comparison and anticipation window. When every unit eventually adopts, preparation records any restrictions or recoding."
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Staggered comparison group"
            value={spec.controls}
            onChange={(controls) => update({ controls })}
            options={[
              { value: 'never-treated', label: 'Never treated' },
              { value: 'not-yet-treated', label: 'Not yet treated' },
            ]}
          />
        </div>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Pre-treatment baseline"
            help="Varying baselines compare successive pre-treatment periods. Universal baselines normalize the period immediately before the anticipation window to zero. Post-treatment group-time estimates use the same baseline under either choice."
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Staggered baseline"
            value={spec.baseline}
            onChange={(baseline) => update({ baseline })}
            options={[
              { value: 'varying', label: 'Varying' },
              { value: 'universal', label: 'Universal' },
            ]}
          />
        </div>
        <div className="max-w-3xl">
          <div className="flex items-center justify-between gap-2">
            <ParameterLabel
              className={fieldLabel}
              label="Adjustment covariates"
              help={`Without covariates, comparisons use outcome changes. ${staggeredAdjustmentDescriptions[adjustment.kind].description} Covariates are taken at the comparison baseline; do not include variables affected by treatment.`}
            />
            <SelectionActions
              selectLabel="Select all staggered DiD covariates"
              clearLabel="Clear staggered DiD covariates"
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
            label="Staggered DiD covariates"
            candidates={candidates}
            selected={configuration.covariates}
            restrictions={restrictions}
            check={
              inspection.kind === 'pending'
                ? 'pending'
                : inspection.kind === 'ready'
                  ? 'checked'
                  : 'unchecked'
            }
            onChange={(covariates) => onChange({ ...configuration, covariates: [...covariates] })}
          />
        </div>
        <label className="block max-w-xs">
          <ParameterLabel
            className={fieldLabel}
            label="Anticipation periods"
            help="The number of periods before adoption in which treatment may already affect outcomes."
          />
          <input
            aria-label="Anticipation periods"
            className={field('text', 'mt-1 w-full')}
            type="number"
            min={0}
            step={1}
            value={spec.anticipation}
            onChange={(e) => update({ anticipation: Number(e.target.value) })}
          />
        </label>
      </div>
    )
  return (
    <div className={settingsStack} data-testid="staggered-did-controls">
      <label>
        <ParameterLabel
          className={fieldLabel}
          label="Independent clusters"
          help="By default, each panel unit is a cluster. Select a column when units belong to larger independent groups, such as firms within states. Each unit must remain in one cluster. Additional clustering requires bootstrap inference."
        />
        <Select
          aria-label="Staggered cluster column"
          className={field('text', 'mt-1 w-full max-w-xs')}
          value={configuration.clustering?.kind === 'column' ? configuration.clustering.column : ''}
          onChange={(e) => {
            if (e.target.value === '') {
              onChange({ ...configuration, clustering: { kind: 'unit' } })
              return
            }
            const column = clusterCandidates.find((c) => c.id === e.target.value)
            if (column !== undefined)
              onChange({ ...configuration, clustering: { kind: 'column', column: column.id } })
          }}
        >
          <option value="">Each panel unit</option>
          {clusterCandidates.map((c) => (
            <option key={c.id} value={c.id}>
              {c.name}
            </option>
          ))}
        </Select>
      </label>
      {configuration.clustering?.kind === 'column' && spec.inference.kind === 'analytical' && (
        <p className="m-0 text-body text-muted" role="status">
          Select pointwise or simultaneous bootstrap to use the cluster column.
        </p>
      )}
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Overall ATT"
          help={`${staggeredHeadlineDescriptions[headline].description} Every average is computed in the same run; this choice sets which one is the headline.`}
        />
        <SegmentedControl
          className="mt-1"
          ariaLabel="Staggered overall ATT"
          value={headline}
          onChange={(value) => onChange({ ...configuration, headline: value })}
          options={(['dynamic', 'group', 'calendar', 'simple'] as const).map((value) => ({
            value,
            label: staggeredHeadlineDescriptions[value].label,
          }))}
        />
      </div>
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Uncertainty"
          help="This choice changes standard errors and intervals, not ATT estimates. Analytical uses pointwise intervals; simultaneous bootstrap covers each plotted family jointly. Overall ATT intervals remain pointwise. Bootstrap resampling uses the selected independent clusters."
        />
        <SegmentedControl
          className="mt-1"
          ariaLabel="Staggered uncertainty"
          value={spec.inference.kind}
          onChange={(kind) =>
            update({
              inference:
                kind === 'analytical'
                  ? { kind }
                  : {
                      kind,
                      iterations:
                        spec.inference.kind === 'analytical' ? 999 : spec.inference.iterations,
                      seed: spec.inference.kind === 'analytical' ? 731 : spec.inference.seed,
                    },
            })
          }
          options={[
            { value: 'analytical', label: 'Analytical' },
            { value: 'bootstrapPointwise', label: 'Pointwise bootstrap' },
            { value: 'bootstrapSimultaneous', label: 'Simultaneous bootstrap' },
          ]}
        />
      </div>
      <div className={fieldRow.three}>
        <label>
          <span className={fieldLabel}>Confidence level</span>
          <input
            aria-label="Staggered confidence level"
            className={field('text', 'mt-1 w-full')}
            type="number"
            min={0.01}
            max={0.999}
            step={0.01}
            value={spec.confidence}
            onChange={(e) => update({ confidence: Number(e.target.value) })}
          />
        </label>
        {spec.inference.kind !== 'analytical' && (
          <>
            <label>
              <span className={fieldLabel}>Replications</span>
              <input
                aria-label="Staggered bootstrap replications"
                className={field('text', 'mt-1 w-full')}
                type="number"
                min={1}
                step={1}
                value={spec.inference.iterations}
                onChange={(e) => {
                  if (spec.inference.kind !== 'analytical')
                    update({ inference: { ...spec.inference, iterations: Number(e.target.value) } })
                }}
              />
            </label>
            <label>
              <span className={fieldLabel}>Seed</span>
              <input
                aria-label="Staggered bootstrap seed"
                className={field('text', 'mt-1 w-full')}
                type="number"
                min={0}
                step={1}
                value={spec.inference.seed}
                onChange={(e) => {
                  if (spec.inference.kind !== 'analytical')
                    update({ inference: { ...spec.inference, seed: Number(e.target.value) } })
                }}
              />
            </label>
          </>
        )}
      </div>
      <SettingsDisclosure
        title="Event window"
        items={[
          {
            icon: 'first_page',
            text:
              spec.firstEvent === null
                ? 'earliest event time'
                : `from event time ${spec.firstEvent}`,
          },
          {
            icon: 'last_page',
            text: spec.lastEvent === null ? 'latest event time' : `to event time ${spec.lastEvent}`,
          },
          {
            icon: 'balance',
            text: spec.balance === null ? 'all cohorts' : `balanced through ${spec.balance}`,
          },
        ]}
      >
        <div className={fieldRow.three}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="First event time"
              help="Periods relative to adoption. Leave blank to retain all supported event times."
            />
            <input
              aria-label="First event time"
              className={field('text', 'mt-1 w-full')}
              type="number"
              step={1}
              value={spec.firstEvent ?? ''}
              onChange={(e) =>
                update({ firstEvent: e.target.value === '' ? null : Number(e.target.value) })
              }
            />
          </label>
          <label>
            <span className={fieldLabel}>Last event time</span>
            <input
              aria-label="Last event time"
              className={field('text', 'mt-1 w-full')}
              type="number"
              step={1}
              value={spec.lastEvent ?? ''}
              onChange={(e) =>
                update({ lastEvent: e.target.value === '' ? null : Number(e.target.value) })
              }
            />
          </label>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Balance through"
              help="Keep cohorts observed for at least this many post-adoption periods and restrict dynamic effects accordingly. Leave blank for all available cohort support. This changes the dynamic aggregation, not the cohort or calendar summaries."
            />
            <input
              aria-label="Balance event support through"
              className={field('text', 'mt-1 w-full')}
              type="number"
              min={0}
              step={1}
              value={spec.balance ?? ''}
              onChange={(e) =>
                update({ balance: e.target.value === '' ? null : Number(e.target.value) })
              }
            />
          </label>
        </div>
      </SettingsDisclosure>
    </div>
  )
}
