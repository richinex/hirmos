import { useState } from 'react'
import { Alert } from '@/components/ui/Alert'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { RunMeta } from '@/components/ui/RunMeta'
import { Select } from '@/components/ui/Select'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { button, caption, field, fieldLabel } from '@/components/ui/recipes'
import type { DecisionEvidence, PlanEstimate } from '@/domain/decisionEvidence'
import type { DecisionFrame } from '@/domain/decisionFrame'
import {
  describeDecisionRecordProblem,
  draftDecisionRecord,
  expectedValue,
  recordablePlans,
  type DecisionBasis,
  type DecisionRecord,
  type DecisionRecordDraft,
} from '@/domain/decisionRecord'
import { formatDay, formatTime } from '@/lib/format/date'
import { formatCount, formatPercent, formatStatistic } from '@/lib/format/number'

const rate = (value: number) => formatStatistic('mean', value).text
const signed = (value: number) => (value > 0 ? `+${rate(value)}` : rate(value))

const stated = (estimate: PlanEstimate, show: (value: number) => string) =>
  estimate.interval === null
    ? `${show(estimate.value)}, interval not evaluated`
    : `${show(estimate.value)}, 95% interval ${show(estimate.interval.lower)} to ${show(estimate.interval.upper)}`

const CONTROL_HELP =
  'Keeping a share of cases on current practice makes the deployed rule a mixture: the chosen plan for the other cases, current practice for this share. The record states the mixture; the plan’s estimate is not adjusted for it.'

const EMPTY: DecisionRecordDraft = {
  plan: '',
  control: '',
  rationale: '',
  reconsider: '',
  reviewOn: '',
}

/** Where the plan's evidence comes from, in one line per source. */
function evidenceLines(basis: DecisionBasis): readonly string[] {
  return [
    ...(basis.weighted === null
      ? []
      : [
          `Weighted by ${basis.weighted.propensity.name} over ${formatCount(basis.weighted.rows).text} ${basis.weighted.heldOut ? 'held-out ' : ''}logged rows, ${formatCount(Math.round(basis.weighted.effectiveRows)).text} effective`,
        ]),
    ...(basis.randomised === null
      ? []
      : [
          `Randomised estimate ${rate(basis.randomised.mean)} from ${formatCount(basis.randomised.rows).text} rows`,
        ]),
    ...(basis.bounds === null
      ? []
      : [`Natural bounds ${rate(basis.bounds.lower)} to ${rate(basis.bounds.upper)}`]),
  ]
}

function BasisRows({ basis }: { readonly basis: DecisionBasis }) {
  const expected = expectedValue(basis)
  return (
    <>
      <RecordRow term="Evidence">
        <ul className="m-0 list-none p-0 text-body text-muted">
          {evidenceLines(basis).map((line) => (
            <li key={line}>{line}</li>
          ))}
        </ul>
      </RecordRow>
      <RecordRow term="Expected value">
        <p className="m-0 text-body text-ink">
          {expected === null
            ? basis.bounds === null
              ? 'Not evaluated'
              : `Between ${rate(basis.bounds.lower)} and ${rate(basis.bounds.upper)}`
            : stated(expected, rate)}
        </p>
      </RecordRow>
      <RecordRow term="Against current practice">
        <p className="m-0 text-body text-ink">
          {basis.weighted === null ? 'Not evaluated' : stated(basis.weighted.difference, signed)}
        </p>
      </RecordRow>
    </>
  )
}

/** The form that records a decision from the current evidence run. */
export function DecisionRecordForm({
  run,
  frame,
  firstStep,
  onRecord,
}: {
  readonly run: DecisionEvidence
  readonly frame: DecisionFrame
  readonly firstStep: number
  readonly onRecord: (record: DecisionRecord) => void
}) {
  const plans = recordablePlans(run)
  const [draft, setDraft] = useState<DecisionRecordDraft>({
    ...EMPTY,
    plan: plans[0]?.key ?? '',
  })
  const [problem, setProblem] = useState<string | null>(null)
  const [saved, setSaved] = useState(false)
  const set = (patch: Partial<DecisionRecordDraft>) => {
    setDraft((current) => ({ ...current, ...patch }))
    setSaved(false)
  }
  const plan = plans.find((candidate) => candidate.key === draft.plan) ?? null

  function record() {
    const built = draftDecisionRecord(run, frame, draft, new Date())
    if (!built.ok) {
      setProblem(describeDecisionRecordProblem(built.error))
      return
    }
    setProblem(null)
    onRecord(built.value)
    setDraft({ ...EMPTY, plan: draft.plan })
    setSaved(true)
  }

  return (
    <section aria-labelledby="decision-record-title" className="flex flex-col gap-5">
      <p className="m-0 max-w-[70ch] text-body text-muted" id="decision-record-title">
        Record what was known and expected now, so the decision can later be judged on how it was
        made, not only on how it turned out. Recording assigns no one and starts no experiment.
      </p>
      <SettingsStep number={firstStep} title="Choose the plan">
        <label className="block max-w-md">
          <span className={fieldLabel}>Plan</span>
          <Select
            className={field('text', 'mt-1')}
            value={draft.plan}
            onChange={(event) => set({ plan: event.target.value })}
          >
            {plans.map((candidate) => (
              <option key={candidate.key} value={candidate.key}>
                {candidate.label}
              </option>
            ))}
          </Select>
        </label>
        {plan !== null && (
          <div className="mt-3 max-w-2xl">
            <RecordList>
              <BasisRows basis={plan.basis} />
            </RecordList>
          </div>
        )}
      </SettingsStep>
      <SettingsStep number={firstStep + 1} title="State the reasons">
        <div className="grid max-w-2xl grid-cols-1 gap-4">
          <label className="block">
            <span className={fieldLabel}>Rationale</span>
            <textarea
              className={field('text', 'mt-1')}
              rows={3}
              value={draft.rationale}
              onChange={(event) => set({ rationale: event.target.value })}
            />
          </label>
          <label className="block">
            <span className={fieldLabel}>What would change this decision</span>
            <textarea
              className={field('text', 'mt-1')}
              rows={3}
              value={draft.reconsider}
              onChange={(event) => set({ reconsider: event.target.value })}
            />
          </label>
        </div>
      </SettingsStep>
      <SettingsStep number={firstStep + 2} title="Set the review">
        <div className="flex flex-wrap items-end gap-4">
          <div className="w-56">
            <ParameterLabel
              className={fieldLabel}
              htmlFor={`${run.id}-control`}
              label="Kept on current practice"
              help={CONTROL_HELP}
            />
            <span className="mt-1 flex items-center gap-2">
              <input
                id={`${run.id}-control`}
                className={field('text')}
                inputMode="decimal"
                placeholder="None"
                value={draft.control}
                onChange={(event) => set({ control: event.target.value })}
              />
              <span className="text-body text-muted">%</span>
            </span>
          </div>
          <label className="block w-48">
            <span className={fieldLabel}>Review on</span>
            <input
              type="date"
              className={field('text', 'mt-1')}
              value={draft.reviewOn}
              onChange={(event) => set({ reviewOn: event.target.value })}
            />
          </label>
        </div>
      </SettingsStep>
      {problem !== null && <Alert tone="warn">{problem}</Alert>}
      <div className="flex flex-wrap items-center gap-3">
        <button
          type="button"
          className={button('signal')}
          disabled={plans.length === 0}
          onClick={record}
        >
          Record decision
        </button>
        {saved && (
          <span role="status" className="text-body text-ok">
            Recorded with the graph revision, dataset and evidence run it used.
          </span>
        )}
      </div>
    </section>
  )
}

/** A recorded decision, laid out as the framed decision is. */
export function RecordedDecision({ record }: { readonly record: DecisionRecord }) {
  return (
    <section aria-label={`Recorded decision: ${record.plan}`} className="flex flex-col gap-3">
      <RecordList>
        <RecordRow term="Plan">
          <p className="m-0 text-body text-ink">{record.plan}</p>
          {record.control !== null && (
            <p className={caption('mb-0 mt-1')}>
              Deployed as a mixture: {formatPercent(record.control.share).text} of cases stay on
              current practice.
            </p>
          )}
        </RecordRow>
        <BasisRows basis={record.basis} />
        <RecordRow term="Assumptions">
          <ul className="m-0 list-disc pl-4 text-body text-muted">
            {record.assumptions.map((assumption) => (
              <li key={assumption}>{assumption}</li>
            ))}
          </ul>
        </RecordRow>
        <RecordRow term="Rationale">
          <p className="m-0 text-body text-muted">{record.rationale}</p>
        </RecordRow>
        <RecordRow term="What would change it">
          <p className="m-0 text-body text-muted">{record.reconsider}</p>
        </RecordRow>
        <RecordRow term="Review on">
          <p className="m-0 text-body text-ink">{formatDay(`${record.reviewOn}T00:00`)}</p>
        </RecordRow>
      </RecordList>
      <p className={caption('m-0')}>
        <RunMeta>{['Recorded decision', formatTime(record.createdAt)]}</RunMeta>
      </p>
    </section>
  )
}
