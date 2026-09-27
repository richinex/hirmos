import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { field, fieldLabel, fieldRow } from '@/components/ui/recipes'
import type { PropensityUncertainty } from '@/domain/estimation'

const ROUNDS = 200
const SEED = 123
const LEVEL = 0.95

export function BootstrapIntervalFields({ value, help, onChange }: {
  readonly value: PropensityUncertainty
  readonly help: string
  readonly onChange: (value: PropensityUncertainty) => void
}) {
  const drawn = value.kind === 'bootstrap' ? value : { kind: 'bootstrap' as const, rounds: ROUNDS, seed: SEED, level: LEVEL }
  return <>
    <div>
      <ParameterLabel className={fieldLabel} label="Uncertainty" help={help} />
      <SegmentedControl className="mt-1 max-w-md" fill ariaLabel="Interval" value={value.kind}
        options={[{ value: 'none', label: 'Point estimate' }, { value: 'bootstrap', label: 'Bootstrap interval' }]}
        onChange={(kind) => onChange(kind === 'none' ? { kind: 'none' } : drawn)} />
    </div>
    {value.kind === 'bootstrap' && (
      <div className={fieldRow.three}>
        <label className="block"><span className={fieldLabel}>Bootstrap rounds</span><input className={field('text', 'mt-1 w-full')} aria-label="Bootstrap rounds" type="number" min={2} max={2000} value={value.rounds} onChange={(event) => onChange({ ...value, rounds: Math.min(2000, Math.max(2, Math.floor(Number(event.target.value) || 2))) })} /></label>
        <label className="block"><span className={fieldLabel}>Bootstrap seed</span><input className={field('text', 'mt-1 w-full')} aria-label="Bootstrap seed" type="number" min={0} max={4294967295} value={value.seed} onChange={(event) => onChange({ ...value, seed: Math.min(4294967295, Math.max(0, Math.floor(Number(event.target.value) || 0))) })} /></label>
        <div><span className={fieldLabel}>Confidence level</span><SegmentedControl className="mt-1" fill ariaLabel="Confidence level" value={String(value.level)} options={[{ value: '0.9', label: '90%' }, { value: '0.95', label: '95%' }, { value: '0.99', label: '99%' }]} onChange={(level) => onChange({ ...value, level: Number(level) })} /></div>
      </div>
    )}
  </>
}
