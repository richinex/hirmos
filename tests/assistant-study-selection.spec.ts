import { expect, test } from '@playwright/test'
import type { StudySpecification, IdentificationArtifact } from '../src/domain/study'

test('study selection requires an available choice and refuses changed capabilities', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { proposeStudySelection, applyStudySelection } = await import(
      new URL('/src/assistant/studySelection.tsx', location.href).href
    )
    let selections = 0
    const capability = {
      revision: 'initial',
      inspection: {
        selected: 'old',
        choices: [
          {
            identification: 'new',
            study: 'law-study',
            question: 'Law on fatalities',
            treatment: 'law',
            outcome: 'DriversKilled',
            adjustment: 'kms, PetrolPrice',
          },
        ],
      },
      select: () => {
        selections++
        return true
      },
    }
    const proposal = proposeStudySelection(capability, {
      identification: 'new',
      reason: 'Select the intended study.',
    })
    if (!proposal.ok) throw new Error(proposal.error)
    const before = selections
    const stale = applyStudySelection({ ...capability, revision: 'changed' }, proposal.value).ok
    const missing = applyStudySelection(null, proposal.value).ok
    const removed = applyStudySelection(
      { ...capability, inspection: { selected: 'old', choices: [] } },
      proposal.value,
    ).ok
    const afterRefusals = selections
    const applied = applyStudySelection(capability, proposal.value).ok
    const unknown = proposeStudySelection(capability, {
      identification: 'missing',
      reason: 'Select',
    }).ok
    return { before, stale, missing, removed, afterRefusals, applied, selections, unknown }
  })
  expect(result).toEqual({
    before: 0,
    stale: false,
    missing: false,
    removed: false,
    afterRefusals: 0,
    applied: true,
    selections: 1,
    unknown: false,
  })
})

test('O-set provenance survives project serialization without relabelling supplied sets', async ({
  page,
}) => {
  await page.goto('/app')
  const kinds = await page.evaluate(async () => {
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const { identificationFrom } = await import(new URL('/src/domain/study.ts', location.href).href)
    const { studyAdjustmentDesign } = await import(
      new URL('/src/domain/studyAdjustment.ts', location.href).href
    )
    const { validateAdjustmentSets } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const parsed = persistence.parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error('Invalid fixture')
    const workflow = { ...parsed.value, kind: 'profiled' }
    const record = workflow.identifications[0]
    const study = workflow.studies.find((item: StudySpecification) => item.id === record.study)
    const checked = await validateAdjustmentSets(studyAdjustmentDesign(study))
    if (
      !checked.ok ||
      checked.value.analysis.kind !== 'identified' ||
      checked.value.analysis.recommendation.kind !== 'available'
    )
      throw new Error('No recommendation')
    const recommendation = checked.value.analysis.recommendation
    return ['o-set', 'supplied'].map((kind) => {
      const result = identificationFrom(study, record.evidence, {
        kind,
        nodes: recommendation.nodes,
        guarantee: recommendation.guarantee,
      })
      if (!result.ok) throw new Error('Identification failed')
      const snapshot = persistence.snapshotWorkflow(
        {
          ...workflow,
          identifications: workflow.identifications.map((item: IdentificationArtifact) =>
            item.id === record.id ? { ...item, result: result.value } : item,
          ),
        },
        new Date().toISOString(),
      )
      const restored = persistence.parseSnapshot(persistence.serialiseSnapshot(snapshot))
      if (!restored.ok) throw new Error(JSON.stringify(restored.error))
      return restored.value.identifications[0].result.adjustment.kind
    })
  })
  expect(kinds).toEqual(['o-set', 'supplied'])
})

test('recording an identical study is idempotent and summaries expose adjustment meaning', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const { stepWorkflow } = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const { existingStudyRecord } = await import(
      new URL('/src/domain/studyRecords.ts', location.href).href
    )
    const { assistantContext } = await import(
      new URL('/src/assistant/context.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw new Error('Invalid fixture')
    const original = parsed.value.identifications[0]
    const study = parsed.value.studies.find(
      (item: StudySpecification) => item.id === original.study,
    )
    const record = {
      ...original,
      result: {
        ...original.result,
        adjustment: {
          kind: 'o-set',
          variables: original.result.adjustment.variables,
          guarantee: 'established',
        },
      },
    }
    const state = { ...parsed.value, kind: 'profiled', identifications: [record] }
    const nextStudy = { ...study, id: 'new-study', createdAt: 'new-time' }
    const nextRecord = {
      ...record,
      id: 'new-identification',
      study: nextStudy.id,
      createdAt: 'new-time',
    }
    const after = stepWorkflow(state, {
      type: 'study-identified',
      study: nextStudy,
      identification: nextRecord,
    })
    const reworded = stepWorkflow(state, {
      type: 'study-identified',
      study: nextStudy,
      identification: {
        ...nextRecord,
        result: {
          ...nextRecord.result,
          basis: [
            { kind: 'graph-result', id: 'backdoor-criterion', statement: 'Reworded explanation.' },
          ],
        },
      },
    })
    const changedChoice = {
      ...nextRecord,
      result: {
        ...nextRecord.result,
        adjustment: { ...nextRecord.result.adjustment, kind: 'supplied' },
      },
    }
    const changed = stepWorkflow(state, {
      type: 'study-identified',
      study: nextStudy,
      identification: changedChoice,
    })
    const changedSpecifications = [
      { ...nextStudy, dagRevision: 'other-revision' },
      { ...nextStudy, preparedDataset: 'other-data' },
      {
        ...nextStudy,
        estimand: {
          kind: 'average-treatment-effect-on-treated',
          scale: 'additive',
          treatedValue: 1,
        },
      },
      {
        ...nextStudy,
        assignment: { ...nextStudy.assignment, description: 'Different assignment' },
      },
    ].map((candidate) =>
      Boolean(existingStudyRecord(state.studies, state.identifications, candidate, nextRecord)),
    )
    const summary = JSON.parse(assistantContext(state)).studies.find(
      (item: { id: string }) => item.id === study.id,
    ).adjustment
    const notGuaranteed = JSON.parse(
      assistantContext({
        ...state,
        identifications: [
          {
            ...record,
            result: {
              ...record.result,
              adjustment: { ...record.result.adjustment, guarantee: 'notEstablished' },
            },
          },
        ],
      }),
    ).studies.find((item: { id: string }) => item.id === study.id).adjustment
    return {
      unchanged: after === state,
      rewordingUnchanged: reworded === state,
      runsUnchanged: after.estimationRuns === state.estimationRuns,
      changedChoiceAdded: changed.studies.length === state.studies.length + 1,
      changedSpecifications,
      summary,
      notGuaranteed,
    }
  })
  expect(result.unchanged).toBe(true)
  expect(result.rewordingUnchanged).toBe(true)
  expect(result.runsUnchanged).toBe(true)
  expect(result.changedChoiceAdded).toBe(true)
  expect(result.changedSpecifications).toEqual([false, false, false, false])
  expect(result.summary.kind).toBe('o-set')
  expect(result.summary.guarantee).toBe('established')
  expect(result.summary.meaning).toContain('lowest asymptotic variance')
  expect(result.summary.variables.length).toBeGreaterThan(0)
  expect(result.notGuaranteed.meaning).toContain('not established')
})
