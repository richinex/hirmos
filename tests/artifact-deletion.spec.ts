import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { parseBundle } from '../src/domain/bundle'
import { stepWorkflow, type Workflow } from '../src/domain/workflow'
import { assessArtifactDeletion } from '../src/domain/artifactLifecycle'
import { parseSnapshot, serialiseSnapshot, snapshotWorkflow } from '../src/domain/persistence'
import { brand } from '../src/domain/dop'

function fixture(name: string): Extract<Workflow, { kind: 'profiled' }> {
  const parsed = parseBundle(
    readFileSync(new URL('../public/examples/' + name + '.hirmos.json', import.meta.url), 'utf8'),
  )
  if (!parsed.ok) throw Error(JSON.stringify(parsed.error))
  const bundle = parsed.value
  const data = bundle.data
  if (data.kind !== 'source-file') throw Error('Source required')
  const restored = stepWorkflow(
    {
      kind: 'awaiting-data',
      project: bundle.project.project,
      origin: bundle.project.origin,
      problem: null,
      restore: bundle.project,
    },
    {
      type: 'project-restored',
      file: new File([Buffer.from(data.base64, 'base64')], data.name, { type: data.mediaType }),
    },
  )
  if (restored.kind !== 'profiled') throw Error('Restore failed')
  return restored
}

test('unreferenced DAG deletion clears selections, preserves data, and survives serialization', () => {
  const state = { ...fixture('confounded-dose'), interventionQueries: [] }
  const document = state.dagDocuments[0]!
  const selected = {
    ...state,
    studyDraft: { ...state.studyDraft, dagDocument: document.id },
    rootCause: {
      ...state.rootCause,
      selection: {
        dagDocument: document.id,
        dagRevision: document.current.id,
        preparedDataset: document.preparedDataset,
      },
    },
  }
  const target = { kind: 'dag', id: document.id } as const
  expect(assessArtifactDeletion(selected, target).kind).toBe('ready')
  const next = stepWorkflow(selected, { type: 'artifact-deletion-requested', target })
  if (next.kind !== 'profiled') throw Error('Wrong state')
  expect(next.dagDocuments).toEqual([])
  expect(next.studyDraft.dagDocument).toBeNull()
  expect(next.rootCause.selection).toBeNull()
  expect(next.prepared).toBe(state.prepared)
  expect(next.discoveryRuns).toBe(state.discoveryRuns)
  const snapshot = snapshotWorkflow(next, '2026-10-05T12:00:00.000Z')
  if (snapshot === null) throw Error('Missing snapshot')
  const parsed = parseSnapshot(serialiseSnapshot(snapshot))
  expect(parsed.ok).toBe(true)
  if (parsed.ok) expect(parsed.value.dagDocuments).toEqual([])
  expect(stepWorkflow(next, { type: 'artifact-deletion-requested', target })).toBe(next)
})

test('study and check references block DAG deletion, including an older revision', () => {
  const state = fixture('lalonde')
  const document = state.dagDocuments[0]!
  const target = { kind: 'dag', id: document.id } as const
  const nextRevision = { ...document.current, id: brand<string, 'DagRevisionId'>('later-revision') }
  const revised: Extract<Workflow, { kind: 'profiled' }> = {
    ...state,
    dagDocuments: [
      { ...document, current: nextRevision, audit: [...document.audit, nextRevision] },
    ],
  }
  const decision = assessArtifactDeletion(revised, target)
  expect(decision.kind).toBe('blocked')
  if (decision.kind !== 'blocked') return
  expect(decision.references.some((item) => item.location === 'Study design, Studies')).toBe(true)
  expect(decision.references.some((item) => item.removable?.kind === 'dag-check')).toBe(true)
  expect(stepWorkflow(revised, { type: 'artifact-deletion-requested', target })).toBe(revised)
})

test('saved effects block study deletion; removing the study removes its identification only', () => {
  const state = fixture('lalonde')
  const study = state.studies[0]!
  const target = { kind: 'study', id: study.id } as const
  expect(stepWorkflow(state, { type: 'artifact-deletion-requested', target })).toBe(state)
  const withoutRuns = { ...state, estimationRuns: [], sensitivityRuns: [], counterfactualRuns: [] }
  expect(assessArtifactDeletion(withoutRuns, target).kind).toBe('ready')
  // A stale ready decision cannot authorize deletion after a result has arrived.
  expect(stepWorkflow(state, { type: 'artifact-deletion-requested', target })).toBe(state)
  const next = stepWorkflow(withoutRuns, { type: 'artifact-deletion-requested', target })
  if (next.kind !== 'profiled') throw Error('Wrong state')
  expect(next.studies).toEqual([])
  expect(next.identifications).toEqual([])
  expect(next.dagDocuments).toBe(state.dagDocuments)
  expect(next.dagChecks).toBe(state.dagChecks)
  const lateEstimate = state.estimationRuns[0]!
  expect(stepWorkflow(next, { type: 'estimation-run-created', run: lateEstimate })).toBe(next)
})

test('late identification cannot resurrect a deleted DAG reference', () => {
  const state = fixture('lalonde')
  const bare = {
    ...state,
    studies: [],
    identifications: [],
    estimationRuns: [],
    sensitivityRuns: [],
    counterfactualRuns: [],
    dagChecks: [],
  }
  const target = { kind: 'dag', id: state.dagDocuments[0]!.id } as const
  const next = stepWorkflow(bare, { type: 'artifact-deletion-requested', target })
  expect(
    stepWorkflow(next, {
      type: 'study-identified',
      study: state.studies[0]!,
      identification: state.identifications[0]!,
    }),
  ).toBe(next)
  expect(stepWorkflow(next, { type: 'dag-check-created', check: state.dagChecks[0]! })).toBe(next)
})

test('checks have explicit non-cascading deletion and model results protect their DAG', () => {
  const state = fixture('lalonde')
  const next = stepWorkflow(state, {
    type: 'artifact-deletion-requested',
    target: { kind: 'dag-check', id: state.dagChecks[0]!.id },
  })
  if (next.kind !== 'profiled') throw Error('Wrong state')
  expect(next.dagChecks).toEqual([])
  expect(next.studies).toBe(state.studies)
  const model = fixture('microservices-rca')
  expect(assessArtifactDeletion(model, { kind: 'dag', id: model.dagDocuments[0]!.id }).kind).toBe(
    'blocked',
  )
})
