import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { field, fieldLabel } from '@/components/ui/recipes'
import type { TLearnerUncertainty as Settings } from '@/domain/tLearner'

export function TLearnerUncertainty({
  value,
  onChange,
}: {
  readonly value: Settings
  readonly onChange: (value: Settings) => void
}) {
  return (
    <>
      <div>
        <span className={fieldLabel}>Uncertainty</span>
        <SegmentedControl
          className="mt-1"
          fill
          ariaLabel="T-learner uncertainty"
          value={value.kind}
          options={[
            { value: 'none', label: 'Point estimates' },
            { value: 'bootstrap', label: 'Bootstrap intervals' },
          ]}
          onChange={(kind) =>
            onChange(
              kind === 'none'
                ? { kind }
                : { kind, samples: 100, seed: 47, level: 0.95, method: 'percentile' },
            )
          }
        />
      </div>
      {value.kind === 'bootstrap' && (
        <>
          <label className="block">
            <span className={fieldLabel}>Bootstrap samples</span>
            <input
              className={field('text', 'mt-1')}
              aria-label="Bootstrap samples"
              type="number"
              min={2}
              max={1000}
              value={value.samples}
              onChange={(event) =>
                onChange({
                  ...value,
                  samples: Math.min(1000, Math.max(2, Math.floor(Number(event.target.value) || 2))),
                })
              }
            />
          </label>
          <label className="block">
            <span className={fieldLabel}>Bootstrap seed</span>
            <input
              className={field('text', 'mt-1')}
              aria-label="Bootstrap seed"
              type="number"
              min={0}
              max={4294967295}
              value={value.seed}
              onChange={(event) =>
                onChange({
                  ...value,
                  seed: Math.min(
                    4294967295,
                    Math.max(0, Math.floor(Number(event.target.value) || 0)),
                  ),
                })
              }
            />
          </label>
          <div>
            <span className={fieldLabel}>Confidence level</span>
            <SegmentedControl
              className="mt-1"
              fill
              ariaLabel="Bootstrap confidence level"
              value={String(value.level)}
              options={[
                { value: '0.9', label: '90%' },
                { value: '0.95', label: '95%' },
                { value: '0.99', label: '99%' },
              ]}
              onChange={(level) => onChange({ ...value, level: Number(level) })}
            />
          </div>
          <div>
            <span className={fieldLabel}>Row interval method</span>
            <SegmentedControl
              className="mt-1"
              fill
              ariaLabel="Row interval method"
              value={value.method}
              options={[
                { value: 'percentile', label: 'Percentile' },
                { value: 'pivot', label: 'Basic' },
                { value: 'normal', label: 'Normal' },
              ]}
              onChange={(method) => onChange({ ...value, method })}
            />
          </div>
        </>
      )}
    </>
  )
}
