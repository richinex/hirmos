import { useState } from 'react'
import { createRoot } from 'react-dom/client'
import { useStore } from 'zustand'
import { WorkflowProvider } from '../../src/components/WorkflowProvider'
import { PreparationProvider, usePreparationSession } from '../../src/components/data/PreparationProvider'
import { SqlShell } from '../../src/components/data/SqlPreparationWorkspace'
import { prepareSqlInputs } from '../../src/data/sqlPreparation'
import type { SqlPreparationSession } from '../../src/data/sqlPreparation'
import type { SqlController } from '../../src/components/data/sqlSession'
import { RecordBatchReader, Table } from 'apache-arrow'
import { openSqlPreparation, closeSqlPreparation } from '../../src/data/sqlPreparation'

export async function queryParity() {
  const inputs = await prepareSqlInputs([new File(['x\n1\n2\n'], 'input.csv', { type: 'text/csv' })])
  if (!inputs.ok) throw new Error('Input preparation failed')
  const opened = await openSqlPreparation(inputs.value)
  if (!opened.ok) throw new Error('Database preparation failed')
  const session = opened.value
  const connection = await session.database.connectInternal()
  const statements = [
    'SELECT 9007199254740993::BIGINT AS n, 1.234567890::DECIMAL(20,9) AS d, NULL::BOOLEAN AS b',
    "SELECT DATE '2024-01-02' AS day, TIMESTAMP '2024-01-02 03:04:05.123456' AS time, [1,2,NULL] AS list, {'a': 4} AS struct",
    'SELECT x::BIGINT AS n, NULL::DECIMAL(20,9) AS d FROM input WHERE false',
    'CREATE OR REPLACE VIEW parity_view AS SELECT * FROM input',
    'SELECT * FROM parity_view',
    "SELECT 'é;λ' AS text; -- a trailing ; comment",
    "SELECT ';' AS text /* ; */",
  ]
  const describe = (bytes: Uint8Array) => {
    const reader = RecordBatchReader.from(bytes)
    const table = new Table(reader)
    return { schema: table.schema.toString(), rows: Array.from(table, row => row.toString()) }
  }
  try {
    const comparisons = []
    for (const sql of statements) {
      comparisons.push({ sql, direct: describe(await session.database.runQuery(connection, sql)), pending: describe(await session.shellDatabase.runQuery(connection, sql)) })
    }
    return comparisons
  } finally { await closeSqlPreparation(session) }
}

export async function batchBehavior() {
  const inputs = await prepareSqlInputs([new File(['x\n1\n'], 'input.csv', { type: 'text/csv' })])
  if (!inputs.ok) throw new Error('Input preparation failed')
  const opened = await openSqlPreparation(inputs.value)
  if (!opened.ok) throw new Error('Database preparation failed')
  const session = opened.value
  const connection = await session.database.connectInternal()
  const run = async (sql: string) => {
    const bytes = await session.shellDatabase.runQuery(connection, sql)
    return Array.from(new Table(RecordBatchReader.from(bytes)), row => row.toString())
  }
  try {
    const last = await run('SELECT 1 AS first; SELECT 2 AS last')
    const rollback = await run('CREATE TEMP TABLE changes (n INTEGER); BEGIN; INSERT INTO changes VALUES (1); ROLLBACK; SELECT count(*)::INTEGER AS n FROM changes')
    const commit = await run('BEGIN; INSERT INTO changes VALUES (1); COMMIT; SELECT count(*)::INTEGER AS n FROM changes')
    let rejected = false
    try { await run('BEGIN; INSERT INTO changes VALUES (2); SELECT missing_column FROM changes; COMMIT') }
    catch { rejected = true }
    const recovered = await run('ROLLBACK; SELECT count(*)::INTEGER AS n FROM changes')
    return { last, rollback, commit, rejected, recovered }
  } finally { await closeSqlPreparation(session) }
}

export async function mount() {
  const inputs = await prepareSqlInputs([new File(['x\n1\n2\n3\n'], 'input.csv', { type: 'text/csv' })])
  if (!inputs.ok) throw new Error('Input preparation failed')
  const files = inputs.value
  let live: SqlPreparationSession | null = null
  let controller: SqlController | null = null
  let openings = 0
  function Status({ current }: { current: SqlController }) {
    const { shell } = useStore(current.store)
    if ('session' in shell && shell.session !== live) { live = shell.session; openings++ }
    return <output data-testid="sql-session-state">{JSON.stringify({ openings, shell: shell.kind, connection: live?.shellConnection.current })}</output>
  }
  function Editor() {
    const owner = usePreparationSession()
    const entry = useStore(owner.store, state => state.entry)
    const [visible, show] = useState(true)
    if (entry.kind === 'sql') controller = entry.controller
    return <>
      <button onClick={() => show(!visible)}>{visible ? 'Hide test editor' : 'Show test editor'}</button>
      {entry.kind === 'sql' && <Status current={entry.controller} />}
      {visible && <SqlShell inputs={files} resume={null} onPrepared={() => {}} onCleared={owner.clear} />}
    </>
  }
  function Harness() {
    const [opened, setOpened] = useState(true)
    const [closed, report] = useState('')
    return <>
      <button onClick={() => setOpened(false)}>Close test project</button>
      {opened ? <WorkflowProvider><PreparationProvider><Editor /></PreparationProvider></WorkflowProvider>
        : <button onClick={() => report(JSON.stringify({ lifecycle: live?.lifecycle.current, active: controller?.active() }))}>Inspect closed session</button>}
      <output data-testid="closed-sql-session">{closed}</output>
    </>
  }
  const host = document.createElement('div')
  host.id = 'sql-session-test'
  host.style.cssText = 'position:fixed;inset:0;z-index:99999;background:var(--color-stage);overflow:auto;padding:16px'
  document.body.append(host)
  createRoot(host).render(<Harness />)
}
