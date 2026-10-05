import { Icon } from '@/components/Icon'
import { chromeAction, iconControl } from '@/components/ui/recipes'
import { useIsMobile } from '@/lib/useMediaQuery'

/**
 * The control that opens a saved project or an example. It is the word-sized sibling of the export
 * and delete icon controls beside it, so the row reads as one control family; on a phone, where the
 * row has no room for a word, it becomes a glyph of the same family and the name moves to its label.
 */
export function OpenControl({
  name,
  onOpen,
}: {
  readonly name: string
  readonly onOpen: () => void
}) {
  const isMobile = useIsMobile()
  return isMobile ? (
    <button
      type="button"
      className={iconControl('quiet')}
      aria-label={`Open ${name}`}
      onClick={onOpen}
    >
      <Icon name="open_in_new" size={14} />
    </button>
  ) : (
    <button
      type="button"
      className={chromeAction('quiet')}
      aria-label={`Open ${name}`}
      onClick={onOpen}
    >
      Open
    </button>
  )
}
