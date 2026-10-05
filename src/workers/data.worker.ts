/// <reference lib="webworker" />

import {
  inspectPanelStructure,
  materializeNumericColumns,
  materializePanelKeys,
  materializePanelLong,
  materializeTimeSeriesColumns,
  previewWindow,
  profileColumn,
  profileSource,
  summarizeColumns,
} from '@/data/duckdb'
import { assertNever } from '@/domain/dop'
import { previewTimeColumn } from '@/data/duckdb'
import { describeSourceSelectionProblem, readAs, selectSource } from '@/domain/workflow'
import { fileReadingOf } from '@/domain/fileReading'
import { parseDataWorkerCommand, type DataWorkerEvent } from './dataProtocol'

const emit = (event: DataWorkerEvent, transfer: Transferable[] = []) =>
  self.postMessage(event, { transfer })

self.onmessage = (message: MessageEvent<unknown>) => {
  void (async () => {
    const parsed = parseDataWorkerCommand(message.data)
    if (!parsed.ok) {
      emit({ kind: 'protocol-failed', detail: parsed.error.detail })
      return
    }

    const command = parsed.value
    const source = selectSource(command.file)
    if (!source.ok) {
      const detail = describeSourceSelectionProblem(source.error)
      const problem = { kind: 'worker-protocol-failed', detail } as const
      switch (command.kind) {
        case 'profile-source':
          emit({ kind: 'profile-failed', request: command.request, problem })
          return
        case 'profile-column':
          emit({ kind: 'column-profile-failed', request: command.request, problem })
          return
        case 'materialize-numeric':
          emit({ kind: 'materialization-failed', request: command.request, problem })
          return
        case 'materialize-time-series':
          emit({ kind: 'materialization-failed', request: command.request, problem })
          return
        case 'preview-time':
          emit({ kind: 'materialization-failed', request: command.request, problem })
          return
        case 'summarize-columns':
          emit({ kind: 'summary-failed', request: command.request, problem })
          return
        case 'preview-window':
          emit({ kind: 'preview-window-failed', request: command.request, problem })
          return
        case 'inspect-panel':
          emit({ kind: 'panel-data-failed', request: command.request, problem })
          return
        case 'materialize-panel-keys':
          emit({ kind: 'panel-data-failed', request: command.request, problem })
          return
        case 'materialize-panel':
          emit({ kind: 'panel-data-failed', request: command.request, problem })
          return
        default:
          return assertNever(command)
      }
    }

    switch (command.kind) {
      case 'preview-time': {
        const result = await previewTimeColumn(
          source.value,
          command.profile,
          command.timeColumn,
          command.interpretation,
          command.calendar,
        )
        emit(
          result.ok
            ? { kind: 'time-preview-succeeded', request: command.request, preview: result.value }
            : { kind: 'materialization-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'profile-source': {
        const reading = fileReadingOf(source.value.format, command.declared)
        if (!reading.ok) {
          emit({
            kind: 'profile-failed',
            request: command.request,
            problem: {
              kind: 'worker-protocol-failed',
              detail: 'A Parquet file stores its column types, so none can be declared.',
            },
          })
          return
        }
        const result = await profileSource(readAs(source.value, reading.value))
        emit(
          result.ok
            ? { kind: 'profile-succeeded', request: command.request, profile: result.value }
            : { kind: 'profile-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'materialize-numeric': {
        const result = await materializeNumericColumns(
          source.value,
          command.profile,
          command.columnIds,
          command.membership,
        )
        if (!result.ok) {
          emit({ kind: 'materialization-failed', request: command.request, problem: result.error })
          return
        }
        emit(
          { kind: 'materialization-succeeded', request: command.request, matrix: result.value },
          [result.value.values.buffer, result.value.validity.buffer],
        )
        return
      }
      case 'materialize-time-series': {
        const result = await materializeTimeSeriesColumns(
          source.value,
          command.profile,
          command.timeColumn,
          command.columnIds,
          command.interpretation,
        )
        if (!result.ok) {
          emit({ kind: 'materialization-failed', request: command.request, problem: result.error })
          return
        }
        emit(
          {
            kind: 'time-series-materialization-succeeded',
            request: command.request,
            matrix: result.value,
          },
          [
            result.value.timeAxis.kind === 'calendar'
              ? result.value.timeAxis.timestamps.buffer
              : result.value.timeAxis.values.buffer,
            result.value.values.buffer,
            result.value.validity.buffer,
          ],
        )
        return
      }
      case 'profile-column': {
        const result = await profileColumn(source.value, command.profile, command.columnId)
        emit(
          result.ok
            ? { kind: 'column-profile-succeeded', request: command.request, profile: result.value }
            : { kind: 'column-profile-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'summarize-columns': {
        const result = await summarizeColumns(source.value, command.profile, command.categoryColumn)
        emit(
          result.ok
            ? { kind: 'summary-succeeded', request: command.request, summary: result.value }
            : { kind: 'summary-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'preview-window': {
        const result = await previewWindow(source.value, command.profile, command.query)
        emit(
          result.ok
            ? { kind: 'preview-window-succeeded', request: command.request, window: result.value }
            : { kind: 'preview-window-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'inspect-panel': {
        const result = await inspectPanelStructure(
          source.value,
          command.profile,
          command.unitColumn,
          command.timeColumn,
        )
        emit(
          result.ok
            ? {
                kind: 'panel-inspection-succeeded',
                request: command.request,
                structure: result.value,
              }
            : { kind: 'panel-data-failed', request: command.request, problem: result.error },
        )
        return
      }
      case 'materialize-panel': {
        const result = await materializePanelLong(
          source.value,
          command.profile,
          command.unitColumn,
          command.timeColumn,
          command.outcomeColumn,
          command.treatmentColumn,
          command.covariates,
          command.clusterColumn,
        )
        if (!result.ok) {
          emit({ kind: 'panel-data-failed', request: command.request, problem: result.error })
          return
        }
        emit(
          {
            kind: 'panel-materialization-succeeded',
            request: command.request,
            matrix: result.value,
          },
          [result.value.values.buffer],
        )
        return
      }
      case 'materialize-panel-keys': {
        const result = await materializePanelKeys(
          source.value,
          command.profile,
          command.unitColumn,
          command.timeColumn,
        )
        if (!result.ok) {
          emit({ kind: 'panel-data-failed', request: command.request, problem: result.error })
          return
        }
        emit({ kind: 'panel-keys-succeeded', request: command.request, matrix: result.value })
        return
      }
      default:
        return assertNever(command)
    }
  })()
}
