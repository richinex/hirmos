import { expect, test } from '@playwright/test'

for (const workspace of ['discovery', 'counterfactual'] as const) {
  test(`${workspace}: saved evidence and graph provenance survive changes to the current workspace`, async ({
    page,
  }) => {
    await page.goto('/app')
    const result = await page.evaluate(async (workspace) => {
      const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
      const { parseSnapshotValue } = await import(
        new URL('/src/domain/persistence.ts', location.href).href
      )
      const { assistantContext } = await import(
        new URL('/src/assistant/context.ts', location.href).href
      )
      const bundle = await (await fetch('/examples/seatbelts.hirmos.json')).json()
      const parsed = parseSnapshotValue(bundle.project)
      if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
      const workflow = { ...parsed.value, kind: 'profiled' }
      const run = (
        workspace === 'discovery' ? workflow.discoveryRuns : workflow.counterfactualRuns
      )[0]
      if (!run) throw new Error(`Example has no ${workspace} run`)
      const before = JSON.stringify(workflow)
      const evidence = readRun(workflow, { workspace, id: run.id, view: 'evidence' })
      const specification = readRun(workflow, { workspace, id: run.id, view: 'specification' })
      const index = JSON.parse(assistantContext(workflow))[
        workspace === 'discovery' ? 'savedDiscoveryRuns' : 'savedCounterfactualRuns'
      ]
      const unchanged = before === JSON.stringify(workflow)
      if (workspace === 'counterfactual') {
        const study = workflow.studies.find((s: { id: string }) => s.id === run.study)
        const document = workflow.dagDocuments.find(
          (d: { id: string }) => d.id === study.dagDocument,
        )
        document.current = {
          ...document.current,
          id: 'new-revision',
          graph: { nodes: [], edges: [] },
        }
      }
      workflow.prepared = { ...workflow.prepared, id: 'different-data' }
      const changed = readRun(workflow, { workspace, id: run.id, view: 'specification' })
      workflow.studies = []
      return {
        evidence,
        specification,
        changed,
        unchanged,
        index,
        expected: workspace === 'discovery' ? run.result : run.evidence,
        missing: readRun(workflow, { workspace, id: run.id }),
        wrong: readRun(workflow, {
          workspace: workspace === 'discovery' ? 'counterfactual' : 'discovery',
          id: run.id,
        }),
        unknown: readRun(workflow, { workspace, id: 'unknown' }),
      }
    }, workspace)
    expect(result.evidence.kind).toBe('saved-run')
    expect(result.evidence.evidence).toEqual(result.expected)
    expect(result.unchanged).toBe(true)
    expect(result.index.some((r: { id: string }) => r.id === result.evidence.run)).toBe(true)
    expect(result.changed.preparedDatasetRecord).toBeNull()
    expect(result.changed.provenance.preparedDataset).toBe('unavailable')
    expect(result.wrong.kind).toBe('refused')
    expect(result.unknown.kind).toBe('refused')
    if (workspace === 'counterfactual') {
      expect(result.changed.provenance.graph).toBe('earlier')
      expect(result.changed.graphRevision).toEqual(result.specification.graphRevision)
      expect(result.missing.subject).toBeNull()
      expect(result.missing.provenance.graph).toBe('missing-study')
    }
  })
}
