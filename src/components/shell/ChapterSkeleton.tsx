/**
 * Stands in for a chapter panel while its code chunk loads. The blocks sketch the chapter's shape —
 * heading, controls, work surface — so the region keeps its height instead of collapsing to a line
 * and jumping when the panel arrives.
 */
export function ChapterSkeleton({ label }: { readonly label: string }) {
  return (
    <div role="status" aria-label={label} className="flex min-h-[70vh] flex-col gap-4">
      <div className="skeleton h-7 w-64 max-w-full rounded-md" />
      <div className="skeleton h-28 w-full max-w-[65ch] rounded-lg" />
      <div className="skeleton h-72 w-full rounded-lg" />
    </div>
  )
}
