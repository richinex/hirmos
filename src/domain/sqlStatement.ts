const READS = new Set([
  'select',
  'with',
  'from',
  'summarize',
  'describe',
  'show',
  'explain',
  'values',
  'pivot',
  'unpivot',
  'table',
])

const stripped = (batch: string): string =>
  batch
    .replace(/--[^\n]*/g, ' ')
    .replace(/\/\*[\s\S]*?\*\//g, ' ')
    .replace(/'(?:[^']|'')*'/g, "''")
    .replace(/"(?:[^"]|"")*"/g, '""')

export const changesCatalog = (batch: string): boolean =>
  stripped(batch)
    .split(';')
    .map((statement) => statement.trim().toLowerCase())
    .filter((statement) => statement.length > 0)
    .some((statement) => !READS.has(statement.split(/[\s(]/, 1)[0] ?? ''))
