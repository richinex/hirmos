/** The pointer a replay moves and presses with, hidden until the replay animates it. */
export function ReplayCursor() {
  return (
    <svg
      className="replay-cursor"
      viewBox="0 0 24 24"
      width="26"
      height="26"
      style={{ opacity: 0 }}
      aria-hidden
    >
      <path d="M4 2L4 20L9 15L12.5 22L15.5 20.5L12 13.5L19 13.5Z" />
    </svg>
  )
}
