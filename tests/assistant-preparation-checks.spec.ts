import { expect, test } from '@playwright/test'

for (const lossless of [true, false])
  test(`data checks preserve counts with lossless Arrow conversion ${lossless}`, async ({
    page,
  }) => {
    await page.goto('/app')
    const result = await page.evaluate(async (lossless) => {
      const { isolatedDuckDbEngine } = await import(
        new URL('/src/data/duckdb.ts', location.href).href
      )
      const { profileTable } = await import(
        new URL('/src/assistant/preparation.ts', location.href).href
      )
      const engine = await isolatedDuckDbEngine({ arrowLosslessConversion: lossless })
      const c = await engine.db.connect()
      try {
        await c.query(
          "CREATE TABLE samples AS SELECT * FROM (VALUES ('a', 1), ('a', 2), ('a', 2), ('a', 4), ('b', 1), (NULL, 1)) v(unit, period)",
        )
        await c.query(
          "CREATE TABLE lookup AS SELECT * FROM (VALUES ('a'), ('a'), ('c'), (NULL)) v(id)",
        )
        const profile = await profileTable(
          engine.db,
          ['samples', 'lookup'],
          {
            table: 'samples',
            columns: [],
            checks: [
              { kind: 'key', columns: ['unit', 'period'] },
              { kind: 'time', column: 'period', units: ['unit'], step: 1 },
              { kind: 'join', table: 'lookup', left: ['unit'], right: ['id'] },
            ],
          },
          AbortSignal.timeout(15000),
        )
        await c.query('CREATE TABLE empty AS SELECT * FROM samples WHERE false')
        await c.query(
          "CREATE TABLE missing_keys AS SELECT * FROM (VALUES ('a', 1), ('a', 1), (NULL, 1), (NULL, 1), ('a', NULL), ('a', NULL), (NULL, NULL), (NULL, NULL)) v(unit, period)",
        )
        const missing = await profileTable(
          engine.db,
          ['missing_keys'],
          {
            table: 'missing_keys',
            columns: [],
            checks: [{ kind: 'key', columns: ['unit', 'period'] }],
          },
          AbortSignal.timeout(15000),
        )
        const empty = await profileTable(
          engine.db,
          ['empty'],
          {
            table: 'empty',
            columns: [],
            checks: [
              { kind: 'key', columns: ['unit'] },
              { kind: 'time', column: 'period', units: [], step: 1 },
            ],
          },
          AbortSignal.timeout(15000),
        )
        let refused = false
        try {
          await profileTable(
            engine.db,
            ['samples'],
            {
              table: 'samples',
              columns: [],
              checks: [{ kind: 'join', table: 'lookup', left: ['unit'], right: ['id'] }],
            },
            AbortSignal.timeout(15000),
          )
        } catch {
          refused = true
        }
        const rows = Number((await c.query('SELECT count(*) AS n FROM samples')).toArray()[0].n)
        return {
          checks: profile.checks,
          missing: missing.checks,
          empty: empty.checks,
          refused,
          rows,
        }
      } finally {
        await c.close()
        await engine.db.terminate()
      }
    }, lossless)
    expect(result.checks[0]).toMatchObject({
      groups: 4,
      duplicateGroups: 1,
      duplicateRows: 1,
      nullRows: 1,
    })
    expect(result.checks[1]).toMatchObject({
      rows: 6,
      nullRows: 1,
      duplicatePeriods: 1,
      gaps: 1,
      offGridSteps: 0,
      minimumStep: 0,
      maximumStep: 2,
    })
    expect(result.checks[2]).toMatchObject({
      leftRows: 6,
      rightRows: 4,
      innerJoinRows: 8,
      leftJoinRows: 10,
      unmatchedLeftRows: 2,
      unmatchedRightRows: 2,
      leftDuplicateKeys: 1,
      rightDuplicateKeys: 1,
    })
    expect(result.empty[0]).toMatchObject({ groups: 0, duplicateRows: 0, nullRows: 0 })
    expect(result.missing[0]).toMatchObject({
      groups: 1,
      duplicateGroups: 1,
      duplicateRows: 1,
      nullRows: 6,
    })
    expect(result.empty[1]).toMatchObject({
      rows: 0,
      gaps: 0,
      minimumStep: null,
      maximumStep: null,
    })
    expect(result.refused).toBe(true)
    expect(result.rows).toBe(6)
  })

test('aggregate counts reject negatives, malformed byte arrays and unsafe integers', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { preparationCountSchema } = await import(
      new URL('/src/assistant/preparationChecks.ts', location.href).href
    )
    const bytes = (value: bigint) => {
      const data = new Uint8Array(16)
      const view = new DataView(data.buffer)
      view.setBigUint64(0, BigInt.asUintN(64, value), true)
      view.setBigInt64(8, value >> 64n, true)
      return data
    }
    return {
      counts: [0, 1n, bytes(42n), bytes(BigInt(Number.MAX_SAFE_INTEGER))].map((v) =>
        preparationCountSchema.parse(v),
      ),
      rejected: [-1, -1n, 1.5, bytes(-1n), bytes(9007199254740992n), new Uint8Array(8)].map(
        (v) => !preparationCountSchema.safeParse(v).success,
      ),
    }
  })
  expect(result.counts).toEqual([0, 1, 42, Number.MAX_SAFE_INTEGER])
  expect(result.rejected).toEqual(Array(6).fill(true))
})
