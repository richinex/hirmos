import type { PreparedMatrix } from '@/data/prepared'
import type { DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import type { NonEmptyArray } from '@/domain/dop'
import type { EstimationRunArtifact, EstimatorConfiguration } from '@/domain/estimation'
import type { EstimationDraft, PanelBinding } from '@/domain/estimationDraft'
import type { MethodEligibility } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { IdentificationArtifact, StudySpecification, StudyVariable } from '@/domain/study'
import type { SunAbrahamPanelJob } from '@/domain/sunAbraham'
import type { SelectedSource } from '@/domain/workflow'
import type { AnalysisProgress } from '@/workers/analysisProtocol'

export type EstimationRunEvent =
  | { readonly type: 'run-progressed'; readonly progress: AnalysisProgress }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }

export interface PointwiseRetry {
  readonly configuration: EstimatorConfiguration
  readonly study: StudySpecification['id']
  readonly prepared: PreparedDatasetArtifact['id']
}

export interface EstimationInput {
  readonly configuration: EstimatorConfiguration
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly study: StudySpecification
  readonly identification: IdentificationArtifact
  readonly eligibility: Exclude<MethodEligibility, { kind: 'refused' }>
  readonly encodings: EstimationDraft['encodings']
  readonly panelPreflight: EstimationDraft['panelPreflight']
  readonly panelBinding: PanelBinding | null
  readonly sunPanel: SunAbrahamPanelJob
  readonly document: DagDocument | null
}

export interface EstimationEffects {
  readonly isCurrent: () => boolean
  readonly emit: (event: EstimationRunEvent) => void
  readonly onRun: (run: EstimationRunArtifact) => void
  readonly offerPointwiseRetry: (retry: PointwiseRetry) => void
}

export interface EstimationExecution extends EstimationInput {
  readonly analysis: typeof import('@/analysis/client')
  readonly materialise: (
    columns: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>>,
  ) => Promise<PreparedMatrix>
  readonly columnAt: (values: Float64Array, rows: number, index: number) => number[]
  readonly columnName: (id: string) => string
  readonly identity: Pick<
    EstimationRunArtifact,
    | 'id'
    | 'study'
    | 'identification'
    | 'preparedDataset'
    | 'createdAt'
    | 'eligibility'
    | 'encodings'
  >
  readonly finish: (run: EstimationRunArtifact | null, detail: string) => void
  readonly dispatch: EstimationEffects['emit']
  readonly isCurrent: EstimationEffects['isCurrent']
  readonly offerPointwiseRetry: EstimationEffects['offerPointwiseRetry']
}

export type BackdoorExecution = Omit<EstimationExecution, 'identification'> & {
  readonly identification: IdentificationArtifact & {
    readonly result: Extract<IdentificationArtifact['result'], { kind: 'identified' }>
  }
}
