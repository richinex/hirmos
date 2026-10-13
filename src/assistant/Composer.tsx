import { useCallback, useEffect, useRef, useState, type KeyboardEvent } from 'react'
import { Icon } from '@/components/Icon'
import { cn } from '@/lib/utils'
import { iconControl, label } from '@/components/ui/recipes'
import type { AssistantSeed } from './assistantStore'

/**
 * Terminal-style composer: a signal-coloured prompt caret and a square send key. Up from the start of
 * the box walks back through past questions, and down returns to the stashed draft.
 */
export function Composer({
  onSend,
  onStop,
  isStreaming,
  placeholder,
  history = [],
  unavailable = false,
  seed = null,
}: {
  onSend: (question: string) => void
  onStop: () => void
  isStreaming: boolean
  placeholder?: string
  history?: string[]
  unavailable?: boolean
  seed?: AssistantSeed | null
}) {
  const [value, setValue] = useState('')
  const ref = useRef<HTMLTextAreaElement>(null)
  useEffect(() => {
    if (seed === null) return
    setValue(seed.text)
    const el = ref.current
    if (!el) return
    el.focus()
    el.setSelectionRange(seed.text.length, seed.text.length)
  }, [seed])
  const [histIdx, setHistIdx] = useState<number | null>(null)
  const draftRef = useRef('')

  // The placeholder text. Named once because the autosize below has to measure it too.
  const shown = unavailable
    ? 'Connect OpenRouter and approve the summary first'
    : (placeholder ?? 'Ask about your analysis…')

  const resize = useCallback(() => {
    const el = ref.current
    if (!el) return
    el.style.height = '0px'
    const capped = Math.min(el.scrollHeight, 160)
    el.style.height = `${capped}px`
    el.style.overflowY = el.scrollHeight > 160 ? 'auto' : 'hidden'
  }, [])

  // `shown` is a dependency because the placeholder is measured too. The empty-graph text wraps to two
  // lines on a phone, and without this the box kept that height after compiling swapped in a shorter one.
  useEffect(() => {
    resize()
  }, [value, shown, resize])

  const submit = useCallback(() => {
    const trimmed = value.trim()
    if (!trimmed || isStreaming || unavailable) return
    onSend(trimmed)
    setValue('')
    setHistIdx(null)
  }, [value, isStreaming, unavailable, onSend])

  const showHistory = useCallback(
    (idx: number) => {
      setHistIdx(idx)
      setValue(history[idx])
    },
    [history],
  )

  const onKeyDown = useCallback(
    (e: KeyboardEvent<HTMLTextAreaElement>) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        // Enter confirming an IME conversion is not a send.
        if (e.nativeEvent.isComposing || e.keyCode === 229) return
        // On touch keyboards Enter inserts a newline and the 44px send button submits, because a
        // return key that fires the query mid-thought is a misfire on mobile.
        if (window.matchMedia?.('(pointer: coarse)').matches) return
        e.preventDefault()
        submit()
        return
      }
      const el = ref.current
      if (e.key === 'ArrowUp') {
        const atStart = !el || (el.selectionStart === 0 && el.selectionEnd === 0)
        if (histIdx === null) {
          if (!atStart || history.length === 0) return
          e.preventDefault()
          draftRef.current = value
          showHistory(history.length - 1)
        } else if (histIdx > 0) {
          e.preventDefault()
          showHistory(histIdx - 1)
        } else {
          e.preventDefault()
        }
      } else if (e.key === 'ArrowDown' && histIdx !== null) {
        e.preventDefault()
        if (histIdx < history.length - 1) showHistory(histIdx + 1)
        else {
          setHistIdx(null)
          setValue(draftRef.current)
        }
      }
    },
    [submit, histIdx, history, value, showHistory],
  )

  const canSend = value.trim().length > 0 && !isStreaming && !unavailable

  return (
    <div className="shrink-0 bg-panel px-3.5 pb-3 pt-2">
      <div className="lift flex items-end gap-2.5 rounded-xl border border-hair bg-well px-3 py-2 transition-colors duration-150 focus-within:border-signal/60">
        {/* Bottom-aligned on desktop, where one line is the whole field. On a phone the field pads out to
            the 44px tap target, so the marker centres with it instead of sitting ten pixels low. */}
        <span className="pt-0.5 text-body text-signal max-md:self-center max-md:pt-0">›</span>
        <textarea
          ref={ref}
          value={value}
          onChange={(e) => {
            setValue(e.target.value)
            setHistIdx(null)
          }}
          onKeyDown={onKeyDown}
          rows={1}
          maxLength={8000}
          placeholder={shown}
          aria-label="Ask a question"
          // The phone send button is 44px against a 24px line, and `items-end` leaves that difference as
          // dead space above the text. Padding the field to 44px centres the line against the button.
          // max-md:text-[16px] is a sanctioned exception to the type ramp: below 16px iOS Safari
          // auto-zooms the page on focus.
          className="max-h-[160px] flex-1 resize-none bg-transparent text-body text-ink placeholder:text-faint focus:outline-none max-md:py-2.5 max-md:text-[16px]"
        />
        {isStreaming ? (
          <button
            type="button"
            onClick={onStop}
            aria-label="Stop"
            className={iconControl('signal', 'max-md:h-11 max-md:w-11 max-md:rounded-xl')}
          >
            <Icon name="stop" size={17} />
          </button>
        ) : (
          <button
            type="button"
            onClick={submit}
            disabled={!canSend}
            aria-label="Send"
            className={cn(
              iconControl('quiet', 'max-md:h-11 max-md:w-11 max-md:rounded-xl'),
              canSend
                ? 'border-signal bg-signal text-signal-ink hover:bg-signal hover:brightness-110'
                : 'cursor-not-allowed border-transparent bg-raised text-faint',
            )}
          >
            <Icon name="send" size={17} />
          </button>
        )}
      </div>
      {/* Hidden on phones: these name hardware keys, and on touch keyboards Enter is newline anyway.
          font-sans is explicit because the preflight sets <kbd> to mono, and a keycap is a label. */}
      <div className={label('mt-2 hidden items-center gap-4 text-faint md:flex')}>
        <span className="flex items-center gap-1">
          <kbd className="rounded border border-hair bg-well px-1 py-px font-sans text-small text-muted">
            ↵
          </kbd>{' '}
          Send
        </span>
        <span className="flex items-center gap-1">
          <kbd className="rounded border border-hair bg-well px-1 py-px font-sans text-small text-muted">
            ⇧↵
          </kbd>{' '}
          Newline
        </span>
        {history.length > 0 && (
          <span className="flex items-center gap-1">
            <kbd className="rounded border border-hair bg-well px-1 py-px font-sans text-small text-muted">
              ↑
            </kbd>{' '}
            Recall
          </span>
        )}
      </div>
    </div>
  )
}
