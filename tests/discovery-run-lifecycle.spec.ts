import { expect, test } from '@playwright/test'
import {
  resolveDagOrigin,
  type DagDocument,
  type DagDraftRevision,
} from '../src/domain/dag'
import type { DiscoveryRunId } from '../src/domain/discovery'
import { assessDiscoveryRunDeletion, deleteDiscoveryRun } from '../src/domain/discoveryLifecycle'
import { brand } from '../src/domain/dop'

const run = brand<string, 'DiscoveryRunId'>('run-a')
const otherRun = brand<string, 'DiscoveryRunId'>('run-b')
const documentId = brand<string, 'DagDocumentId'>('dag-a')
const documentName = brand<string, 'DagName'>('Road fatalities')
const prepared = brand<string, 'PreparedDatasetVersionId'>('prepared-a')
const cause = brand<string, 'DagNodeId'>('drivers-killed')
const effect = brand<string, 'DagNodeId'>('kms')
const candidate = brand<string, 'DiscoveryCandidateId'>('candidate-a')
const edge = brand<string, 'DagEdgeId'>('edge-a')

const revision = (
  id: string,
  evidenceRun: DiscoveryRunId | null,
): DagDraftRevision => ({
  kind: 'draft',
  id: brand<string, 'DagRevisionId'>(id),
  parent: null,
  createdAt: '2026-09-03T09:00:00.000Z',
  graph: {
    kind: 'editable-dag',
    nodes: [
      { kind: 'observed', id: cause, column: brand<string, 'ColumnId'>('drivers'), name: 'DriversKilled' },
      { kind: 'observed', id: effect, column: brand<string, 'ColumnId'>('kms'), name: 'kms' },
    ],
    edges: evidenceRun === null ? [] : [{
      kind: 'directed',
      id: edge,
      cause,
      effect,
      timing: { kind: 'lagged', lag: 1 },
      support: { kind: 'unstated' },
      evidence: [{ kind: 'discovery', run: evidenceRun, candidate, semantics: 'endpoint-marked' }],
    }],
  },
  validation: evidenceRun === null
    ? { structure: { kind: 'empty' }, rationales: { kind: 'complete' } }
    : { structure: { kind: 'sound' }, rationales: { kind: 'outstanding', edges: [edge] } },
})

const document = (
  origin: DagDocument['origin'],
  audited: readonly [DagDraftRevision, ...DagDraftRevision[]],
): DagDocument => {
  const current = revision('current', null)
  return {
    kind: 'dag-document',
    id: documentId,
    name: documentName,
    preparedDataset: prepared,
    dataset: { kind: 'time-series', observations: 192 },
    origin,
    history: [],
    future: [],
    audit: [...audited, current],
    current,
  }
}

test('DAG origin resolution records only explicitly selected discovery runs', () => {
  const selected = resolveDagOrigin({ kind: 'discovery-informed', reports: [run] }, [run, otherRun])
  expect(selected).toEqual({ ok: true, value: { kind: 'discovery-informed', reports: [run] } })

  const empty = resolveDagOrigin({ kind: 'discovery-informed', reports: [] }, [run])
  expect(empty).toEqual({ ok: false, error: { kind: 'discovery-evidence-required' } })

  const missing = resolveDagOrigin({ kind: 'discovery-informed', reports: [otherRun] }, [run])
  expect(missing).toEqual({ ok: false, error: { kind: 'discovery-evidence-unavailable', run: otherRun } })

  const duplicate = resolveDagOrigin({ kind: 'discovery-informed', reports: [run, run] }, [run])
  expect(duplicate).toEqual({ ok: false, error: { kind: 'duplicate-discovery-evidence', run } })
})

test('an unreferenced discovery run receives deletion permission', () => {
  const decision = assessDiscoveryRunDeletion([run], [], run)
  expect(decision.kind).toBe('deletable')
  if (decision.kind === 'deletable') {
    expect(decision.deletion.run).toBe(run)
    expect(deleteDiscoveryRun([{ id: run }, { id: otherRun }], decision.deletion)).toEqual([{ id: otherRun }])
  }

  expect(assessDiscoveryRunDeletion([run], [], otherRun)).toEqual({ kind: 'not-found', run: otherRun })
})

test('DAG origins and historical edge evidence prevent discovery-run deletion', () => {
  const originUse = assessDiscoveryRunDeletion(
    [run],
    [document({ kind: 'discovery-informed', reports: [run] }, [revision('origin-only', null)])],
    run,
  )
  expect(originUse.kind).toBe('referenced')
  if (originUse.kind === 'referenced') expect(originUse.references[0].kind).toBe('dag-origin-reference')

  const historicalUse = assessDiscoveryRunDeletion(
    [run],
    [document({ kind: 'user-authored', basis: 'domain-knowledge' }, [revision('historical', run)])],
    run,
  )
  expect(historicalUse.kind).toBe('referenced')
  if (historicalUse.kind !== 'referenced') return
  expect(historicalUse.references).toHaveLength(1)
  expect(historicalUse.references[0]).toMatchObject({
    kind: 'edge-evidence-reference',
    revision: 'historical',
    cause: 'DriversKilled',
    effect: 'kms',
    timing: { kind: 'lagged', lag: 1 },
  })
})
