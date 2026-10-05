/**
 * Stands in for a chapter panel while its code chunk loads. It claims the stage the way a workbench
 * does — full width, stage padding, surface filling the height — and its blocks sketch the chapter's
 * shape: heading, controls, work surface. The region keeps its frame instead of collapsing and
 * jumping when the panel arrives.
 */
export function ChapterSkeleton({ label }: { readonly label: string }) {
  return (
    <div
      role="status"
      aria-label={label}
      className="hold-appear flex min-w-0 flex-1 flex-col gap-4 px-4 py-5"
    >
      <div className="skeleton h-7 w-64 max-w-full rounded-md" />
      <div className="skeleton h-28 w-full max-w-[65ch] rounded-lg" />
      <div className="skeleton min-h-72 w-full flex-1 rounded-lg" />
    </div>
  )
}
