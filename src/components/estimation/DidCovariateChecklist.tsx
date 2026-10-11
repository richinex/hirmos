import type { ColumnId } from '@/domain/dataset'

/** Where the candidate check stands: still reading, checked, or not possible on this panel. */
export type DidCovariateCheck = 'pending' | 'checked' | 'unchecked'

/**
 * The covariate checkboxes of a DiD estimator. A restricted column cannot be newly selected and says why;
 * one already selected stays selectable so it can be removed, and the run refuses it before fitting.
 */
export function DidCovariateChecklist({
  label,
  candidates,
  selected,
  restrictions,
  check,
  onChange,
}: {
  readonly label: string
  readonly candidates: readonly { readonly id: ColumnId; readonly name: string }[]
  readonly selected: readonly ColumnId[]
  readonly restrictions: ReadonlyMap<ColumnId, string>
  readonly check: DidCovariateCheck
  readonly onChange: (selected: readonly ColumnId[]) => void
}) {
  const chosen = new Set(selected)
  return (
    <>
      <div role="group" aria-label={label} className="mt-1 flex flex-wrap gap-2">
        {candidates.map((c) => {
          const reason = restrictions.get(c.id)
          const disabled = reason !== undefined && !chosen.has(c.id)
          return (
            <label
              key={c.id}
              title={reason}
              className={`flex items-center gap-1.5 text-body ${disabled ? 'text-faint' : 'text-ink'}`}
            >
              <input
                type="checkbox"
                aria-label={c.name}
                disabled={disabled}
                checked={chosen.has(c.id)}
                onChange={(e) =>
                  onChange(
                    e.target.checked ? [...selected, c.id] : selected.filter((id) => id !== c.id),
                  )
                }
              />
              {c.name}
            </label>
          )
        })}
      </div>
      {check === 'pending' ? (
        <p className="mb-0 mt-2 text-body text-muted" role="status">
          Checking whether any covariates duplicate treatment or group membership…
        </p>
      ) : check === 'unchecked' ? (
        <p className="mb-0 mt-2 text-body text-muted">
          The candidate list could not be checked. Selected covariates are checked before fitting.
        </p>
      ) : null}
    </>
  )
}
