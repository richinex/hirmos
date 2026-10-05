import { isNonEmpty, type NonEmptyArray } from './dop'
import type { EvidenceGroup } from './methods'
import {
  describeLevelIssue,
  describeLevelIssueGroup,
  groupLevelIssues,
  levelIssueAction,
  type LevelIssueWording,
  type StationarityAssessment,
} from './stationarityAssessment'

/** What the stationarity battery says about a method fitted in levels, grouped by reason. */
export interface LevelEvidence {
  /** Series whose order of integration rules the method out. */
  readonly refused: readonly EvidenceGroup[]
  /** Series to review before trusting the fit. */
  readonly cautions: readonly EvidenceGroup[]
}

export function levelEvidence(
  readings: readonly {
    readonly name: string
    readonly assessment: StationarityAssessment | null
  }[],
  wording: LevelIssueWording,
): LevelEvidence {
  const refused: EvidenceGroup[] = []
  const cautions: EvidenceGroup[] = []
  for (const group of groupLevelIssues(readings)) {
    const evidence: EvidenceGroup = {
      summary: describeLevelIssueGroup(group, wording),
      reason: describeLevelIssue(group.issue, wording),
      series: group.series,
      action: levelIssueAction(group.issue),
    }
    ;(group.issue.kind === 'higherOrder' ? refused : cautions).push(evidence)
  }
  return { refused, cautions }
}

/** The groups' stage lines as one text, for the evaluation's evidence field. */
export const summaries = (groups: readonly EvidenceGroup[]): string =>
  groups.map((group) => group.summary).join(' ')

export const nonEmptyGroups = (
  groups: readonly EvidenceGroup[],
): NonEmptyArray<EvidenceGroup> | undefined => (isNonEmpty(groups) ? groups : undefined)
