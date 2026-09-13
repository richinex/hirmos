/** Directed H, shared by the home link and workbench navigation. */
export function HirmosMark({ className, size = 20 }: { readonly className?: string; readonly size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 32 32" fill="none" aria-hidden className={className}>
      <rect width="32" height="32" rx="7" fill="#252e28" />
      <g fill="#e5f0ae">
        <path d="M7 7h5v7h6v-4l8 6-8 6v-4h-6v7H7z" />
        <path d="M21 7h4v5l-4-3zm0 16 4-3v5h-4z" />
      </g>
    </svg>
  )
}
