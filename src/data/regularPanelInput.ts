import { isNumericDuckDbType, type DatasetProfile } from '@/domain/dataset'
import { assertNever, err, ok, type Result } from '@/domain/dop'
import { describePanelDataProblem } from '@/domain/panel'
import {
  calendarSchedules,
  regularPanel,
  type RegularPanel,
  type PanelProblem,
  type NumericPeriodSpacing,
} from '@/domain/regularPanel'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import type { PreparedMatrix } from './prepared'
import { materializePanelKeysInWorker, previewTimeColumnInWorker } from './client'

export async function prepareRegularPanel(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: Extract<PreparedDatasetArtifact, { kind: 'prepared-panel' }>,
  matrix: PreparedMatrix,
  current: () => boolean,
  numericSpacing: NumericPeriodSpacing = 'consecutive',
): Promise<Result<RegularPanel, PanelProblem>> {
  const { unitColumn, timeColumn, frequency } = prepared.sampling
  const time = profile.columns.find((column) => column.id === timeColumn)
  const unit = profile.columns.find((column) => column.id === unitColumn)
  if (time === undefined || unit === undefined) return err({ kind: 'column-missing' })
  const clock = isNumericDuckDbType(time.duckdbType)
    ? { kind: 'ordinal' as const }
    : { kind: 'calendar' as const, schedules: calendarSchedules(frequency) }
  const raw = await materializePanelKeysInWorker(source.file, profile, unitColumn, timeColumn)
  if (!current()) return err({ kind: 'cancelled' })
  if (!raw.ok) return err({ kind: 'read-failed', detail: describePanelDataProblem(raw.error) })
  if (raw.value.sourceFingerprint !== profile.source.fingerprint)
    return err({ kind: 'source-mismatch' })
  switch (clock.kind) {
    case 'ordinal':
      return regularPanel(raw.value, matrix, clock, numericSpacing)
    case 'calendar': {
      for (const schedule of clock.schedules) {
        const checked = await previewTimeColumnInWorker(
          source.file,
          profile,
          timeColumn,
          { kind: 'source-type' },
          { schedule, unitColumn },
        )
        if (!current()) return err({ kind: 'cancelled' })
        if (!checked.ok) return err({ kind: 'calendar-unavailable' })
        const report = checked.value.calendar
        if (checked.value.kind !== 'calendar' || report === undefined)
          return err({ kind: 'calendar-unavailable' })
        if (report.issues.length > 0) continue
        return regularPanel(raw.value, matrix, { kind: 'calendar', schedule, report })
      }
      return err({ kind: 'calendar-alignment' })
    }
    default:
      return assertNever(clock)
  }
}
