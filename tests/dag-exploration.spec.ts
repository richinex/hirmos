import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { parseBundle } from '../src/domain/bundle'
import { parseSnapshotValue } from '../src/domain/persistence'
import { dagExploration, selectDagExploration } from '../src/domain/dag'
import { createWorkflowStore } from '../src/domain/workflowStore'
import { stepWorkflow } from '../src/domain/workflow'
import { isNonEmpty } from '../src/domain/dop'

function fixture() {
  const result = parseBundle(readFileSync('public/examples/confounded-dose.hirmos.json', 'utf8'))
  if (!result.ok) throw Error(JSON.stringify(result.error))
  return result.value
}

test('exploration rejects invalid endpoints without altering studies or graph revisions', () => {
  const bundle = fixture()
  const document = bundle.project.dagDocuments[0]!
  const nodes = document.current.graph.nodes.filter((n) => n.kind === 'observed')
  const selected = selectDagExploration(document, {
    treatment: nodes[0]!.id,
    outcome: nodes[1]!.id,
  })
  if (!selected.ok) throw Error('Invalid fixture')
  expect(selected.value.current).toBe(document.current)
  expect(selected.value.history).toBe(document.history)
  expect(
    selectDagExploration(document, { treatment: nodes[0]!.id, outcome: nodes[0]!.id }).ok,
  ).toBe(false)
  const remaining = document.current.graph.nodes.filter((n) => n.id !== nodes[0]!.id)
  if (!isNonEmpty(remaining)) throw Error('Fixture needs remaining nodes')
  const stale = {
    ...selected.value,
    current: {
      ...document.current,
      graph: {
        ...document.current.graph,
        nodes: remaining,
      },
    },
  }
  expect(dagExploration(stale)).toEqual({ treatment: null, outcome: nodes[1]!.id })
  expect(parseSnapshotValue({ ...bundle.project, dagDocuments: [stale] }).ok).toBe(false)
  expect(parseSnapshotValue({ ...bundle.project, dagDocuments: [selected.value] }).ok).toBe(true)
})

test('Zustand keeps per-DAG choices in durable workflow, not session drafts', () => {
  const bundle = fixture()
  if (bundle.data.kind !== 'source-file') throw Error('Missing fixture data')
  const workflow = stepWorkflow(
    {
      kind: 'awaiting-data',
      project: bundle.project.project,
      origin: bundle.project.origin,
      problem: null,
      restore: bundle.project,
    },
    {
      type: 'project-restored',
      file: new File([Buffer.from(bundle.data.base64, 'base64')], bundle.data.name),
    },
  )
  if (workflow.kind !== 'profiled') throw Error('Restore failed')
  const store = createWorkflowStore(workflow)
  const document = workflow.dagDocuments[0]!
  const nodes = document.current.graph.nodes.filter((n) => n.kind === 'observed')
  const changed = selectDagExploration(document, { treatment: nodes[0]!.id, outcome: nodes[1]!.id })
  if (!changed.ok) throw Error('Invalid fixture')
  store.getState().dispatch({ type: 'dag-document-revised', document: changed.value })
  const state = store.getState().workflow
  if (state.kind !== 'profiled') throw Error('Wrong state')
  expect(state.dagDocuments[0]!.exploration).toEqual(changed.value.exploration)
  expect(state.studies).toBe(workflow.studies)
  expect(state.studyDraft).toBe(workflow.studyDraft)
})
