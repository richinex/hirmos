import { z } from 'zod'
import type { AsyncDuckDBConnection } from '@duckdb/duckdb-wasm'
import { isNumericDuckDbType } from '@/domain/dataset'
import { wideInteger } from '@/data/duckdb'

const names = z.array(z.string().min(1)).min(1).max(8)
export const preparationCheckSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('key'), columns: names }).strict(),
  z
    .object({
      kind: z.literal('time'),
      column: z.string().min(1),
      units: z.array(z.string().min(1)).max(8),
      step: z.number().positive().finite(),
    })
    .strict(),
  z
    .object({ kind: z.literal('join'), table: z.string().min(1), left: names, right: names })
    .strict(),
])
type Check = z.infer<typeof preparationCheckSchema>
const identifier = (name: string) => `"${name.replaceAll('"', '""')}"`
export const preparationCountSchema = z.preprocess(
  (value) =>
    ArrayBuffer.isView(value) && value.byteLength === 16 ? wideInteger(value, true) : value,
  z.union([z.number(), z.bigint().transform(Number)]).pipe(z.number().int().nonnegative().safe()),
)
const count = preparationCountSchema
const keyStats = z.object({
  groups: count,
  duplicateGroups: count,
  duplicateRows: count,
  nullRows: count,
})
const timeStats = z.object({
  rows: count,
  nullRows: count,
  duplicatePeriods: count,
  gaps: count,
  offGridSteps: count,
  minimumStep: z.number().nullable(),
  maximumStep: z.number().nullable(),
})
const joinStats = z.object({
  leftRows: count,
  rightRows: count,
  innerJoinRows: count,
  leftJoinRows: count,
  unmatchedLeftRows: count,
  unmatchedRightRows: count,
  leftDuplicateKeys: count,
  rightDuplicateKeys: count,
})

export async function checkPreparation(
  connection: AsyncDuckDBConnection,
  tables: readonly string[],
  source: string,
  checks: readonly Check[],
  signal: AbortSignal,
) {
  const columns = async (table: string) => {
    if (!tables.includes(table))
      throw new Error('The check refers to a table outside this workspace.')
    return (await connection.query(`DESCRIBE SELECT * FROM ${identifier(table)}`))
      .toArray()
      .map((row) => z.object({ column_name: z.string(), column_type: z.string() }).parse(row))
  }
  const available = await columns(source)
  const valid = (requested: readonly string[], known: typeof available) => {
    if (
      new Set(requested).size !== requested.length ||
      requested.some((name) => !known.some((c) => c.column_name === name))
    )
      throw new Error('Choose distinct columns from the table being checked.')
  }
  const results = []
  for (const check of checks) {
    signal.throwIfAborted()
    switch (check.kind) {
      case 'key': {
        valid(check.columns, available)
        const keys = check.columns.map(identifier)
        const complete = keys.map((k) => `${k} IS NOT NULL`).join(' AND ')
        const query = `WITH groups AS (SELECT ${keys.join(', ')}, count(*) n FROM ${identifier(source)} GROUP BY ${keys.join(', ')}) SELECT count(*) AS groups, count(*) FILTER (WHERE n > 1 AND ${complete}) AS "duplicateGroups", coalesce(sum(n - 1) FILTER (WHERE n > 1 AND ${complete}), 0) AS "duplicateRows", coalesce(sum(n) FILTER (WHERE NOT (${complete})), 0) AS "nullRows" FROM groups`
        results.push({
          kind: 'key' as const,
          columns: check.columns,
          ...keyStats.parse((await connection.query(query)).toArray()[0]),
        })
        break
      }
      case 'time': {
        valid([check.column, ...check.units], available)
        const column = available.find((c) => c.column_name === check.column)!
        const numeric = isNumericDuckDbType(column.column_type)
        if (!numeric && !/^(DATE|TIMESTAMP)/.test(column.column_type))
          throw new Error(
            'Time coverage needs a numeric period, date or timestamp column. Parse text dates first.',
          )
        const period = numeric
          ? `CAST(${identifier(check.column)} AS DOUBLE)`
          : `epoch(${identifier(check.column)})`
        const partition = check.units.length
          ? `PARTITION BY ${check.units.map(identifier).join(', ')}`
          : ''
        const nulls = [check.column, ...check.units]
          .map((n) => `${identifier(n)} IS NULL`)
          .join(' OR ')
        const query = `WITH periods AS (SELECT *, ${period} AS __hirmos_time, (${nulls}) AS __hirmos_null FROM ${identifier(source)}), steps AS (SELECT __hirmos_time - lag(__hirmos_time) OVER (${partition} ORDER BY __hirmos_time) AS delta FROM periods WHERE NOT __hirmos_null) SELECT (SELECT count(*) FROM periods) AS rows, (SELECT count(*) FROM periods WHERE __hirmos_null) AS "nullRows", count(*) FILTER (WHERE delta = 0) AS "duplicatePeriods", count(*) FILTER (WHERE delta > ${check.step} * (1 + 1e-8)) AS gaps, count(*) FILTER (WHERE delta > 0 AND abs(delta / ${check.step} - round(delta / ${check.step})) > 1e-8) AS "offGridSteps", min(delta) AS "minimumStep", max(delta) AS "maximumStep" FROM steps`
        results.push({
          kind: 'time' as const,
          column: check.column,
          units: check.units,
          step: check.step,
          scale: numeric ? 'column units' : 'seconds',
          ...timeStats.parse((await connection.query(query)).toArray()[0]),
        })
        break
      }
      case 'join': {
        valid(check.left, available)
        valid(check.right, await columns(check.table))
        if (check.left.length !== check.right.length)
          throw new Error('Each left join key needs a corresponding right key.')
        const groups = (table: string, keys: string[]) =>
          `SELECT ${keys.map((n, i) => `${identifier(n)} AS k${i}`).join(', ')}, count(*) AS n FROM ${identifier(table)} GROUP BY ${keys.map(identifier).join(', ')}`
        const condition = check.left.map((_, i) => `l.k${i} = r.k${i}`).join(' AND ')
        const query = `WITH l AS (${groups(source, check.left)}), r AS (${groups(check.table, check.right)}), joined AS (SELECT l.n AS ln, r.n AS rn FROM l FULL OUTER JOIN r ON ${condition}) SELECT (SELECT coalesce(sum(n), 0) FROM l) AS "leftRows", (SELECT coalesce(sum(n), 0) FROM r) AS "rightRows", coalesce(sum(ln * rn), 0) AS "innerJoinRows", coalesce(sum(ln * coalesce(rn, 1)), 0) AS "leftJoinRows", coalesce(sum(ln) FILTER (WHERE rn IS NULL), 0) AS "unmatchedLeftRows", coalesce(sum(rn) FILTER (WHERE ln IS NULL), 0) AS "unmatchedRightRows", (SELECT count(*) FROM l WHERE n > 1) AS "leftDuplicateKeys", (SELECT count(*) FROM r WHERE n > 1) AS "rightDuplicateKeys" FROM joined`
        results.push({
          kind: 'join' as const,
          table: check.table,
          left: check.left,
          right: check.right,
          nullKeys: 'do not match' as const,
          ...joinStats.parse((await connection.query(query)).toArray()[0]),
        })
        break
      }
    }
  }
  return results
}
