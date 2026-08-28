/// <reference lib="webworker" />

import { materializeNumericColumns, profileSource } from '@/data/duckdb'
import { assertNever } from '@/domain/dop'
import { describeSourceSelectionProblem, selectSource } from '@/domain/workflow'
import { parseDataWorkerCommand, type DataWorkerEvent } from './dataProtocol'

const emit = (event: DataWorkerEvent, transfer: Transferable[] = []) => self.postMessage(event, { transfer })

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
      emit(command.kind === 'profile-source'
        ? { kind: 'profile-failed', request: command.request, problem: { kind: 'worker-protocol-failed', detail } }
        : { kind: 'materialization-failed', request: command.request, problem: { kind: 'worker-protocol-failed', detail } })
      return
    }

    switch (command.kind) {
      case 'profile-source': {
        const result = await profileSource(source.value)
        emit(result.ok
          ? { kind: 'profile-succeeded', request: command.request, profile: result.value }
          : { kind: 'profile-failed', request: command.request, problem: result.error })
        return
      }
      case 'materialize-numeric': {
        const result = await materializeNumericColumns(source.value, command.profile, command.columnIds)
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
      default:
        return assertNever(command)
    }
  })()
}
