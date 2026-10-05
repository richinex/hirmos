import { SettingsStep } from '@/components/ui/SettingsStep'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { Select } from '@/components/ui/Select'
import { field, fieldLabel, fieldHint, fieldRow } from '@/components/ui/recipes'
import type { SunAbrahamConfiguration } from '@/domain/sunAbraham'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import type { SunAbrahamPanelJob } from './useSunAbrahamPanel'

export function SunAbrahamControls({
  configuration,
  onChange,
  panel,
}: {
  readonly configuration: SunAbrahamConfiguration
  readonly onChange: (c: SunAbrahamConfiguration) => void
  readonly panel: SunAbrahamPanelJob
}) {
  return (
    <>
      <SettingsStep
        number={2}
        title="Define the event-time comparison"
        help="Estimate cohort-by-event-time interactions with unit and period fixed effects, then aggregate the supported treatment effects. Never-treated units are reference units. You can also select adoption cohorts as references: their observations remain in the fit, but their treatment effects are not estimated. Units treated throughout their observed window are excluded."
      >
        {panel.kind !== 'ready' ? (
          <p className={fieldHint} role="status">
            {panel.kind === 'loading' ? 'Reading adoption cohorts…' : panel.detail}
          </p>
        ) : (
          <>
            <p className={fieldHint}>
              {panel.catalog.neverTreated} never-treated units. Reference cohorts are excluded from
              the reported ATT average. Their effects are normalized throughout the observed period,
              including after adoption.
            </p>
            <div className="max-h-56 overflow-y-auto rounded-md bg-well p-3">
              <ColumnChecklist
                title="Reference adoption cohorts"
                help="Select by adoption period. With no never-treated units, select at least one reference cohort and leave another cohort for estimation."
                columns={panel.catalog.cohorts.map((c) => ({ id: String(c.code), name: c.label }))}
                selected={configuration.referenceCohorts.map(String)}
                onChange={(selected) =>
                  onChange({
                    ...configuration,
                    referenceCohorts: panel.catalog.cohorts
                      .filter((c) => selected.includes(String(c.code)))
                      .map((c) => c.code),
                  })
                }
              />
            </div>
          </>
        )}
        <div className={fieldRow.two}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Reference event period"
              help="The selected period before adoption is normalized to zero, not estimated as a zero effect."
            />
            <Select
              className={field('text', 'mt-1 w-full')}
              aria-label="Sun–Abraham reference period"
              value={String(configuration.referencePeriods[0])}
              onChange={(event) => {
                const selected = [-1, -2, -3, -4].find((v) => String(v) === event.target.value)
                if (selected !== undefined)
                  onChange({ ...configuration, referencePeriods: [selected] })
              }}
            >
              {[-1, -2, -3, -4].map((p) => (
                <option key={p} value={p}>
                  {Math.abs(p)} {p === -1 ? 'period' : 'periods'} before adoption
                </option>
              ))}
            </Select>
          </label>
        </div>
      </SettingsStep>
      <SettingsStep number={3} title="Report uncertainty">
        <div className={fieldRow.two}>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Confidence level"
              help="Pointwise Student-t intervals use unit-clustered covariance and cluster-count degrees of freedom."
            />
            <Select
              className={field('text', 'mt-1 w-full')}
              aria-label="Sun–Abraham confidence level"
              value={String(configuration.confidence)}
              onChange={(event) => {
                const selected = [0.9, 0.95, 0.99].find((v) => String(v) === event.target.value)
                if (selected !== undefined) onChange({ ...configuration, confidence: selected })
              }}
            >
              {[0.9, 0.95, 0.99].map((v) => (
                <option key={v} value={v}>
                  {Math.round(v * 100)}%
                </option>
              ))}
            </Select>
          </label>
        </div>
      </SettingsStep>
    </>
  )
}
