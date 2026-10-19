import type { ChapterId } from '@/domain/navigation'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { chapterIntro, chapterIntroSingle, field, panel } from '@/components/ui/recipes'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'
import { chapterFrame, type ChapterHead, type StageBody } from './chapterFrames'
import { WorkbenchLayout } from './WorkbenchLayout'

/**
 * Stands in for a chapter while its code chunk loads. A workbench chapter is sketched inside the same
 * WorkbenchLayout it mounts into, with the same pane ids, so the panes, seams and stored sizes are the
 * real ones and nothing moves when the chapter arrives. Its heading and intro are the chapter's own
 * text drawn as bars, so they wrap to the same lines at every width.
 */
export function ChapterSkeleton({
  label,
  chapter,
}: {
  readonly label: string
  readonly chapter?: ChapterId
}) {
  return (
    <div role="status" aria-label={label} className="hold-appear flex min-w-0 flex-1">
      <div inert className="chapter-sketch flex min-w-0 flex-1">
        <ChapterSketch chapter={chapter} />
      </div>
    </div>
  )
}

function ChapterSketch({ chapter }: { readonly chapter?: ChapterId }) {
  const mobile = useIsMobile()
  const frame = chapter === undefined ? ({ kind: 'page' } as const) : chapterFrame(chapter)

  if (frame.kind === 'workbench')
    return (
      <WorkbenchLayout
        id={frame.layoutId}
        stage={<StageSketch head={frame.head} body={frame.body} />}
        inspector={{ ...frame.inspector, body: <InspectorSketch /> }}
        bottom={{ ...frame.bottom, body: <BottomSketch /> }}
      />
    )
  if (frame.kind === 'desk' && !mobile)
    return <WorkbenchLayout id="results" stagePadding={false} stageScroll={false} stage={null} />
  return <PageSketch />
}

function PageSketch() {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-4 px-4 py-5">
      <div className="skeleton h-7 w-64 max-w-full rounded-md" />
      <div className="skeleton h-28 w-full max-w-[65ch] rounded-lg" />
      <div className="skeleton min-h-72 w-full flex-1 rounded-lg" />
    </div>
  )
}

function HeadSketch({
  head,
  className,
}: {
  readonly head: ChapterHead
  readonly className?: string
}) {
  const title = <span className="skeleton-text">{head.title}</span>
  return (
    <div className={className}>
      {head.help ? (
        <div className="mb-2 flex items-center gap-1.5">
          <ChapterHeading className="mb-0">{title}</ChapterHeading>
          <span className="grid size-5 shrink-0 place-items-center pointer-coarse:size-11">
            <span className="skeleton size-3.5 rounded-full" />
          </span>
        </div>
      ) : (
        <ChapterHeading className="mb-2">{title}</ChapterHeading>
      )}
      <p className={head.introLayout === 'columns' ? chapterIntro : chapterIntroSingle}>
        <span className="skeleton-text">{head.intro}</span>
      </p>
    </div>
  )
}

function FieldSketch({ width = 'w-full' }: { readonly width?: string }) {
  return (
    <div className="grid gap-1.5">
      <div className="skeleton h-3 w-28 rounded" />
      <div className={field('text', cn('h-9 max-w-full', width))} />
    </div>
  )
}

function StageSketch({ head, body }: { readonly head: ChapterHead; readonly body: StageBody }) {
  if (body === 'canvas')
    return (
      <section className="@container/panel flex min-h-full flex-1 flex-col">
        <HeadSketch head={head} className="mb-3" />
        <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
          <div className="flex gap-3">
            <div className={field('text', 'h-9 w-40')} />
            <div className={field('text', 'h-9 w-40')} />
          </div>
          <div className="flex gap-2">
            <div className="h-9 w-28 rounded-md border border-line" />
            <div className="h-9 w-28 rounded-md border border-line" />
          </div>
        </div>
        <div className="flex min-h-[22rem] flex-1 rounded-xl border border-edge bg-well @max-md/panel:min-h-[26rem]" />
      </section>
    )
  return (
    <section className="@container/panel flex min-h-full flex-1 flex-col gap-5">
      <HeadSketch head={head} />
      {body === 'card' ? (
        <div className={panel('flex flex-1 flex-col gap-5 p-(--panel-space)')}>
          <div className="skeleton h-7 w-80 max-w-full rounded-md" />
          <FieldSketch />
          <div className="grid gap-4 @3xl/panel:grid-cols-2">
            <FieldSketch />
            <FieldSketch />
          </div>
        </div>
      ) : (
        <div className="flex flex-col gap-6">
          {[0, 1, 2].map((step) => (
            <div key={step} className="grid gap-3">
              <div className="skeleton h-5 w-56 max-w-full rounded" />
              <FieldSketch width="w-[28rem]" />
            </div>
          ))}
        </div>
      )}
    </section>
  )
}

function InspectorSketch() {
  return (
    <div className="flex flex-col gap-3">
      <div className="skeleton h-4 w-2/3 rounded" />
      <div className="skeleton h-8 w-full rounded-md" />
      <div className="mt-2 grid gap-2">
        <div className="skeleton h-3 w-full rounded" />
        <div className="skeleton h-3 w-11/12 rounded" />
        <div className="skeleton h-3 w-4/5 rounded" />
      </div>
    </div>
  )
}

function BottomSketch() {
  return (
    <div className="grid gap-2 px-3 py-2">
      <div className="skeleton h-4 w-full max-w-xl rounded" />
      <div className="skeleton h-4 w-full max-w-md rounded" />
    </div>
  )
}
