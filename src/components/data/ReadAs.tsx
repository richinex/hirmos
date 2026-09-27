import { Select } from '@/components/ui/Select'
import { literal } from '@/components/ui/recipes'
import { DECLARED_TYPE_NAMES, declaredTypeLabel, declaredTypeSchema, type ColumnReading, type DeclaredType } from '@/domain/fileReading'
import { cn } from '@/lib/utils'
import { kindGlyph, kindOf, kindText } from './columnKind'

const DETECTED = 'detected'

/**
 * The type glyph beside a column of a delimited file, as the button that sets how the column is
 * read: as DuckDB detects it, or as a declared type. The schema table and the pipeline's input card
 * both use it, so a declaration looks and reads the same in each.
 */
export function ReadAsButton({ column, reading, type, disabled, onDeclare }: {
  readonly column: string
  readonly reading: ColumnReading
  /** The DuckDB type the column is read as now. */
  readonly type: string
  readonly disabled?: boolean
  readonly onDeclare: (type: DeclaredType | null) => void
}) {
  const kind = kindOf(type)
  return (
    <Select
      aria-label={`Read ${column} as`}
      heading={`Read ${column} as`}
      className={cn(
        'inline-flex h-5 w-6 shrink-0 items-center justify-center rounded-sm border text-micro text-faint hover:border-control hover:bg-well hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-current data-[state=open]:border-control data-[state=open]:bg-well data-[state=open]:text-ink',
        reading.kind === 'declared' ? 'border-signal/45 text-signal-text' : 'border-transparent',
      )}
      trigger={<span aria-hidden className={literal()} title={`${kindText(kind)}. Choose how ${column} is read.`}>{kindGlyph(kind)}</span>}
      value={reading.kind === 'declared' ? reading.type : DETECTED}
      disabled={disabled}
      onChange={(event) => {
        if (event.target.value === DETECTED) { onDeclare(null); return }
        const declared = declaredTypeSchema.safeParse(event.target.value)
        if (declared.success) onDeclare(declared.data)
      }}
    >
      <option value={DETECTED}>{reading.kind === 'detected' ? `Detected, ${type}` : 'Detected'}</option>
      {DECLARED_TYPE_NAMES.map((name) => <option key={name} value={name}>{declaredTypeLabel(name)}</option>)}
    </Select>
  )
}

/** The glyph for a column whose reading cannot change: a Parquet file's, or where nothing is declared. */
export function TypeGlyph({ type }: { readonly type: string }) {
  const kind = kindOf(type)
  return <span aria-hidden className={literal('inline-flex h-5 w-6 shrink-0 items-center justify-center text-micro text-faint')} title={kindText(kind)}>{kindGlyph(kind)}</span>
}

/** The DuckDB type a column is read as, marked when the reader declared it. */
export function ColumnTypeText({ reading, type }: { readonly reading: ColumnReading; readonly type: string }) {
  return (
    <span className="whitespace-nowrap">
      <span className={literal('text-faint')}>{type}</span>
      {reading.kind === 'declared' && <span className="ml-1.5 text-micro font-medium text-signal-text">declared</span>}
    </span>
  )
}
