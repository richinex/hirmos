import { SettingsDisclosure } from '@/components/ui/SettingsDisclosure'
import { useRef, useState } from 'react'
import {
  rootCauseRequestSchema,
  rootCauseRunSchema,
  type RootCauseRequest,
} from '@/domain/rootCauseAnalysis'
import { matchesRootCauseGraph, type RootCauseGraph } from '@/domain/rootCause'
import { button, fieldHint } from '@/components/ui/recipes'

export function RootCauseSettings({
  graph,
  value,
  onChange,
}: {
  readonly graph: RootCauseGraph
  readonly value: RootCauseRequest | null
  readonly onChange: (value: RootCauseRequest | null) => void
}) {
  const input = useRef<HTMLInputElement>(null)
  const [problem, setProblem] = useState<string | null>(null)
  const load = async (file: File) => {
    try {
      const decoded: unknown = JSON.parse(await file.text())
      const settings = rootCauseRequestSchema
        .or(rootCauseRunSchema.transform((run) => run.model))
        .parse(decoded)
      if (!matchesRootCauseGraph(graph, settings))
        throw new Error('The settings must use the selected graph and its variable order.')
      onChange(settings)
      setProblem(null)
    } catch (error: unknown) {
      setProblem(error instanceof Error ? error.message : String(error))
    }
  }
  return (
    <SettingsDisclosure
      title="Reuse saved settings"
      items={[{ icon: 'upload_file', text: value === null ? 'none loaded' : 'loaded' }]}
    >
      <div className="space-y-3">
        <input
          ref={input}
          type="file"
          accept=".json"
          aria-label="Saved analysis settings"
          className="hidden"
          onChange={(event) => {
            const file = event.target.files?.[0]
            if (file !== undefined) void load(file)
          }}
        />
        <div>
          <button
            type="button"
            className={button('outline')}
            onClick={() => input.current?.click()}
          >
            Load saved settings
          </button>
          <p className={`${fieldHint} max-w-[65ch]`}>
            Choose a settings file or exported analysis record. To repeat the analysis, select the
            same input data separately; it is not included in the record.
          </p>
        </div>
        {value !== null && (
          <>
            <p className={`${fieldHint} max-w-[65ch]`} role="status">
              Loaded{' '}
              {value.query.kind === 'anomaly'
                ? 'unusual-observation'
                : value.query.kind === 'change'
                  ? 'distribution-change'
                  : 'shift-intervention'}{' '}
              settings for {value.names[value.target]}, with {value.repetitions} refitted estimates.
            </p>
            <button type="button" className={button('quiet')} onClick={() => onChange(null)}>
              Use editable settings
            </button>
          </>
        )}
        {problem !== null && (
          <p role="alert" className="text-warn">
            {problem}
          </p>
        )}
      </div>
    </SettingsDisclosure>
  )
}
