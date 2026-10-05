import type { DagDocument } from '@/domain/dag'
import type { SwigProjection } from '@/domain/swigProjection'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { field, fieldLabel, fieldHint } from '@/components/ui/recipes'

export function SwigProjectionForm({
  document,
  value,
  confirmed,
  onConfirm,
  onChange,
  disabled,
  boundaryArrows,
}: {
  readonly document: DagDocument
  readonly value: SwigProjection
  readonly confirmed: boolean
  readonly onConfirm: (value: boolean) => void
  readonly onChange: (value: SwigProjection) => void
  readonly disabled: boolean
  readonly boundaryArrows: number | null
}) {
  return (
    <fieldset disabled={disabled} className="min-w-0 space-y-3 border-0 p-0">
      <label className="block">
        <span className={fieldLabel}>Source graph representation</span>
        <Select
          aria-label="Source graph representation"
          className={field('text', 'mt-1')}
          value={value.kind}
          onChange={(e) =>
            onChange(
              e.target.value === 'explicit'
                ? { kind: 'explicit' }
                : { kind: 'temporal', start: 0, end: 1, invariant: [], boundary: 'closed-history' },
            )
          }
        >
          <option value="explicit">Explicit period nodes</option>
          <option value="temporal">Expand a time-series template</option>
        </Select>
      </label>
      {value.kind === 'temporal' && (
        <>
          <div className="grid grid-cols-2 gap-2">
            {(['start', 'end'] as const).map((key) => (
              <label key={key}>
                <span className={fieldLabel}>
                  {key === 'start' ? 'First period' : 'Last period'}
                </span>
                <input
                  type="number"
                  min={-32}
                  max={32}
                  aria-label={key === 'start' ? 'First expansion period' : 'Last expansion period'}
                  className={field('text', 'mt-1')}
                  value={value[key]}
                  onChange={(e) => onChange({ ...value, [key]: e.target.valueAsNumber })}
                />
              </label>
            ))}
          </div>
          <div className="panel-scroll max-h-48 overflow-y-auto">
            <ColumnChecklist
              title="Time-invariant variables"
              help="These variables have one shared node, rather than a separate node in each period."
              columns={document.current.graph.nodes.map((n) => ({ id: n.id, name: n.name }))}
              selected={value.invariant}
              onChange={(invariant) => onChange({ ...value, invariant: [...invariant] })}
            />
          </div>
          <p className={fieldHint}>
            Only the selected periods are represented.{' '}
            {boundaryArrows === null
              ? ''
              : `${boundaryArrows} arrows cross the first-period boundary and are omitted.`}{' '}
            Extend the history or represent its common causes if those omissions would change the
            causal assumptions.
          </p>
          <label className="flex items-start gap-2 text-body">
            <input
              type="checkbox"
              checked={confirmed}
              onChange={(e) => onConfirm(e.target.checked)}
            />
            <span>
              I assume the displayed initial-history graph is causally sufficient. I have recorded
              the reason in the rationale.
            </span>
          </label>
        </>
      )}
    </fieldset>
  )
}
