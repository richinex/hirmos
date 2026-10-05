import { CHAPTER_METADATA, type ChapterId } from '@/domain/navigation'
import { chromeAction } from '@/components/ui/recipes'
import { useChapterRun, useRunningChapter } from '@/lib/runActivityStore'

/** The header's chip for the run in flight. It subscribes itself, so progress redraws only this chip. */
export function RunActivityChip({ onOpen }: { readonly onOpen: (chapter: ChapterId) => void }) {
  const chapter = useRunningChapter()
  const run = useChapterRun(chapter)
  if (chapter === null || run === null) return null
  return (
    <button
      type="button"
      className={chromeAction('quiet', 'relative min-w-0 gap-2 pr-3 text-muted')}
      aria-label={`${run.label} running in ${CHAPTER_METADATA[chapter].name}; open it`}
      onClick={() => onOpen(chapter)}
    >
      <span className="truncate">{run.label}</span>
      <span
        aria-hidden
        className="bar-live absolute inset-x-2 bottom-[3px] h-[2px] rounded-full bg-line"
      >
        <span
          className="bar-live__fill block rounded-full bg-signal"
          style={{ width: `${Math.round((run.progress ?? 0) * 100)}%` }}
        />
      </span>
    </button>
  )
}
