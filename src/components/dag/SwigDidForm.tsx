import { NumberInput, numberValue } from '@/components/ui/NumberInput'
import { useState, type Dispatch, type SetStateAction } from 'react'
import {
  assignSwigRole,
  prepareDidSwig,
  type SwigDidDraft,
  type SwigRoleKind,
} from '@/domain/swigDidDraft'
import type { ProjectedSwigGraph } from '@/domain/swigProjection'
import type { SwigSpecification } from '@/domain/swig'
import { Select } from '@/components/ui/Select'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { RunActions } from '@/components/ui/RunActions'
import { Alert } from '@/components/ui/Alert'
import { Icon } from '@/components/Icon'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { button, field, fieldHint, fieldLabel } from '@/components/ui/recipes'

const ROLE_LISTS: readonly {
  readonly kind: SwigRoleKind
  readonly title: string
  readonly help: string
}[] = [
  {
    kind: 'confounder',
    title: 'Common causes',
    help: 'Choose the time-invariant common causes.',
  },
  {
    kind: 'treatment',
    title: 'Treatments',
    help: 'Choose one treatment node for each period from 1 to the last outcome period.',
  },
  {
    kind: 'outcome',
    title: 'Outcomes',
    help: 'Choose one outcome node for each period, starting at period 0 with no gaps and at least two periods. Outcomes must not cause other variables (R-Y).',
  },
  {
    kind: 'covariate',
    title: 'Covariates',
    help: 'Only covariates marked Observed in the DAG count as measured controls.',
  },
  {
    kind: 'disturbance',
    title: 'Exogenous disturbances',
    help: 'Choose disturbance variables with no incoming arrows and exactly one outgoing arrow to the variable they affect. Each outcome must have its own disturbance variable. Other variables may have one too. This requirement applies to this DiD assessment.',
  },
]

const ASSUMPTIONS = [
  [
    'Treatment timing',
    'Treatment is either on or off. All units are untreated in period 0. Once a unit becomes treated, it remains treated.',
  ],
  [
    'Consistency',
    "A unit's observed outcome equals its potential outcome under the treatment history it actually experienced.",
  ],
  [
    'Independent disturbances',
    'Disturbance terms represent influences outside the model and are assumed to be mutually independent. Represent any shared unobserved cause explicitly in the graph.',
  ],
] as const

export function SwigDidForm({
  graph,
  draft,
  onChange,
  blocked,
  running,
  onCancel,
  onRun,
}: {
  readonly draft: SwigDidDraft
  readonly onChange: Dispatch<SetStateAction<SwigDidDraft>>
  readonly graph: ProjectedSwigGraph
  readonly blocked: boolean
  readonly running: boolean
  readonly onCancel: () => void
  readonly onRun: (specification: SwigSpecification, rationale: string) => void
}) {
  const { roles, adoption, outcome, comparison, selected, rationale } = draft
  const [problem, setProblem] = useState<string | null>(null)
  const setAdoption = (adoption: number) => onChange((d) => ({ ...d, adoption }))
  const setOutcome = (outcome: number) => onChange((d) => ({ ...d, outcome }))
  const setComparison = (comparison: SwigDidDraft['comparison']) =>
    onChange((d) => ({ ...d, comparison }))
  const setSelected = (selected: readonly string[]) => onChange((d) => ({ ...d, selected }))
  const setRationale = (rationale: string) => onChange((d) => ({ ...d, rationale }))
  const assign = (kind: SwigRoleKind, chosen: readonly number[]) =>
    onChange((d) => {
      const next = assignSwigRole(graph, d.roles, kind, chosen)
      return {
        ...d,
        roles: next,
        selected: d.selected.filter((id) => next[Number(id)]?.kind === 'covariate'),
      }
    })
  const setPeriod = (index: number, period: number) =>
    onChange((d) => ({
      ...d,
      roles: d.roles.map((role, i) =>
        i === index && 'period' in role ? { ...role, period } : role,
      ),
    }))

  const unassigned = graph.names.filter((_, i) => roles[i]?.kind === 'unassigned')
  // Nodes from a time expansion carry their period; only explicit nodes need one entered.
  const timed = roles.flatMap((role, index) =>
    'period' in role && graph.origins[index]?.period == null ? [{ index, role }] : [],
  )

  const run = () => {
    const result = prepareDidSwig(graph, roles, {
      adoption,
      outcome,
      comparison,
      selected: selected.map(Number),
    })
    if (!result.ok) {
      setProblem(result.error)
      return
    }
    setProblem(null)
    onRun(result.value, rationale.trim())
  }

  return (
    <div className="space-y-4">
      <p className={fieldHint}>
        Assess a sufficient adjustment rule for conditional parallel trends. Period 0 is the
        untreated baseline; treatment begins in periods 1 or later. This checks declared structure
        and control availability, not whether parallel trends or overlap hold in the data.
      </p>
      <fieldset disabled={blocked} className="min-w-0 space-y-4 border-0 p-0">
        {ROLE_LISTS.map(({ kind, title, help }) => (
          <div key={kind} className="panel-scroll max-h-48 overflow-y-auto">
            <ColumnChecklist
              title={title}
              help={help}
              columns={graph.names.flatMap((name, i) =>
                roles[i]?.kind === 'unassigned' || roles[i]?.kind === kind
                  ? [{ id: String(i), name }]
                  : [],
              )}
              selected={roles.flatMap((role, i) => (role.kind === kind ? [String(i)] : []))}
              onChange={(chosen) => assign(kind, chosen.map(Number))}
            />
          </div>
        ))}
        {unassigned.length > 0 && (
          <p className={fieldHint}>Choose a role for {unassigned.join(', ')}.</p>
        )}
        {timed.length > 0 && (
          <fieldset className="min-w-0 space-y-2 border-0 p-0">
            <legend className={fieldLabel}>Periods</legend>
            <div className="grid grid-cols-[minmax(0,1fr)_5rem] items-center gap-x-3 gap-y-2">
              {timed.map(({ index, role }) => (
                <label key={index} className="contents">
                  <span className="truncate text-body text-ink">{graph.names[index]}</span>
                  <NumberInput
                    min={role.kind === 'treatment' ? 1 : 0}
                    max={32}
                    step={1}
                    aria-label={`Period for ${graph.names[index]}`}
                    className={field('text')}
                    value={role.period}
                    onChange={(e) => setPeriod(index, numberValue(e.target))}
                  />
                </label>
              ))}
            </div>
          </fieldset>
        )}
        <section aria-labelledby="swig-did-assumptions">
          <h4
            id="swig-did-assumptions"
            className="m-0 flex items-center gap-1.5 text-body font-medium text-ink"
          >
            <Icon name="gavel" size={14} className="text-faint" />
            Assumptions
          </h4>
          <p className={fieldHint}>
            These assumptions are needed for this DiD assessment. The graph and data do not
            establish that they hold. You can record your assumptions below.
          </p>
          <div className="mt-2 rounded-lg border border-hair bg-raised p-3 text-body">
            <RecordList>
              <RecordRow term="Common additive component">
                <span className="inline-flex items-start gap-1.5">
                  Time-invariant unobserved causes contribute to untreated outcomes in the same
                  additive way in every period. This shared contribution cancels when outcomes are
                  differenced.
                  <ParameterHelp
                    label="Common additive component"
                    help="The paper calls this restriction R-alpha. Each untreated outcome is alpha plus a period-specific function g. Alpha is the same function, with the same coefficient, in every period. Its arguments are the common causes and any covariate with an arrow into every outcome. Each g takes the outcome's other random parents, including its disturbance. A covariate in alpha may also be an argument of g. The paper calls binary, absorbing treatment with an untreated baseline ST."
                  />
                </span>
              </RecordRow>
              {ASSUMPTIONS.map(([term, statement]) => (
                <RecordRow key={term} term={term}>
                  {statement}
                </RecordRow>
              ))}
            </RecordList>
          </div>
        </section>
        <div className="grid grid-cols-2 gap-3">
          <label>
            <span className={fieldLabel}>Adoption period</span>
            <NumberInput
              aria-label="DiD adoption period"
              min={1}
              max={32}
              className={field('text', 'mt-1')}
              value={adoption}
              onChange={(e) => setAdoption(numberValue(e.target))}
            />
          </label>
          <label>
            <span className={fieldLabel}>Outcome period</span>
            <NumberInput
              aria-label="DiD outcome period"
              min={0}
              max={32}
              className={field('text', 'mt-1')}
              value={outcome}
              onChange={(e) => setOutcome(numberValue(e.target))}
            />
          </label>
        </div>
        <label className="block">
          <span className={fieldLabel}>Comparison group</span>
          <Select
            aria-label="DiD graph comparison group"
            value={comparison.kind}
            className={field('text', 'mt-1')}
            onChange={(e) =>
              setComparison(
                e.target.value === 'neverTreated'
                  ? { kind: 'neverTreated' }
                  : { kind: 'notYetTreated', through: Math.max(adoption - 1, outcome) },
              )
            }
          >
            <option value="neverTreated">Never treated</option>
            <option value="notYetTreated">Not yet treated</option>
          </Select>
        </label>
        {comparison.kind === 'notYetTreated' && (
          <label className="block">
            <span className={fieldLabel}>Untreated through period</span>
            <NumberInput
              aria-label="Comparison untreated through period"
              min={0}
              max={32}
              className={field('text', 'mt-1')}
              value={comparison.through}
              onChange={(e) =>
                setComparison({ kind: 'notYetTreated', through: numberValue(e.target) })
              }
            />
          </label>
        )}
        <div className="panel-scroll max-h-48 overflow-y-auto">
          <ColumnChecklist
            title="Proposed controls"
            help="The result distinguishes the required controls from optional observed controls and checks this selection."
            columns={graph.names.flatMap((name, i) =>
              roles[i]?.kind === 'covariate' && graph.measured.includes(i)
                ? [{ id: String(i), name }]
                : [],
            )}
            selected={selected}
            onChange={setSelected}
          />
        </div>
        <label className="block">
          <span className={fieldLabel}>Assumptions and rationale (optional)</span>
          <textarea
            aria-label="DiD graph rationale"
            className={field('text', 'mt-1')}
            rows={4}
            value={rationale}
            onChange={(e) => setRationale(e.target.value)}
          />
        </label>
      </fieldset>
      {problem !== null && <Alert tone="warn">{problem}</Alert>}
      <RunActions
        running={running}
        onCancel={onCancel}
        orbLabel="Assessing conditional parallel trends"
      >
        <button type="button" className={button('signal')} disabled={blocked} onClick={run}>
          Assess DiD adjustment
        </button>
      </RunActions>
    </div>
  )
}
