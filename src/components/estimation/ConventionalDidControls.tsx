import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { NumberInput } from '@/components/ui/NumberInput'
import { field, fieldHint, fieldLabel, fieldRow } from '@/components/ui/recipes'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { cn } from '@/lib/utils'
import {
  conventionalDidInferenceHelp,
  defaultConventionalDidInference,
  type ConventionalDidInference,
} from '@/domain/conventionalDid'

export function ConventionalDidControls({
  inference = defaultConventionalDidInference,
  placeboUnavailable = null,
  onChange,
}: {
  readonly inference?: ConventionalDidInference
  readonly placeboUnavailable?: string | null
  readonly onChange: (value: ConventionalDidInference) => void
}) {
  return (
    <div className="grid gap-3">
      <SegmentedControl
        className="justify-self-start"
        ariaLabel="Conventional DiD uncertainty"
        value={inference.kind}
        options={[
          { value: 'bootstrap', label: 'Unit bootstrap' },
          { value: 'jackknife', label: 'Jackknife' },
          {
            value: 'placebo',
            label: 'Placebo',
            disabled: placeboUnavailable !== null,
            title: placeboUnavailable ?? undefined,
          },
        ]}
        onChange={(kind) =>
          onChange(
            kind === 'jackknife'
              ? { kind }
              : {
                  kind,
                  replications: inference.kind === 'jackknife' ? 200 : inference.replications,
                  seed: inference.kind === 'jackknife' ? 0 : inference.seed,
                },
          )
        }
      />
      {placeboUnavailable !== null && (
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>{placeboUnavailable}</p>
      )}
      <ParameterLabel
        className={fieldLabel}
        label="Uncertainty assumptions"
        help={conventionalDidInferenceHelp[inference.kind]}
      />
      {inference.kind !== 'jackknife' && (
        <div className={fieldRow.two}>
          <label className="block">
            <span className={fieldLabel}>Replications</span>
            <NumberInput
              aria-label="DiD replications"
              className={field('text', 'mt-1 w-full')}
              min={2}
              max={2000}
              value={inference.replications}
              onChange={(event) =>
                onChange({
                  ...inference,
                  replications: Math.max(
                    2,
                    Math.min(2000, Math.floor(Number(event.target.value) || 2)),
                  ),
                })
              }
            />
          </label>
          <label className="block">
            <span className={fieldLabel}>Seed</span>
            <NumberInput
              aria-label="DiD seed"
              className={field('text', 'mt-1 w-full')}
              min={0}
              max={0xffff_ffff}
              value={inference.seed}
              onChange={(event) =>
                onChange({
                  ...inference,
                  seed: Math.max(
                    0,
                    Math.min(0xffff_ffff, Math.floor(Number(event.target.value) || 0)),
                  ),
                })
              }
            />
          </label>
        </div>
      )}
    </div>
  )
}
