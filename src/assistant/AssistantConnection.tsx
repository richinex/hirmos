import { useEffect, useRef, useState } from 'react'
import * as Popover from '@radix-ui/react-popover'
import { Icon } from '@/components/Icon'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { button, chromeAction, field, fieldLabel, fieldRow, label } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { assistantLimits } from './limits'
import type { CreditStatus } from './budget'
import { CreditGauge } from './BudgetMeter'

/** The model ID without its provider prefix; the full ID stays in the Model ID field. */
const modelName = (model: string): string => model.trim().split('/').at(-1) || model.trim()

/** The privacy terms behind one control, so the settings stay short and nothing is sent before the user agrees. */
function PrivacyNotice({
  context,
  agreed,
  onAgree,
}: {
  readonly context: string
  readonly agreed: boolean
  readonly onAgree: () => void
}) {
  const [open, setOpen] = useState(false)
  return (
    <Popover.Root open={open} onOpenChange={setOpen}>
      <Popover.Trigger asChild>
        <button type="button" className={chromeAction(agreed ? 'quiet' : 'signal')}>
          <Icon
            name="privacy_tip"
            size={15}
            fill={agreed}
            className={agreed ? 'text-ok' : undefined}
          />
          Privacy
        </button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content
          aria-label="Privacy"
          side="top"
          align="start"
          sideOffset={8}
          collisionPadding={12}
          className="pop float z-(--z-dialog) flex max-h-[var(--radix-popover-content-available-height)] w-[22rem] max-w-[calc(100vw-24px)] flex-col gap-3 overflow-y-auto rounded-xl border border-line bg-panel p-4 text-body text-ink"
        >
          <p className="m-0 text-label text-muted">
            Using these settings allows your questions, this project's current and updated
            summaries, generated code and outputs to be sent to OpenRouter and its model provider,
            including subagent requests. Data rows are not included. Data profiles can include
            category values, ranges and small counts; they are not anonymised. Conversation history
            is saved in this browser for this project. The assistant can retrieve earlier messages,
            code, outputs and answers; retrieved records are sent to the provider. Semantic search
            runs locally and downloads its model when first needed. Clear conversation deletes this
            project's stored memory. Sharing a new analysis summary still requires separate
            approval. The key and model have not been verified.
          </p>
          <details>
            <DisclosureSummary icon="visibility" className="cursor-pointer text-label">
              Project summary
            </DisclosureSummary>
            <pre className="mt-2 max-h-52 overflow-auto whitespace-pre-wrap break-words text-label">
              {context}
            </pre>
          </details>
          {agreed ? null : (
            <button
              type="button"
              className={button('signal', 'self-start', 'sm')}
              onClick={() => {
                onAgree()
                setOpen(false)
              }}
            >
              I agree
            </button>
          )}
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  )
}

/** Open until the assistant is connected, then folded to one line so the conversation has the room. */
export function ConnectionBar({
  apiKey,
  model,
  onApiKey,
  onModel,
  connected,
  disabled,
  onConnect,
  onForget,
  context,
  maxIters,
  maxRequests,
  maxTokens,
  onMaxIters,
  onMaxRequests,
  onMaxTokens,
  credit = { kind: 'unchecked' },
}: {
  readonly apiKey: string
  readonly model: string
  readonly onApiKey: (value: string) => void
  readonly onModel: (value: string) => void
  readonly connected: boolean
  readonly disabled: boolean
  readonly onConnect: () => void
  readonly onForget: () => void
  readonly context: string
  readonly maxIters: string
  readonly maxRequests: string
  readonly maxTokens: string
  readonly onMaxIters: (value: string) => void
  readonly onMaxRequests: (value: string) => void
  readonly onMaxTokens: (value: string) => void
  readonly credit?: CreditStatus
}) {
  const [open, setOpen] = useState(!connected)
  const [agreed, setAgreed] = useState(connected)
  const [localKey, setLocalKey] = useState<'idle' | 'loading' | 'failed'>('idle')
  const localRequest = useRef<AbortController | null>(null)
  useEffect(() => () => localRequest.current?.abort(), [])
  const loadLocalKey = async () => {
    const controller = new AbortController()
    localRequest.current?.abort()
    localRequest.current = controller
    setLocalKey('loading')
    try {
      const response = await fetch('/__hirmos/local-openrouter-key', {
        method: 'POST',
        headers: { 'X-Hirmos-Local-Key': 'load' },
        signal: controller.signal,
        cache: 'no-store',
      })
      if (!response.ok) throw new Error('Local key unavailable.')
      const value: unknown = await response.json()
      if (
        typeof value !== 'object' ||
        value === null ||
        !('key' in value) ||
        typeof value.key !== 'string' ||
        !value.key.trim()
      )
        throw new Error('Local key unavailable.')
      if (!controller.signal.aborted) {
        onApiKey(value.key)
        setLocalKey('idle')
      }
    } catch {
      if (!controller.signal.aborted) setLocalKey('failed')
    }
  }
  useEffect(() => {
    if (connected) setOpen(false)
  }, [connected])
  const limits = assistantLimits.safeParse({
    maxIters: Number(maxIters),
    maxRequests: Number(maxRequests),
    maxTokens: Number(maxTokens),
  })
  const ready = apiKey.trim() !== '' && model.trim() !== '' && limits.success
  return (
    <section
      aria-label="OpenRouter connection"
      className="flex max-h-[65%] shrink-0 flex-col border-b border-line bg-well"
    >
      <button
        type="button"
        aria-expanded={open}
        onClick={() => setOpen((value) => !value)}
        className="@container/connection flex min-h-9 w-full shrink-0 items-center gap-2 px-4 text-left text-label text-muted transition-colors hover:text-ink"
      >
        <Icon
          name="key"
          size={15}
          fill={connected}
          className={cn('shrink-0', connected ? 'text-ok' : 'text-faint')}
        />
        <span className={label('text-ink @max-[20rem]/connection:hidden')}>OpenRouter</span>
        {model.trim() === '' ? null : (
          <span className="min-w-0 truncate font-mono text-faint">{modelName(model)}</span>
        )}
        {connected && credit.kind !== 'unchecked' ? (
          <span className="ml-auto flex min-w-0 justify-end">
            <CreditGauge credit={credit} />
          </span>
        ) : (
          <span className="ml-auto shrink-0">{connected ? 'Configured' : 'Not configured'}</span>
        )}
        <Icon name={open ? 'expand_less' : 'expand_more'} size={15} className="shrink-0" />
      </button>
      {open ? (
        <div className="panel-scroll flex min-h-0 flex-col gap-4 overflow-y-auto px-4 pb-4 pt-1">
          <label className="flex flex-col gap-1.5">
            <span className="text-body font-medium">API key</span>
            <input
              autoComplete="off"
              aria-label="OpenRouter API key"
              type="password"
              className={field('text', 'font-mono')}
              value={apiKey}
              disabled={disabled}
              onChange={(event) => onApiKey(event.target.value)}
            />
            <span className="text-label text-faint">
              The key stays in memory and clears when this panel closes.
            </span>
          </label>
          {import.meta.env.DEV ? (
            <div className="grid gap-2">
              <button
                type="button"
                className={button('quiet', 'justify-self-start', 'sm')}
                disabled={disabled || localKey === 'loading'}
                onClick={() => void loadLocalKey()}
              >
                {localKey === 'loading' ? 'Loading local key…' : 'Load local key'}
              </button>
              <span className="text-label text-faint">
                Development only: load OPENROUTER_API_KEY from the local .env into this browser
                session. This does not contact OpenRouter.
              </span>
              {localKey === 'failed' ? (
                <p role="alert" className="m-0 text-label">
                  The local key could not be loaded. Use localhost:5179 and check the .env file.
                </p>
              ) : null}
            </div>
          ) : null}
          <label className="flex flex-col gap-1.5">
            <span className="text-body font-medium">Model ID</span>
            <input
              className={field('text', 'font-mono')}
              placeholder="provider/model"
              aria-label="OpenRouter model ID"
              value={model}
              disabled={disabled}
              onChange={(event) => onModel(event.target.value)}
            />
          </label>
          <div className={fieldRow.two}>
            <label>
              <ParameterLabel
                className={fieldLabel}
                label="Maximum steps per agent"
                help="Each agent has its own step limit. A question can run for up to four minutes."
              />
              <NumberInput
                min={1}
                max={24}
                step={1}
                aria-label="Maximum steps per agent"
                className={field('text', 'mt-1 w-full')}
                value={maxIters}
                disabled={disabled}
                onChange={(event) => onMaxIters(event.target.value)}
              />
            </label>
            <label>
              <ParameterLabel
                className={fieldLabel}
                label="Shared request limit"
                help="Requests, including truncation retries, share one budget across the parent and subagents. Higher limits can increase provider charges."
              />
              <NumberInput
                min={1}
                max={120}
                step={1}
                aria-label="Shared request limit"
                className={field('text', 'mt-1 w-full')}
                value={maxRequests}
                disabled={disabled}
                onChange={(event) => onMaxRequests(event.target.value)}
              />
            </label>
          </div>
          <label>
            <ParameterLabel
              className={fieldLabel}
              label="Initial output tokens per request"
              help="Includes reasoning tokens on most providers. A truncated reply is retried once with up to four times this allowance, capped at 64,000. Your model may support less; provider limits still apply."
            />
            <NumberInput
              min={1}
              max={64000}
              step={1}
              aria-label="Initial output tokens per request"
              className={field('text', 'mt-1 w-full')}
              value={maxTokens}
              disabled={disabled}
              onChange={(event) => onMaxTokens(event.target.value)}
            />
          </label>
          {limits.success ? null : (
            <p role="alert" className="m-0 text-label text-danger">
              Choose 1–24 steps, 1–120 shared requests and 1–64,000 output tokens, with at least one
              request per step.
            </p>
          )}
          <div className="flex flex-wrap items-center gap-2">
            {connected ? (
              <button
                type="button"
                className={button('quiet', undefined, 'sm')}
                disabled={disabled}
                onClick={() => {
                  setAgreed(false)
                  onForget()
                }}
              >
                Forget the key
              </button>
            ) : (
              <button
                type="button"
                className={button('signal')}
                disabled={!ready || !agreed || disabled}
                onClick={onConnect}
              >
                Use these settings
              </button>
            )}
            <PrivacyNotice context={context} agreed={agreed} onAgree={() => setAgreed(true)} />
          </div>
        </div>
      ) : null}
    </section>
  )
}
