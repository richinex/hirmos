export function activeTimer(milliseconds: number, expired: () => void) {
  let remaining = milliseconds
  let started = performance.now()
  let pauses = 0
  let stopped = false
  let timer = setTimeout(expired, remaining)
  return {
    pause() {
      if (stopped || pauses++ > 0) return
      clearTimeout(timer)
      remaining = Math.max(0, remaining - (performance.now() - started))
    },
    resume() {
      if (stopped || pauses === 0 || --pauses > 0) return
      started = performance.now()
      timer = setTimeout(expired, remaining)
    },
    stop() {
      stopped = true
      clearTimeout(timer)
    },
  }
}
