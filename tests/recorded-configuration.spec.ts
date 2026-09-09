import { expect, test } from '@playwright/test'
import { brand } from '../src/domain/dop'
import { withRecordedConfiguration, type DiscoveryRunArtifact } from '../src/domain/discovery'

// A bundle replays a chapter from the configuration its runs carry. Runs saved before they carried
// one are completed from what the method reported, so an old bundle still reopens on its settings.

const column = (id: string, name: string) => ({ id: brand<string, 'ColumnId'>(id), name })
const identity = { id: brand<string, 'DiscoveryRunId'>('run'), preparedDataset: brand<string, 'PreparedDatasetVersionId'>('prepared'), createdAt: '2026-09-09T00:00:00.000Z' }

test('a PCMCI+ run saved without its configuration reopens on the lag and alpha it reported', () => {
  const saved = { ...identity, kind: 'pcmci-plus-run', method: 'pcmci-plus-parcorr', variables: [column('a', 'A')], eligibility: { kind: 'eligible' }, result: { tauMax: 4, pcAlpha: 0.01 } } as unknown as DiscoveryRunArtifact
  const restored = withRecordedConfiguration(saved)
  expect(restored.configuration).toEqual({ kind: 'pcmci-plus', tauMax: 4, pcAlpha: 0.01 })
})

test('a J-PCMCI+ run saved without its configuration reopens on the roles its nodes record', () => {
  const saved = {
    ...identity, kind: 'jpcmci-plus-run', method: 'jpcmciplus-parcorr-mult', variables: [column('x', 'X'), column('c4', 'C4'), column('c5', 'C5')], eligibility: { kind: 'eligible' },
    nodes: [
      { kind: 'observed', column: column('x', 'X'), role: 'system' },
      { kind: 'observed', column: column('c4', 'C4'), role: 'timeContext' },
      { kind: 'observed', column: column('c5', 'C5'), role: 'spaceContext' },
      { kind: 'generated', role: 'timeDummy', name: 'Time context (generated)' },
    ],
    result: { tauMax: 2, pcAlpha: 0.05 },
  } as unknown as DiscoveryRunArtifact
  const restored = withRecordedConfiguration(saved)
  expect(restored.configuration).toEqual({
    kind: 'jpcmci-plus', tauMax: 2, pcAlpha: 0.05,
    assignments: [{ column: 'c4', role: 'timeContext' }, { column: 'c5', role: 'spaceContext' }],
    timeDummy: true, spaceDummy: false,
  })
})

test('a lag the controls do not offer falls back to the default rather than an impossible choice', () => {
  const saved = { ...identity, kind: 'cdnots-run', method: 'cdnots-parcorr', variables: [column('a', 'A')], eligibility: { kind: 'eligible' }, result: { maxLag: 5, alpha: 0.05 } } as unknown as DiscoveryRunArtifact
  expect(withRecordedConfiguration(saved).configuration).toEqual({ kind: 'cdnots', maxLag: 2, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear' })
})

test('a run that carries its configuration is returned as it is', () => {
  const configuration = { kind: 'lpcmci', tauMax: 3, pcAlpha: 0.1 } as const
  const saved = { ...identity, kind: 'lpcmci-run', configuration, method: 'lpcmci-parcorr', variables: [column('a', 'A')], eligibility: { kind: 'eligible' }, result: { tauMax: 1, pcAlpha: 0.01 } } as unknown as DiscoveryRunArtifact
  expect(withRecordedConfiguration(saved)).toBe(saved)
})
