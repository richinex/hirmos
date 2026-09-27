import { expect, test } from '@playwright/test'

test('Script Arrow transport preserves typed values and refuses lossy results', async ({ page }) => {
  test.setTimeout(180_000)
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const { isolatedDuckDbEngine } = await import(new URL('/src/data/duckdb.ts', location.href).href) as typeof import('../src/data/duckdb')
    const { createPythonRuntime } = await import(new URL('/src/data/pythonRuntime.ts', location.href).href) as typeof import('../src/data/pythonRuntime')
    const { pipelineBlockId } = await import(new URL('/src/domain/pipeline.ts', location.href).href) as typeof import('../src/domain/pipeline')
    const { db } = await isolatedDuckDbEngine()
    const connection = await db.connect()
    const session = createPythonRuntime()
    const runtime = session.scripts
    const findings: { name: string; error?: string; different?: number; schemaEqual?: boolean }[] = []
    const cases: Record<string, string> = {
      booleans: 'SELECT * FROM (VALUES (true),(false),(NULL::BOOLEAN))t(x)',
      integers: 'SELECT 9007199254740993::BIGINT a, 18446744073709551615::UBIGINT b, NULL::INTEGER c',
      decimal: 'SELECT 12345678901234567890.123456789::DECIMAL(38,9) x',
      dates: "SELECT DATE '2024-02-29' d, TIMESTAMP '2024-02-29 12:13:14.123456' t, TIMESTAMPTZ '2024-02-29 12:13:14.123456+03' z",
      nanoseconds: "SELECT '2024-01-01 00:00:00.123456789'::TIMESTAMP_NS x",
      time: "SELECT TIME '12:13:14.123456' x",
      strings: "SELECT * FROM (VALUES ('00001'),('NA'),(''),('a,\"b\"'),('é中文'),(NULL::VARCHAR))t(x)",
      nulls: 'SELECT NULL::VARCHAR s, NULL::DOUBLE d, NULL::BOOLEAN b',
      specialFloats: "SELECT * FROM (VALUES ('NaN'::DOUBLE),('Infinity'::DOUBLE),('-Infinity'::DOUBLE),(NULL::DOUBLE))t(x)",
      binary: "SELECT '\\x00\\xFF'::BLOB x",
      nested: "SELECT [1,NULL,3] x, {'name':'a','flag':true} y",
      interval: "SELECT INTERVAL '1 month 2 days 3 microseconds' x",
      hugeint: "SELECT 170141183460469231731687303715884105727::HUGEINT x, '-170141183460469231731687303715884105728'::HUGEINT y",
      uhugeint: 'SELECT 340282366920938463463374607431768211455::UHUGEINT x',
      uuid: "SELECT '550e8400-e29b-41d4-a716-446655440000'::UUID x",
      empty: 'SELECT 1::BIGINT x, NULL::VARCHAR y WHERE false',
      duplicates: 'SELECT * FROM (VALUES (1),(1),(2))t(x)',
    }
    const run = (code: string, inputs = ['script_input']) => runtime.run({ kind: 'script', id: pipelineBlockId('arrow-test'), view: 'script_output', inputs, code }, connection, db)
    const queryRows = async (sql: string) => (await connection.query(sql)).toArray().map(row => row.toJSON())
    try {
      for (const [name, sql] of Object.entries(cases)) {
        await connection.query(`CREATE OR REPLACE TABLE script_input AS ${sql}`)
        const result = await run('prepared = inputs[0]')
        if (!result.ok) { findings.push({ name, error: result.error.detail }); continue }
        const different = Number((await queryRows('SELECT count(*) n FROM ((SELECT * FROM script_input EXCEPT ALL SELECT * FROM script_output) UNION ALL (SELECT * FROM script_output EXCEPT ALL SELECT * FROM script_input))'))[0]!.n)
        const schema = (await queryRows('DESCRIBE script_output')).map(row => [row.column_name, row.column_type])
        const expected = (await queryRows('DESCRIBE script_input')).map(row => [row.column_name, row.column_type])
        findings.push({ name, different, schemaEqual: JSON.stringify(schema) === JSON.stringify(expected) })
      }
      await connection.query('CREATE OR REPLACE TABLE script_input AS SELECT 2::BIGINT count, 7::BIGINT id')
      const modulo = await run('df=inputs[0]\nassert str(df["count"].dtype)=="Int64"\nassert (df["count"] % 1 == 0).all()\nprepared=df.loc[df.index.repeat(df["count"])].reset_index(drop=True)')
      findings.push({ name: 'modulo', ...(modulo.ok ? { different: Number((await queryRows('SELECT abs(count(*)-2) n FROM script_output'))[0]!.n) } : { error: modulo.error.detail }) })
      const failures: Record<string, string> = {
        mixed: 'prepared=pd.DataFrame({"mixed":[1,"hello"]})',
        duplicateNames: 'prepared=pd.DataFrame([[1,2]],columns=["x","X"])',
        invalidOutput: 'prepared=[1, 2, 3]',
        missingOutput: 'value=42',
        tooPrecise: 'import pyarrow as pa\nfrom decimal import Decimal\nprepared=pd.DataFrame({"amount":pd.Series([Decimal("1")],dtype=pd.ArrowDtype(pa.decimal256(50,0)))})',
        zonedNanoseconds: 'import pyarrow as pa\nprepared=pd.DataFrame({"time":pd.Series([1],dtype=pd.ArrowDtype(pa.timestamp("ns",tz="UTC")))})',
        intervalNanoseconds: 'import pyarrow as pa\nprepared=pd.DataFrame({"elapsed":pd.Series([(0,0,1)],dtype=pd.ArrowDtype(pa.month_day_nano_interval()))})',
      }
      for (const [name, code] of Object.entries(failures)) {
        const result = await run(code)
        findings.push({ name, error: result.ok ? 'UNEXPECTED SUCCESS' : result.error.detail })
        // Every failed rerun leaves the previous successful two-row result intact.
        findings.push({ name: name + '-retained', different: Number((await queryRows('SELECT abs(count(*)-2) n FROM script_output'))[0]!.n) })
      }
      const changed = await run('prepared=pd.DataFrame({"replacement":["ok"]})')
      findings.push({ name: 'changed-columns', ...(changed.ok ? { different: (await queryRows('SELECT replacement FROM script_output'))[0]!.replacement === 'ok' ? 0 : 1 } : { error: changed.error.detail }) })
      return findings
    } finally { session.dispose(); await connection.close(); await db.terminate() }
  })
  await test.info().attach('typed-round-trips', { body: JSON.stringify(results, null, 2), contentType: 'application/json' })
  for (const result of results) {
    if (['mixed', 'duplicateNames', 'invalidOutput', 'missingOutput', 'tooPrecise', 'zonedNanoseconds', 'intervalNanoseconds'].includes(result.name)) {
      expect(result.error, result.name).toBeTruthy()
      expect(result.error, result.name).not.toBe('UNEXPECTED SUCCESS')
      continue
    }
    expect.soft(result.error, result.name).toBeUndefined()
    expect.soft(result.different, result.name).toBe(0)
    if (result.schemaEqual !== undefined) expect.soft(result.schemaEqual, result.name).toBe(true)
  }
})
