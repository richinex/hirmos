import { useLayoutEffect, useRef, useState } from 'react'
import { useStore } from 'zustand'
import { z } from 'zod'
import { err, ok, type Result } from '@/domain/dop'
import type { ConfigurationSetting } from '@/domain/estimation'
import { useAssistantBridge } from './applicationTools'
import { ProposalActions, ProposalCard, ProposalSettings } from './ProposalParts'

export interface WorkspaceConfigurationCapability {
  readonly action?: 'settings' | 'open-editor'
  readonly revision: string
  readonly title: string
  readonly current: unknown
  readonly schema: unknown
  readonly columns: readonly { readonly id: string; readonly name: string }[]
  /** The page's own description of a checked configuration; without one, the raw fields are listed. */
  readonly describe: ((value: unknown) => readonly ConfigurationSetting[]) | null
  readonly check: (raw: unknown) => Result<unknown, string>
  readonly apply: (raw: unknown) => Result<void, string>
}
export interface WorkspaceConfigurationProposal {
  readonly action?: 'settings' | 'open-editor'
  readonly key: string
  readonly revision: string
  readonly title: string
  readonly configuration: unknown
  readonly settings: readonly ConfigurationSetting[]
  readonly reason: string
}

export function readWorkspaceSettings(
  capabilities: Readonly<Record<string, WorkspaceConfigurationCapability>>,
  raw: unknown,
) {
  const available = Object.entries(capabilities).map(([key, value]) => ({
    key,
    title: value.title,
  }))
  const key = raw !== null && typeof raw === 'object' && 'key' in raw ? raw.key : undefined
  if (key === undefined) return available
  if (typeof key !== 'string' || !Object.hasOwn(capabilities, key))
    return {
      kind: 'refused',
      reason:
        available.length === 0
          ? 'No workspace settings tools are available on this page. This does not mean the page is closed. Estimation uses the separate estimator proposal tools.'
          : `Unknown workspace settings key. Available keys: ${available.map((entry) => entry.key).join(', ')}.`,
      available,
    }
  const { title, current, columns, schema, describe, action } = capabilities[key]!
  return {
    key,
    title,
    action: action ?? 'settings',
    current,
    columns,
    schema,
    settings: describe?.(current) ?? settings(current, columns),
  }
}
const requestSchema = z
  .object({
    key: z.string().min(1),
    configuration: z.unknown(),
    reason: z.string().trim().min(1).max(2000),
  })
  .strict()

function settings(
  raw: unknown,
  columns: WorkspaceConfigurationCapability['columns'],
  path = '',
): WorkspaceConfigurationProposal['settings'] {
  if (raw !== null && typeof raw === 'object' && !Array.isArray(raw))
    return Object.entries(raw).flatMap(([key, value]) =>
      settings(value, columns, path ? `${path} / ${key}` : key),
    )
  if (Array.isArray(raw))
    return raw.flatMap((value, index) => settings(value, columns, `${path} / ${index + 1}`))
  const reference = /(^|\/ )(outcome|variable)$/.test(path) || /^outcomes \/ \d+$/.test(path)
  const name =
    typeof raw === 'string' || (reference && typeof raw === 'number')
      ? columns.find((column) => column.id === String(raw))?.name
      : undefined
  return [
    {
      key: path,
      label: path.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/^./, (c) => c.toUpperCase()),
      value: name ?? (raw === null ? 'Not selected' : String(raw)),
    },
  ]
}

export function proposeWorkspaceConfiguration(
  capabilities: Readonly<Record<string, WorkspaceConfigurationCapability>>,
  raw: unknown,
): Result<WorkspaceConfigurationProposal, string> {
  const parsed = requestSchema.safeParse(raw)
  if (!parsed.success) return err('Supply the workspace key, complete configuration and reason.')
  const capability = capabilities[parsed.data.key]
  if (!capability) return err('Open the requested workspace before configuring it.')
  const checked = capability.check(parsed.data.configuration)
  if (!checked.ok) return checked
  return ok({
    key: parsed.data.key,
    action: capability.action,
    revision: capability.revision,
    title: capability.title,
    configuration: checked.value,
    settings: capability.describe?.(checked.value) ?? settings(checked.value, capability.columns),
    reason: parsed.data.reason,
  })
}

export function useWorkspaceConfiguration<T>(
  key: string,
  binding: string,
  options: {
    readonly title: string
    readonly action?: 'settings' | 'open-editor'
    readonly current: T
    readonly schema: z.ZodType<T>
    readonly columns: WorkspaceConfigurationCapability['columns']
    readonly describe?: (value: T) => readonly ConfigurationSetting[]
    readonly validate: (value: T) => Result<T, string>
    readonly configure: (value: T) => void
  } | null,
) {
  const bridge = useAssistantBridge()
  const latest = useRef(options)
  useLayoutEffect(() => {
    latest.current = options
  })
  const description = JSON.stringify(
    options && { current: options.current, columns: options.columns },
  )
  useLayoutEffect(() => {
    const value = latest.current
    if (!value) return
    const check = (raw: unknown): Result<T, string> => {
      const current = latest.current
      if (!current) return err('This workspace is unavailable.')
      const parsed = current.schema.safeParse(raw)
      if (!parsed.success)
        return err(
          parsed.error.issues
            .map((issue) => `${issue.path.join('.')}: ${issue.message}`)
            .join('; '),
        )
      return current.validate(parsed.data)
    }
    const capability: WorkspaceConfigurationCapability = {
      revision: crypto.randomUUID(),
      action: value.action,
      title: value.title,
      current: value.current,
      columns: value.columns,
      describe: value.describe
        ? (checked) => (latest.current?.describe ?? value.describe!)(checked as T)
        : null,
      schema: z.toJSONSchema(value.schema, { io: 'input' }),
      check,
      apply: (raw) => {
        const checked = check(raw)
        if (!checked.ok) return checked
        latest.current!.configure(checked.value)
        return ok(undefined)
      },
    }
    bridge.store.setState((state) => ({
      configurations: { ...state.configurations, [key]: capability },
    }))
    return () => {
      if (bridge.store.getState().configurations[key] !== capability) return
      const configurations = { ...bridge.store.getState().configurations }
      delete configurations[key]
      bridge.store.setState({ configurations })
    }
  }, [bridge, key, binding, description, options === null])
}

export function WorkspaceConfigurationCard({
  proposal,
}: {
  readonly proposal: WorkspaceConfigurationProposal
}) {
  const bridge = useAssistantBridge()
  const capability = useStore(bridge.store, (state) => state.configurations[proposal.key])
  const [status, setStatus] = useState<'pending' | 'applied' | 'dismissed'>('pending')
  const [problem, setProblem] = useState<string | null>(null)
  const stale = capability?.revision !== proposal.revision
  return (
    <ProposalCard
      name="Workspace configuration proposal"
      title={`${proposal.action === 'open-editor' ? 'Open' : 'Configure'} ${proposal.title}`}
    >
      <ProposalSettings settings={proposal.settings} />
      <p className="m-0">{proposal.reason}</p>
      {status === 'pending' && !stale ? (
        <ProposalActions
          action={proposal.action === 'open-editor' ? 'Open editor' : 'Apply settings'}
          help={
            proposal.action === 'open-editor'
              ? 'This opens the data editor. A pipeline evaluates its saved blocks locally. The accepted source, prepared dataset and results remain unchanged until you accept a replacement.'
              : `This updates the ${proposal.title} settings. No analysis is run.`
          }
          onAction={() => {
            const current = bridge.store.getState().configurations[proposal.key]
            if (!current || current.revision !== proposal.revision) {
              setProblem('The settings changed. Ask for a new proposal.')
              return
            }
            const result = current.apply(proposal.configuration)
            if (result.ok) setStatus('applied')
            else setProblem(result.error)
          }}
          onDismiss={() => setStatus('dismissed')}
        />
      ) : (
        <p role="status" className="m-0 text-label text-muted">
          {status === 'applied'
            ? proposal.action === 'open-editor'
              ? 'Editor requested. No data was replaced and no analysis was run.'
              : 'Settings applied. No analysis was run.'
            : status === 'dismissed'
              ? 'Proposal dismissed.'
              : 'The workspace or settings changed. Ask for a new proposal.'}
        </p>
      )}
      {problem ? (
        <p role="alert" className="m-0 text-label text-danger">
          {problem}
        </p>
      ) : null}
    </ProposalCard>
  )
}
