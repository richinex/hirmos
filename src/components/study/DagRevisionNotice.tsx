import { useWorkflow } from '@/components/WorkflowProvider'
import { Alert } from '@/components/ui/Alert'
import { resolveStudyDagRevision, studyRevisionNotice } from '@/domain/dagProvenance'
import type { StudySpecification } from '@/domain/study'

export function DagRevisionNotice({ study }: { readonly study: StudySpecification }) {
  const notice = useWorkflow((state) =>
    state.workflow.kind === 'profiled'
      ? studyRevisionNotice(resolveStudyDagRevision(study, state.workflow.dagDocuments))
      : null,
  )
  return notice === null ? null : (
    <Alert tone="warn" live={false}>
      <p className="m-0">{notice}</p>
    </Alert>
  )
}
