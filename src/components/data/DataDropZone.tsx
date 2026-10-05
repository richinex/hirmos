import { useRef, useState, type DragEvent } from 'react'
import { Icon } from '@/components/Icon'
import { button } from '@/components/ui/recipes'
import { isNonEmpty, type NonEmptyArray } from '@/domain/dop'
import { cn } from '@/lib/utils'

const ACCEPT =
  '.csv,.tsv,.parquet,text/csv,text/tab-separated-values,application/vnd.apache.parquet'

interface DataDropZoneProps {
  /** The first line in the zone: what to drop here. */
  readonly invitation: string
  /** The second line: what happens to it. */
  readonly consequence?: string
  readonly action: string
  readonly multiple?: boolean
  readonly busy?: boolean
  readonly onFiles: (files: NonEmptyArray<File>) => void
  /** Fires on hover and focus of the action, before the picker opens; the SQL path warms its chunks here. */
  readonly onIntent?: () => void
}

/** Accept files selected with the picker or dropped on the same control. */
export function DataDropZone({
  invitation,
  consequence,
  action,
  multiple = false,
  busy = false,
  onFiles,
  onIntent,
}: DataDropZoneProps) {
  const input = useRef<HTMLInputElement>(null)
  const [over, setOver] = useState(false)

  const take = (list: FileList | null) => {
    const files = list === null ? [] : Array.from(list)
    if (!isNonEmpty(files)) return
    onFiles(multiple ? files : [files[0]])
  }
  const drop = (event: DragEvent<HTMLDivElement>) => {
    event.preventDefault()
    setOver(false)
    if (busy) return
    take(event.dataTransfer.files)
  }

  return (
    <div
      className={cn(
        'flex min-h-[18rem] flex-col items-center justify-center gap-3 rounded-lg border border-dashed bg-well px-6 py-8 text-center transition-colors duration-(--motion-fast)',
        over ? 'border-signal' : 'border-line',
      )}
      onDragOver={(event) => {
        event.preventDefault()
        if (!over) setOver(true)
      }}
      onDragLeave={() => setOver(false)}
      onDrop={drop}
    >
      <Icon name="upload_file" size={28} className="text-faint" aria-hidden />
      <p className="m-0 text-body text-ink">{invitation}</p>
      {consequence !== undefined && (
        <p className="m-0 max-w-[40ch] text-body text-faint text-pretty">{consequence}</p>
      )}
      <input
        ref={input}
        type="file"
        multiple={multiple}
        accept={ACCEPT}
        className="sr-only"
        onChange={(event) => {
          take(event.target.files)
          event.target.value = ''
        }}
      />
      <button
        type="button"
        className={button('signal', 'mt-1 inline-flex items-center gap-2')}
        disabled={busy}
        aria-busy={busy}
        onMouseEnter={onIntent}
        onFocus={onIntent}
        onClick={() => input.current?.click()}
      >
        {busy ? 'Reading the files' : action}
      </button>
    </div>
  )
}
