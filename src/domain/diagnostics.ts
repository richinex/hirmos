import type { ColumnId, TimeAxis } from './dataset'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { SeriesStructureEvidence } from './sensitivity'
import type { PreparedMatrix } from '@/data/prepared'
import type { MulticollinearityEvidence } from './multicollinearity'

export interface Redundancy {
  readonly prepared: PreparedDatasetVersionId
  readonly matrix: PreparedMatrix
  readonly evidence: MulticollinearityEvidence
}

export interface SeriesFacts {
  readonly column: ColumnId
  readonly name: string
  readonly values: readonly number[]
  readonly evidence: SeriesStructureEvidence['series'][number]
  readonly timeAxis: TimeAxis | null
}

export interface TemporalStructure {
  readonly prepared: PreparedDatasetVersionId
  readonly period: number | null
  readonly minSize: number
  readonly maxLag: number
  readonly series: readonly SeriesFacts[]
}
