import { useRef, useState } from 'react'
import { isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { button } from '@/components/ui/recipes'

type Selection = { readonly kind: 'empty' } | { readonly kind: 'chosen'; readonly files: NonEmptyArray<File> }

export function DatabaseFolder({ onFiles, busy = false }: {
  readonly onFiles: (files: NonEmptyArray<File>) => void
  readonly busy?: boolean
}) {
  const picker = useRef<HTMLInputElement | null>(null)
  const [selection, setSelection] = useState<Selection>({ kind: 'empty' })
  return <>
    <input type="file" multiple className="sr-only" aria-label="DuckDB export folder" ref={node => { picker.current = node; node?.setAttribute('webkitdirectory', '') }} onChange={event => {
      const files = Array.from(event.target.files ?? [])
      if (isNonEmpty(files)) setSelection({ kind: 'chosen', files })
      event.target.value = ''
    }} />
    <button type="button" className={button('outline')} disabled={busy} onClick={() => picker.current?.click()}>Choose export folder</button>
    {selection.kind === 'chosen' && <ConfirmDialog open title="Import database" confirmLabel="Import database" message={`${selection.files.length} files selected. DuckDB will run the export's SQL locally to recreate its tables and views. Only import exports you trust. Your original files will not be changed.`} onClose={() => setSelection({ kind: 'empty' })} onConfirm={() => { onFiles(selection.files); setSelection({ kind: 'empty' }) }} />}
  </>
}
