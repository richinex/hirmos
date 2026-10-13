import { assertNever } from '@/domain/dop'
import { describeCalendarWindow, type PipelineBlock, type RowTest } from '@/domain/pipeline'

const testWord = (test: RowTest): string => {
  switch (test) {
    case 'eq':
      return '='
    case 'ne':
      return '≠'
    case 'lt':
      return '<'
    case 'le':
      return '≤'
    case 'gt':
      return '>'
    case 'ge':
      return '≥'
    case 'contains':
      return 'contains'
    case 'starts-with':
      return 'starts with'
    case 'is-null':
      return 'is missing'
    case 'not-null':
      return 'is present'
    default:
      return assertNever(test)
  }
}

/** A block's setting in a line, the same words on the canvas card and in the assistant's proposal. */
export const summariseBlock = (block: PipelineBlock): string => {
  switch (block.kind) {
    case 'input':
      return block.file.kind === 'chosen' ? block.file.alias : 'choose a file'
    case 'filter-rows':
      return block.conditions.length === 0
        ? 'every row'
        : block.conditions
            .map((c) => `${c.column} ${testWord(c.test)}${'value' in c ? ` ${c.value}` : ''}`)
            .join(block.match === 'all' ? ' and ' : ' or ')
    case 'select-columns':
      return block.columns.length === 0
        ? 'every column'
        : `${block.mode} ${block.columns.join(', ')}`
    case 'derive-columns':
      return block.columns.length === 0
        ? 'no new columns yet'
        : block.columns.map((c) => c.name || '?').join(', ')
    case 'calendar-events':
      return block.column === ''
        ? 'choose the date column'
        : `${block.name || '?'}: ${describeCalendarWindow(block.window)} over ${block.column}`
    case 'join':
      return `${block.how}${block.keys.length === 0 ? '' : ` on ${block.keys.map((k) => (k.left === k.right ? k.left : `${k.left} = ${k.right}`)).join(', ')}`}`
    case 'union':
      return `by ${block.by}${block.distinct ? ', distinct' : ''}`
    case 'aggregate':
      return `${block.groupBy.length === 0 ? 'all rows' : `by ${block.groupBy.join(', ')}`}, ${block.measures.map((m) => m.as || m.function).join(', ') || 'no measures yet'}`
    case 'sort-limit':
      return `${block.sort.map((s) => `${s.column} ${s.direction === 'ascending' ? '↑' : '↓'}`).join(', ') || 'unsorted'}${block.limit === null ? '' : `, first ${block.limit}`}`
    case 'script':
      return `${block.code.split('\n').filter((line) => line.trim().length > 0).length} lines`
    case 'output':
      return 'the source'
    default:
      return assertNever(block)
  }
}
