import { memo, useEffect, useLayoutEffect, useMemo, useRef, useState, type ReactNode } from 'react'
import { EChart } from '@/charts/EChart'
import { runComparisonOption, type RunComparisonRow } from '@/charts/estimation/runComparison'
import { useChartTheme } from '@/charts/theme'
import { Icon } from '@/components/Icon'
import { SurrogateResult } from '@/components/estimation/SurrogateResult'
import { ResultRecord, ResultSummary, type BiteCheck } from '@/components/results/ResultRecord'
import { GcmEffectResult } from '@/components/root-cause/GcmEffectsPanel'
import { GcmInfluenceResult } from '@/components/root-cause/GcmInfluencePanel'
import { RootCauseRunResult, rootCauseRunTitle } from '@/components/root-cause/RootCauseRunResult'
import { DidSensitivityResult } from '@/components/sensitivity/DidSensitivityPanel'
import { HonestDidResult } from '@/components/sensitivity/HonestDidPanel'
import { PretestCoverageResult } from '@/components/sensitivity/PretestCoveragePanel'
import { SensitivityRunResult } from '@/components/sensitivity/SensitivityPanel'
import { SurvivalRunResult, survivalRunLabel } from '@/components/survival/SurvivalRunResult'
import { CountSeriesRecord } from '@/components/time-series/CountSeriesCard'
import { TimeSeriesRunResult } from '@/components/time-series/TimeSeriesRunResult'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { chromeAction, table, td, th, tr, num } from '@/components/ui/recipes'
import { useWorkspaceConfiguration } from '@/assistant/workspaceConfiguration'
import { assertNever, err, ok } from '@/domain/dop'
import { describeRun, headlineValue, type EstimationRunArtifact } from '@/domain/estimation'
import { buildResultManifest, compareResults, type ResultInputs } from '@/domain/results'
import {
  DESK_VIEWS,
  deskKey,
  parseDeskKey,
  type DeskState,
  type DeskViewName,
  type DeskWindowRef,
} from '@/domain/resultsDesk'
import { sensitivityEstimationRun, type SensitivityRunArtifact } from '@/domain/sensitivity'
import { estimandSentence, type StudySpecification } from '@/domain/study'
import { timeSeriesRunLabel, type TimeSeriesRun } from '@/domain/timeSeries'
import type { SurvivalRunArtifact } from '@/domain/survival'
import type { SurrogateRun } from '@/domain/surrogateRun'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'
import type { RootCauseRun } from '@/domain/rootCauseAnalysis'
import type { GcmEffectsRun } from '@/domain/gcmEffects'
import type { GcmInfluenceRun } from '@/domain/gcmInfluence'
import { formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { cn } from '@/lib/utils'
import { z } from 'zod'
import {
  closeWindow,
  emptyDesk,
  focusWindow,
  loadDesk,
  minimizeWindow,
  openWindow,
  placeWindows,
  resetRatios,
  setPair,
  setView,
  toggleMaximize,
  useDesk,
} from './deskStore'
import { OnDesk } from './PutOnDesk'
import { createDeskEngine, type DeskEngine, type DeskTrees } from './engine'
import { grid, inOrder, stack, TILE, TILE_GAP, type Split } from './layout'

export interface DeskRecords {
  readonly inputs: ResultInputs
  readonly stepLabel: string
  readonly studies: readonly StudySpecification[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly survivalRuns: readonly SurvivalRunArtifact[]
  readonly surrogateRuns: readonly SurrogateRun[]
  readonly timeSeriesRuns: readonly TimeSeriesRun[]
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly rootCauseRuns: readonly RootCauseRun[]
  readonly gcmEffects: readonly GcmEffectsRun[]
  readonly gcmInfluences: readonly GcmInfluenceRun[]
}

const VIEW_LABELS: Record<DeskViewName, string> = {
  desk: 'Desk',
  compare: 'Compare',
  evidence: 'Evidence',
  focus: 'Focus',
}

const VIEW_ICONS: Record<DeskViewName, string> = {
  desk: 'window',
  compare: 'view_column',
  evidence: 'fact_check',
  focus: 'open_in_full',
}

const ICONS: Record<DeskWindowRef['kind'], string> = {
  runs: 'receipt_long',
  differences: 'difference',
  estimate: 'query_stats',
  record: 'assignment',
  sensitivity: 'fact_check',
  survival: 'vital_signs',
  'time-series': 'timeline',
  'count-series': 'bar_chart',
  surrogate: 'science',
  'root-cause': 'account_tree',
  'gcm-effect': 'schema',
  'gcm-influence': 'scatter_plot',
}

const probeTitle = (probe: SensitivityRunArtifact): string => {
  switch (probe.kind) {
    case 'pretest-coverage-run':
      return 'Pre-test coverage'
    case 'honest-did-run':
      return 'Parallel-trends sensitivity'
    case 'did-sensitivity-run':
      return 'DiD omitted-variable sensitivity'
    case 'linear-refutation-run':
      return 'Perturbation and residual probes'
    case 'dml-refutation-run':
      return 'Double machine learning probe batch'
    case 'unobserved-confounding-run':
      return 'Unmeasured confounder'
    default:
      return assertNever(probe)
  }
}

/** Whether the record a window shows still exists, and its title. */
function describeWindow(ref: DeskWindowRef, records: DeskRecords): { title: string } | null {
  const runTitle = (run: EstimationRunArtifact) => {
    const study = records.studies.find((candidate) => candidate.id === run.study)
    const method = `${describeRun(run.configuration)}, ${formatTime(run.createdAt)}`
    return study === undefined ? method : `${estimandSentence(study)}, ${method}`
  }
  switch (ref.kind) {
    case 'runs':
      return { title: 'Saved results' }
    case 'differences':
      return { title: 'Differences' }
    case 'estimate': {
      const run = records.estimationRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined ? null : { title: runTitle(run) }
    }
    case 'record': {
      const run = records.estimationRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined ? null : { title: `Record: ${runTitle(run)}` }
    }
    case 'sensitivity': {
      const probe = records.sensitivityRuns.find((candidate) => candidate.id === ref.id)
      return probe === undefined
        ? null
        : { title: `${probeTitle(probe)}, ${formatTime(probe.createdAt)}` }
    }
    case 'survival': {
      const run = records.survivalRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined ? null : { title: survivalRunLabel(run) }
    }
    case 'time-series': {
      const run = records.timeSeriesRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined ? null : { title: timeSeriesRunLabel(run) }
    }
    case 'count-series': {
      const model = records.countSeriesModels.find((candidate) => candidate.id === ref.id)
      return model === undefined ? null : { title: `Count model, ${formatTime(model.createdAt)}` }
    }
    case 'surrogate': {
      const run = records.surrogateRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined
        ? null
        : { title: `Two-sample surrogate, ${formatTime(run.createdAt)}` }
    }
    case 'root-cause': {
      const run = records.rootCauseRuns.find((candidate) => candidate.id === ref.id)
      return run === undefined
        ? null
        : { title: `${rootCauseRunTitle(run)}, ${run.comparison.name}` }
    }
    case 'gcm-effect': {
      const run = records.gcmEffects.find((candidate) => candidate.id === ref.id)
      return run === undefined
        ? null
        : { title: `Causal-model effects, ${formatTime(run.createdAt)}` }
    }
    case 'gcm-influence': {
      const run = records.gcmInfluences.find((candidate) => candidate.id === ref.id)
      return run === undefined ? null : { title: `Causal influence, ${formatTime(run.createdAt)}` }
    }
    default:
      return assertNever(ref)
  }
}

function Differences({
  records,
  pair,
}: {
  readonly records: DeskRecords
  readonly pair: DeskState['pair']
}) {
  const theme = useChartTheme()
  const left = records.estimationRuns.find((run) => run.id === pair.selected) ?? null
  const right = records.estimationRuns.find((run) => run.id === pair.compareWith) ?? null
  const differences = useMemo(() => {
    if (left === null || right === null) return null
    const stamp = new Date().toISOString()
    return compareResults(
      buildResultManifest(records.inputs, left, stamp),
      buildResultManifest(records.inputs, right, stamp),
    )
  }, [left, right, records.inputs])
  const chart = useMemo(() => {
    if (left === null || right === null) return null
    if (left.estimate.effect.kind !== 'additive' || right.estimate.effect.kind !== 'additive')
      return null
    const rows: RunComparisonRow[] = [right, left].map((run) => ({
      label: `${describeRun(run.configuration)}, ${formatTime(run.createdAt)}`,
      axisLabel: `${describeRun(run.configuration)}\n${formatTime(run.createdAt)}`,
      estimate: run.estimate.effect.kind === 'additive' ? run.estimate.effect.value : 0,
      lower: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.lower,
      upper: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.upper,
      current: run === left,
    }))
    return { option: runComparisonOption(rows, theme), height: rows.length * 88 + 56 }
  }, [left, right, theme])
  if (left === null || right === null || differences === null)
    return (
      <p className="m-0 text-body text-muted">
        Choose an estimate and one to compare with in Saved results.
      </p>
    )
  return (
    <div className="grid gap-4">
      {chart !== null && (
        <EChart
          option={chart.option}
          label="The two estimates on one axis"
          className="w-full"
          style={{ height: chart.height }}
        />
      )}
      {differences.length === 0 ? (
        <p className="m-0 text-body text-muted">The two runs share every recorded field.</p>
      ) : (
        <div className="figure-strip overflow-x-auto">
          <table className={table}>
            <thead>
              <tr>
                <th className={th()}>Field</th>
                <th className={th()}>Estimate</th>
                <th className={th()}>Compare with</th>
              </tr>
            </thead>
            <tbody>
              {differences.map((difference) => (
                <tr key={difference.field} className={tr()}>
                  <td className={td('text-ink')}>{difference.field}</td>
                  <td className={td('whitespace-normal text-muted')}>{difference.left}</td>
                  <td className={td('whitespace-normal text-muted')}>{difference.right}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}

/** Saved results other than estimates, newest first. */
const otherResults = (
  records: DeskRecords,
): { ref: DeskWindowRef; title: string; created: string }[] =>
  [
    ...records.survivalRuns.map((run) => ({
      ref: { kind: 'survival', id: String(run.id) } as const,
      title: survivalRunLabel(run),
      created: run.createdAt,
    })),
    ...records.timeSeriesRuns.map((run) => ({
      ref: { kind: 'time-series', id: String(run.id) } as const,
      title: timeSeriesRunLabel(run),
      created: run.createdAt,
    })),
    ...records.countSeriesModels.map((model) => ({
      ref: { kind: 'count-series', id: String(model.id) } as const,
      title: 'Count model',
      created: model.createdAt,
    })),
    ...records.surrogateRuns.map((run) => ({
      ref: { kind: 'surrogate', id: String(run.id) } as const,
      title: 'Two-sample surrogate',
      created: run.createdAt,
    })),
    ...records.rootCauseRuns.map((run) => ({
      ref: { kind: 'root-cause', id: String(run.id) } as const,
      title: `${rootCauseRunTitle(run)}, ${run.comparison.name}`,
      created: run.createdAt,
    })),
    ...records.gcmEffects.map((run) => ({
      ref: { kind: 'gcm-effect', id: String(run.id) } as const,
      title: 'Causal-model effects',
      created: run.createdAt,
    })),
    ...records.gcmInfluences.map((run) => ({
      ref: { kind: 'gcm-influence', id: String(run.id) } as const,
      title: 'Causal influence',
      created: run.createdAt,
    })),
    ...records.sensitivityRuns.map((probe) => ({
      ref: { kind: 'sensitivity', id: String(probe.id) } as const,
      title: probeTitle(probe),
      created: probe.createdAt,
    })),
  ].sort((a, b) => b.created.localeCompare(a.created))

function SavedResults({
  records,
  pair,
  onOpen,
}: {
  readonly records: DeskRecords
  readonly pair: DeskState['pair']
  readonly onOpen: (ref: DeskWindowRef) => void
}) {
  const studyOf = (run: EstimationRunArtifact) =>
    records.studies.find((study) => study.id === run.study)
  const others = otherResults(records)
  return (
    <div className="grid grid-cols-[minmax(0,1fr)] gap-4">
      {records.estimationRuns.length > 0 && (
        <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Estimates">
          {[...records.estimationRuns].reverse().map((run) => {
            const study = studyOf(run)
            const effect = run.estimate.effect
            const id = String(run.id)
            const selected = id === pair.selected
            return (
              <li
                key={run.id}
                className="flex items-center gap-3 py-2"
                aria-current={selected || undefined}
              >
                <span
                  className={cn(
                    'h-5 w-0.5 shrink-0 rounded-full',
                    selected ? 'bg-signal' : 'bg-transparent',
                  )}
                  aria-hidden
                />
                <span className="min-w-0 flex-1">
                  <span className="block truncate text-ink">
                    {study === undefined ? run.method : estimandSentence(study)}
                  </span>
                  <span className="block truncate text-label text-faint">
                    {describeRun(run.configuration)}, {formatTime(run.createdAt)}
                  </span>
                </span>
                <span className={num('shrink-0 text-ink')}>
                  {
                    formatStatistic(
                      'raw',
                      effect.kind === 'path' ? effect.aggregate.average : headlineValue(effect),
                    ).text
                  }
                </span>
                <button
                  type="button"
                  className={chromeAction('quiet', 'shrink-0')}
                  onClick={() => {
                    setPair({
                      ...pair,
                      selected: id,
                      compareWith: pair.compareWith === id ? null : pair.compareWith,
                    })
                    onOpen({ kind: 'record', id })
                    onOpen({ kind: 'estimate', id })
                  }}
                >
                  Open
                </button>
                <button
                  type="button"
                  className={chromeAction(
                    pair.compareWith === id ? 'selected' : 'quiet',
                    'shrink-0',
                  )}
                  disabled={pair.selected === null || selected}
                  aria-pressed={pair.compareWith === id}
                  title={
                    pair.selected === null || selected
                      ? 'Open another estimate first'
                      : 'Show this estimate beside the open one'
                  }
                  onClick={() => {
                    setPair({ ...pair, compareWith: id })
                    setView('compare', [
                      { kind: 'estimate', id: pair.selected! },
                      { kind: 'estimate', id },
                      { kind: 'differences' },
                    ])
                  }}
                >
                  Compare
                </button>
              </li>
            )
          })}
        </ul>
      )}
      {others.length > 0 && (
        <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Other results">
          {others.map((item) => (
            <li key={deskKey(item.ref)} className="flex items-center gap-3 py-2">
              <span className="h-5 w-0.5 shrink-0" aria-hidden />
              <span className="min-w-0 flex-1 truncate text-ink">{item.title}</span>
              <span className={num('shrink-0 text-label text-faint')}>
                {formatTime(item.created)}
              </span>
              <button
                type="button"
                className={chromeAction('quiet', 'shrink-0')}
                onClick={() => onOpen(item.ref)}
              >
                Open
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

const WindowBody = memo(function WindowBody({
  deskRef,
  records,
  pair,
  onOpen,
}: {
  readonly deskRef: DeskWindowRef
  readonly records: DeskRecords
  readonly pair: DeskState['pair']
  readonly onOpen: (ref: DeskWindowRef) => void
}) {
  const manifestOf = (id: string) => {
    const run = records.estimationRuns.find((candidate) => candidate.id === id)
    return run === undefined
      ? null
      : { run, manifest: buildResultManifest(records.inputs, run, new Date().toISOString()) }
  }
  switch (deskRef.kind) {
    case 'runs':
      return <SavedResults records={records} pair={pair} onOpen={onOpen} />
    case 'differences':
      return <Differences records={records} pair={pair} />
    case 'estimate': {
      const found = manifestOf(deskRef.id)
      return found === null ? null : (
        <ResultSummary manifest={found.manifest} stepLabel={records.stepLabel} />
      )
    }
    case 'record': {
      const found = manifestOf(deskRef.id)
      if (found === null) return null
      const study = records.studies.find((candidate) => candidate.id === found.run.study)
      const bite: BiteCheck[] = records.studies
        .filter((check) => check.biteOf === found.run.study)
        .map((check) => ({
          study: check,
          run:
            records.estimationRuns.filter((candidate) => candidate.study === check.id).at(-1) ??
            null,
        }))
      return (
        <ResultRecord
          manifest={found.manifest}
          stepLabel={records.stepLabel}
          bite={bite}
          checked={records.studies.find((check) => check.id === study?.biteOf) ?? null}
        />
      )
    }
    case 'sensitivity': {
      const probe = records.sensitivityRuns.find((candidate) => candidate.id === deskRef.id)
      if (probe === undefined) return null
      if (probe.kind === 'honest-did-run') return <HonestDidResult run={probe} />
      if (probe.kind === 'pretest-coverage-run') return <PretestCoverageResult run={probe} />
      if (probe.kind === 'did-sensitivity-run')
        return (
          <DidSensitivityResult
            run={probe}
            source={records.estimationRuns.find((run) => run.id === probe.estimationRun)}
          />
        )
      const study = records.studies.find(
        (candidate) =>
          candidate.id ===
          records.estimationRuns.find((run) => run.id === probe.estimationRun)?.study,
      )
      return study === undefined ? null : <SensitivityRunResult run={probe} study={study} />
    }
    case 'survival': {
      const run = records.survivalRuns.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : <SurvivalRunResult run={run} />
    }
    case 'time-series': {
      const run = records.timeSeriesRuns.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : <TimeSeriesRunResult run={run} />
    }
    case 'count-series': {
      const model = records.countSeriesModels.find((candidate) => candidate.id === deskRef.id)
      return model === undefined ? null : (
        <ul className="m-0 list-none p-0">
          <CountSeriesRecord artifact={model} open />
        </ul>
      )
    }
    case 'surrogate': {
      const run = records.surrogateRuns.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : (
        <SurrogateResult run={run} profile={records.inputs.profile} />
      )
    }
    case 'root-cause': {
      const run = records.rootCauseRuns.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : <RootCauseRunResult run={run} />
    }
    case 'gcm-effect': {
      const run = records.gcmEffects.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : <GcmEffectResult run={run} />
    }
    case 'gcm-influence': {
      const run = records.gcmInfluences.find((candidate) => candidate.id === deskRef.id)
      return run === undefined ? null : <GcmInfluenceResult run={run} />
    }
    default:
      return assertNever(deskRef)
  }
})

function DeskWindow({
  windowKey,
  title,
  icon,
  closable,
  engine,
  onMinimize,
  onMaximize,
  onClose,
  children,
}: {
  readonly windowKey: string
  readonly title: string
  readonly icon: string
  readonly closable: boolean
  readonly engine: DeskEngine | null
  readonly onMinimize: () => void
  readonly onMaximize: () => void
  readonly onClose: () => void
  readonly children: ReactNode
}) {
  const frame = useRef<HTMLElement>(null)
  const content = useRef<HTMLDivElement>(null)
  const titlebar = useRef<HTMLDivElement>(null)
  const grip = useRef<HTMLDivElement>(null)
  const badge = useRef<HTMLSpanElement>(null)
  const handle = useRef<HTMLButtonElement>(null)
  useLayoutEffect(() => {
    if (
      engine === null ||
      !frame.current ||
      !content.current ||
      !titlebar.current ||
      !grip.current ||
      !badge.current ||
      !handle.current
    )
      return
    return engine.register(windowKey, {
      frame: frame.current,
      content: content.current,
      titlebar: titlebar.current,
      grip: grip.current,
      badge: badge.current,
      handle: handle.current,
    })
  }, [engine, windowKey])
  return (
    <section
      ref={frame}
      className="desk-window"
      aria-label={title}
      data-key={windowKey}
      tabIndex={-1}
    >
      <div ref={titlebar} className="desk-titlebar">
        <div className="desk-title">
          <button
            ref={handle}
            type="button"
            className="desk-handle"
            aria-label={`Move ${title}`}
            aria-keyshortcuts="ArrowLeft ArrowRight ArrowUp ArrowDown"
            title="Drag to another place, or use the arrow keys"
          >
            <Icon name={icon} size={15} />
          </button>
          <h3 className="desk-name" title={title}>
            {title}
          </h3>
          <span ref={badge} className="desk-badge" aria-hidden />
        </div>
        <div className="desk-controls">
          <button
            type="button"
            className="desk-button minimize"
            aria-label={`Minimize ${title}`}
            title="Minimize"
            onClick={onMinimize}
          />
          <button
            type="button"
            className="desk-button maximize"
            aria-label={`Maximize ${title}`}
            title="Maximize"
            onClick={onMaximize}
          />
          {closable && (
            <button
              type="button"
              className="desk-button close"
              aria-label={`Close ${title}`}
              title="Close"
              onClick={onClose}
            />
          )}
        </div>
      </div>
      <div ref={content} className="desk-content">
        <OnDesk.Provider value={true}>{children}</OnDesk.Provider>
      </div>
      <div ref={grip} className="desk-grip" aria-hidden />
    </section>
  )
}

function DockTile({
  windowKey,
  title,
  icon,
  engine,
  onActivate,
}: {
  readonly windowKey: string
  readonly title: string
  readonly icon: string
  readonly engine: DeskEngine | null
  readonly onActivate: () => void
}) {
  const tile = useRef<HTMLButtonElement>(null)
  useLayoutEffect(() => {
    if (engine === null || tile.current === null) return
    return engine.registerTile(windowKey, tile.current)
  }, [engine, windowKey])
  return (
    <button
      ref={tile}
      type="button"
      className="desk-tile"
      title={title}
      aria-label={title}
      onClick={onActivate}
    >
      <Icon name={icon} size={18} />
      <i aria-hidden />
    </button>
  )
}

const rootRatio = (desk: DeskState, id: string, fallback: number) => desk.ratios[id] ?? fallback

export function ResultsDesk({
  project,
  records,
  onExportProject,
}: {
  readonly project: string
  readonly records: DeskRecords
  readonly onExportProject: () => void
}) {
  const surface = useRef<HTMLDivElement>(null)
  const [engine, setEngine] = useState<DeskEngine | null>(null)
  const desk = useDesk((state) => state.desk)
  const loaded = useDesk((state) => state.project)
  const dropped = useDesk((state) => state.dropped)

  const exists = (ref: DeskWindowRef) => describeWindow(ref, records) !== null
  const latest = records.estimationRuns.at(-1)
  const previous = records.estimationRuns.at(-2)

  useLayoutEffect(() => {
    loadDesk(
      project,
      () => {
        const seeded = emptyDesk()
        if (latest === undefined) {
          const other = otherResults(records)[0]
          if (other === undefined) return seeded
          const key = deskKey(other.ref)
          return {
            ...seeded,
            windows: [
              ...seeded.windows,
              { key, placed: null, visible: true, maximized: false, rank: 1 },
            ],
            focused: key,
          }
        }
        const id = String(latest.id)
        return {
          ...seeded,
          windows: [
            ...seeded.windows,
            {
              key: deskKey({ kind: 'estimate', id }),
              placed: null,
              visible: true,
              maximized: false,
              rank: 1,
            },
            {
              key: deskKey({ kind: 'record', id }),
              placed: null,
              visible: true,
              maximized: false,
              rank: 2,
            },
          ],
          focused: deskKey({ kind: 'estimate', id }),
          pair: { selected: id, compareWith: previous === undefined ? null : String(previous.id) },
        }
      },
      exists,
    )
    // The desk loads once per project; records that disappear later leave through `exists` on the next load.
  }, [project])

  const sensitivityOf = (id: string) =>
    records.sensitivityRuns.filter((probe) => sensitivityEstimationRun(probe) === id).slice(-2)

  const compareReady =
    desk.pair.selected !== null &&
    desk.pair.compareWith !== null &&
    exists({ kind: 'estimate', id: desk.pair.compareWith })
  const evidenceReady =
    desk.pair.selected !== null && exists({ kind: 'estimate', id: desk.pair.selected })

  const trees = useMemo<DeskTrees>(() => {
    const result: DeskTrees = {}
    const { selected, compareWith } = desk.pair
    if (selected !== null && compareWith !== null) {
      result.compare = {
        kind: 'split',
        id: 'compare.rows',
        dir: 'col',
        ratio: rootRatio(desk, 'compare.rows', 0.5),
        a: {
          kind: 'split',
          id: 'compare.columns',
          dir: 'row',
          ratio: rootRatio(desk, 'compare.columns', 0.5),
          a: { kind: 'leaf', key: deskKey({ kind: 'estimate', id: selected }) },
          b: { kind: 'leaf', key: deskKey({ kind: 'estimate', id: compareWith }) },
        },
        b: { kind: 'leaf', key: 'differences' },
      }
    }
    if (selected !== null) {
      const column = stack(
        [
          deskKey({ kind: 'estimate', id: selected }),
          ...sensitivityOf(selected).map((probe) =>
            deskKey({ kind: 'sensitivity', id: String(probe.id) }),
          ),
        ],
        'evidence.stack',
      )
      result.evidence = {
        kind: 'split',
        id: 'evidence.columns',
        dir: 'row',
        ratio: rootRatio(desk, 'evidence.columns', 0.45),
        a: { kind: 'leaf', key: deskKey({ kind: 'record', id: selected }) },
        b: column as Split,
      }
    }
    if (result.compare) result.compare = inOrder(result.compare, desk.order.compare)
    if (result.evidence) result.evidence = inOrder(result.evidence, desk.order.evidence)
    return result
  }, [desk.pair, desk.ratios, desk.order, records.sensitivityRuns])

  const treesRef = useRef(trees)
  useLayoutEffect(() => {
    treesRef.current = trees
    engine?.refresh()
  }, [trees, engine])

  useLayoutEffect(() => {
    if (surface.current === null) return
    const created = createDeskEngine(surface.current, () => treesRef.current)
    setEngine(created)
    return () => created.dispose()
  }, [])

  const viewWindows = (view: DeskViewName, pair: DeskState['pair']): DeskWindowRef[] => {
    const { selected, compareWith } = pair
    if (view === 'compare' && selected !== null && compareWith !== null)
      return [
        { kind: 'estimate', id: selected },
        { kind: 'estimate', id: compareWith },
        { kind: 'differences' },
      ]
    if (view === 'evidence' && selected !== null)
      return [
        { kind: 'record', id: selected },
        { kind: 'estimate', id: selected },
        ...sensitivityOf(selected).map(
          (probe) => ({ kind: 'sensitivity', id: String(probe.id) }) as const,
        ),
      ]
    return []
  }

  const chooseView = (view: DeskViewName) => setView(view, viewWindows(view, desk.pair))

  const arrange = () => {
    if (engine === null) return
    if (desk.view === 'compare' || desk.view === 'evidence') {
      resetRatios(engine.seamIds())
      return
    }
    const open = [...desk.windows]
      .filter((window) => window.visible)
      .sort((a, b) => a.rank - b.rank)
    const space = engine.workspace()
    placeWindows(
      grid(
        open.map((window) => window.key),
        space,
      ),
      space,
    )
  }

  useWorkspaceConfiguration('results-desk', project, {
    title: 'Results desk',
    current: {
      view: desk.view,
      open: desk.windows.filter((window) => window.visible).map((window) => window.key),
      pair: desk.pair,
    },
    schema: z
      .object({
        view: z.enum(DESK_VIEWS),
        open: z.array(z.string()).max(10),
        pair: z
          .object({ selected: z.string().nullable(), compareWith: z.string().nullable() })
          .strict(),
      })
      .strict(),
    columns: [],
    describe: (value) => [
      { key: 'view', label: 'View', value: VIEW_LABELS[value.view] },
      {
        key: 'open',
        label: 'Windows',
        value:
          value.open
            .map((key) => {
              const ref = parseDeskKey(key)
              return ref === null ? key : (describeWindow(ref, records)?.title ?? key)
            })
            .join('; ') || 'none',
      },
    ],
    validate: (value) => {
      for (const key of value.open) {
        const ref = parseDeskKey(key)
        if (ref === null || !exists(ref)) return err(`${key} is not a saved result on this desk.`)
      }
      for (const id of [value.pair.selected, value.pair.compareWith]) {
        if (id !== null && !exists({ kind: 'estimate', id }))
          return err(`Estimate ${id} is not saved in this project.`)
      }
      if (
        value.view === 'compare' &&
        (value.pair.selected === null || value.pair.compareWith === null)
      )
        return err('Compare needs an estimate and one to compare with.')
      return ok(value)
    },
    configure: (value) => {
      setPair(value.pair)
      for (const key of value.open) {
        const ref = parseDeskKey(key)
        if (ref !== null) openWindow(ref)
      }
      setView(value.view, viewWindows(value.view, value.pair))
    },
  })

  const onKeyDown = (event: React.KeyboardEvent) => {
    if (event.metaKey || event.ctrlKey || event.altKey) return
    if ((event.target as Element).closest('input, textarea, select, [contenteditable]')) return
    if (event.key === 'Escape') {
      const focused = desk.windows.find((window) => window.key === desk.focused)
      if (focused?.maximized) {
        event.preventDefault()
        toggleMaximize(focused.key)
      }
      return
    }
    if (event.key === 'F6') {
      const shown = [...desk.windows]
        .filter((window) => window.visible)
        .sort((a, b) => b.rank - a.rank)
      const next = event.shiftKey ? shown.at(-1) : (shown[1] ?? shown[0])
      if (next === undefined) return
      event.preventDefault()
      focusWindow(next.key)
      surface.current?.querySelector<HTMLElement>(`[data-key="${CSS.escape(next.key)}"]`)?.focus()
      return
    }
    const view = DESK_VIEWS[Number(event.key) - 1]
    if (view === undefined) return
    if ((view === 'compare' && !compareReady) || (view === 'evidence' && !evidenceReady)) return
    event.preventDefault()
    chooseView(view)
  }

  const shown = desk.windows
    .map((window) => {
      const ref = parseDeskKey(window.key)
      const described = ref === null ? null : describeWindow(ref, records)
      return ref === null || described === null
        ? null
        : { key: window.key, ref, title: described.title }
    })
    .filter((entry) => entry !== null)

  const droppedTitle = useMemo(() => {
    if (dropped === null) return null
    const ref = parseDeskKey(dropped)
    return ref === null ? null : (describeWindow(ref, records)?.title ?? null)
  }, [dropped, records])

  useEffect(() => {
    if (loaded === project) engine?.refresh()
  }, [loaded, project, engine])

  return (
    <section
      aria-labelledby="results-title"
      data-pane
      className="results-desk-pane flex h-full min-h-0 flex-col"
      onKeyDown={onKeyDown}
    >
      <h2 id="results-title" className="sr-only">
        Results
      </h2>
      <div ref={surface} className="results-desk" role="region" aria-label="Results desk">
        {shown.map(({ key, ref, title }) => (
          <DeskWindow
            key={key}
            windowKey={key}
            title={title}
            icon={ICONS[ref.kind]}
            closable={ref.kind !== 'runs'}
            engine={engine}
            onMinimize={() => {
              engine?.freeze()
              minimizeWindow(key)
            }}
            onMaximize={() => {
              if (desk.view !== 'desk') {
                focusWindow(key)
                setView('focus')
                return
              }
              toggleMaximize(key)
            }}
            onClose={() => {
              engine?.freeze()
              closeWindow(key)
            }}
          >
            <WindowBody deskRef={ref} records={records} pair={desk.pair} onOpen={openWindow} />
          </DeskWindow>
        ))}
        <nav className="desk-dock" aria-label="Windows on the desk">
          <span
            className="desk-wire"
            aria-hidden
            style={{ width: `${shown.length * (TILE + TILE_GAP) + 28}px` }}
          />
          {shown.map(({ key, ref, title }) => (
            <DockTile
              key={key}
              windowKey={key}
              title={title}
              icon={ICONS[ref.kind]}
              engine={engine}
              onActivate={() => {
                const window = desk.windows.find((candidate) => candidate.key === key)
                engine?.freeze()
                if (desk.view === 'desk' && window?.visible && desk.focused === key)
                  minimizeWindow(key)
                else openWindow(ref)
              }}
            />
          ))}
        </nav>
        <p role="status" className="desk-note m-0 text-label text-faint">
          {droppedTitle === null
            ? ''
            : `${droppedTitle} left the desk, which holds ten windows at a time.`}
        </p>
        <div role="toolbar" aria-label="Desk" className="desk-bar max-md:hidden">
          <SegmentedControl
            variant="track"
            frame="well"
            size="sm"
            ariaLabel="Desk view"
            value={desk.view}
            onChange={chooseView}
            options={DESK_VIEWS.map((view, index) => ({
              value: view,
              label: (
                <span className="flex items-center gap-1.5">
                  <Icon name={VIEW_ICONS[view]} size={16} />
                  <span className="desk-bar-label">{VIEW_LABELS[view]}</span>
                </span>
              ),
              ariaLabel: VIEW_LABELS[view],
              disabled:
                (view === 'compare' && !compareReady) || (view === 'evidence' && !evidenceReady),
              title:
                view === 'compare' && !compareReady
                  ? 'Choose an estimate and one to compare with in Saved results'
                  : view === 'evidence' && !evidenceReady
                    ? 'Open an estimate first'
                    : `Press ${index + 1} to show ${VIEW_LABELS[view].toLowerCase()}`,
            }))}
          />
          <button
            type="button"
            className={chromeAction('quiet', 'ml-auto text-muted')}
            aria-label="Arrange"
            title={
              desk.view === 'compare' || desk.view === 'evidence'
                ? 'Set the dividers back to their first positions'
                : 'Place the open windows side by side'
            }
            onClick={arrange}
          >
            <Icon name="auto_awesome_mosaic" size={16} />
            <span className="desk-bar-label">Arrange</span>
          </button>
          <button
            type="button"
            className={chromeAction('quiet', 'text-muted')}
            aria-label="Export project"
            title="Export this project as a bundle"
            onClick={onExportProject}
          >
            <Icon name="download" size={16} />
            <span className="desk-bar-label">Export project</span>
          </button>
        </div>
      </div>
    </section>
  )
}
