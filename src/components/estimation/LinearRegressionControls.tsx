import { Formula } from '@/components/ui/Formula'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { field, fieldHint, fieldLabel, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { Select } from '@/components/ui/Select'
import { SettingsStep } from '@/components/ui/SettingsStep'
import type { PhysicalColumnProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import {
  selectFixedEffects,
  type EstimatorConfiguration,
  type FixedEffects,
  type LinearErrors,
} from '@/domain/estimation'
import { DEFAULT_ARMA_ITERATIONS, MAX_ARMA_ORDER } from '@/domain/interruptedSeries'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'

type Configuration = Extract<EstimatorConfiguration, { kind: 'backdoor-linear-regression' }>

type Props = {
  readonly covariatesStep: (number: number) => ReactNode
  readonly categoricalChecklist: ReactNode
  readonly configuration: Configuration
  readonly panelTime: PhysicalColumnProfile | null
  readonly timeCandidates: readonly PhysicalColumnProfile[]
  readonly groupingCandidates: readonly PhysicalColumnProfile[]
  readonly configure: (next: Configuration) => void
  readonly panelUnit: PhysicalColumnProfile | null
}

/** The HAC bandwidth rule the hint refers to, typeset like every other formula in the workbench. */
/** The HC1 small-sample factor the hint refers to; G counts the absorbed fixed effects when there are any. */
const hc1Factor = (absorbsEffects: boolean) =>
  absorbsEffects
    ? { tex: 'c = \\dfrac{n}{n - G - k}', plain: 'c = n / (n − G − k)' }
    : { tex: 'c = \\dfrac{n}{n - k}', plain: 'c = n / (n − k)' }

const HAC_BANDWIDTH = {
  tex: 'L = \\left\\lfloor 4\\left(\\dfrac{n}{100}\\right)^{2/9} \\right\\rfloor',
  plain: 'L = floor(4 (n/100)^(2/9))',
} as const

/** What the chosen error treatment assumes and how its interval is built, for the hint under the control. */
const describeErrorTreatment = (errors: LinearErrors, fixedEffects: FixedEffects): string => {
  const effects =
    fixedEffects.kind === 'none'
      ? null
      : fixedEffects.kind === 'unit-and-time'
        ? `${fixedEffects.name} and ${fixedEffects.timeName} effects`
        : `${fixedEffects.name} effects`
  switch (errors.kind) {
    case 'classical':
      return effects !== null
        ? `The interval uses a 95% confidence level. The classical interval assumes errors are independent and share a common variance; its degrees of freedom count the ${effects} as parameters.`
        : 'The interval uses a 95% confidence level. The classical interval assumes errors are independent and share a common variance.'
    case 'hc1':
      return effects !== null
        ? `The interval uses a 95% confidence level. The robust (HC1) interval allows the error variance to differ between rows and scales each squared residual by the factor c, where n is the number of rows and G counts the ${effects}:`
        : 'The interval uses a 95% confidence level. The robust (HC1) interval allows the error variance to differ between rows and scales each squared residual by the factor c, where n is the number of rows:'
    case 'cluster':
      return `The interval uses a 95% confidence level. The clustered interval allows errors to correlate within each value of ${errors.name} and treats the clusters as independent. A finite-sample correction accounts for the cluster count and fitted parameters.${effects === null ? '' : ' Fixed effects whose groups lie entirely within clusters are excluded from the parameter count used in this correction. Intervals and p-values use the number of clusters minus one degrees of freedom.'}`
    case 'hac':
      return 'The interval uses a 95% confidence level. Heteroskedasticity- and autocorrelation-consistent (HAC) covariance uses a Bartlett kernel and a bandwidth of L lags, where n is the number of rows:'
    case 'arma':
      return 'The interval uses a 95% confidence level.'
    default:
      return assertNever(errors)
  }
}

/** What the fixed-effects choice does to the regression, for the hint under the control. */
const describeFixedEffectsChoice = (fixedEffects: FixedEffects): string => {
  switch (fixedEffects.kind) {
    case 'none':
      return 'Every row is used as observed. Choose fixed effects when the same unit appears in several rows.'
    case 'time':
      return `Period effects account for additive differences shared by rows with the same ${fixedEffects.name}. They do not account for persistent differences between units.`
    case 'unit':
      return `Each variable is replaced by its deviation from the mean of its ${fixedEffects.name}. The run lists adjustment variables absorbed by these effects.`
    case 'unit-and-time':
      return `The regression removes additive ${fixedEffects.name} and ${fixedEffects.timeName} effects before fitting the remaining variation. The run lists adjustment variables absorbed by those effects. Unbalanced panels are also supported.`
    default:
      return assertNever(fixedEffects)
  }
}

export function LinearRegressionControls({
  covariatesStep,
  categoricalChecklist,
  configuration,
  panelTime,
  timeCandidates,
  groupingCandidates,
  configure,
  panelUnit,
}: Props): ReactNode {
  return (
    <div className={stepsStack}>
      {covariatesStep(1)}
      <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Fixed effects">
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Fixed effects"
            help={ESTIMATION_PARAMETER_HELP.adjustedRegression.fixedEffects}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Fixed effects"
            value={configuration.fixedEffects.kind}
            onChange={(kind) => {
              const current = configuration.fixedEffects
              const unit =
                kind === 'time'
                  ? (panelTime ?? timeCandidates[0] ?? null)
                  : current.kind === 'none' || current.kind === 'time'
                    ? (groupingCandidates[0] ?? null)
                    : { id: current.column, name: current.name }
              const time =
                current.kind === 'unit-and-time'
                  ? { id: current.timeColumn, name: current.timeName }
                  : (timeCandidates.find((candidate) => candidate.id !== unit?.id) ?? null)
              const selection = selectFixedEffects(kind, unit, time)
              if (!selection.ok) return
              const fixedEffects = selection.value
              // Series error processes are not supported by the within fit. Clustering is independent.
              const errors: LinearErrors =
                fixedEffects.kind !== 'none'
                  ? configuration.errors.kind === 'hac' || configuration.errors.kind === 'arma'
                    ? { kind: 'classical' }
                    : configuration.errors
                  : configuration.errors
              configure({ ...configuration, fixedEffects, errors })
            }}
            options={[
              { value: 'none', label: 'None' },
              {
                value: 'unit',
                label: 'By unit',
                ariaLabel: 'By unit',
                disabled: groupingCandidates.length === 0,
                title:
                  groupingCandidates.length === 0
                    ? 'Requires a grouping column outside the regression design.'
                    : undefined,
              },
              {
                value: 'time',
                label: 'By time',
                ariaLabel: 'By time',
                disabled: timeCandidates.length === 0,
                title:
                  timeCandidates.length === 0
                    ? 'Requires a period column outside the regression design.'
                    : undefined,
              },
              {
                value: 'unit-and-time',
                label: 'By unit and time',
                ariaLabel: 'By unit and time',
                disabled: !groupingCandidates.some((unit) =>
                  timeCandidates.some((time) => time.id !== unit.id),
                ),
                title: !groupingCandidates.some((unit) =>
                  timeCandidates.some((time) => time.id !== unit.id),
                )
                  ? 'Requires distinct unit and period columns outside the regression design.'
                  : undefined,
              },
            ]}
          />
          <p className={cn(fieldHint, 'max-w-[65ch]')}>
            {describeFixedEffectsChoice(configuration.fixedEffects)}
          </p>
          {configuration.fixedEffects.kind !== 'none' && (
            <div className="mt-3">
              <ParameterLabel
                className={fieldLabel}
                label={configuration.fixedEffects.kind === 'time' ? 'Time column' : 'Unit column'}
                help={
                  configuration.fixedEffects.kind === 'time'
                    ? ESTIMATION_PARAMETER_HELP.adjustedRegression.timeColumn
                    : ESTIMATION_PARAMETER_HELP.adjustedRegression.unitColumn
                }
              />
              {(configuration.fixedEffects.kind === 'time' ? timeCandidates : groupingCandidates)
                .length === 0 ? (
                <p className={cn(fieldHint, 'max-w-[65ch]')}>
                  No prepared column is outside the design. Prepare the dataset again with the unit
                  column selected, and leave that column out of the graph.
                </p>
              ) : (
                <Select
                  aria-label={
                    configuration.fixedEffects.kind === 'time' ? 'Time column' : 'Unit column'
                  }
                  className={field('text', 'mt-1 w-full max-w-xs')}
                  value={configuration.fixedEffects.column}
                  onChange={(event) => {
                    const column = (
                      configuration.fixedEffects.kind === 'time'
                        ? timeCandidates
                        : groupingCandidates
                    ).find((candidate) => candidate.id === event.target.value)
                    if (column === undefined) return
                    if (
                      configuration.fixedEffects.kind === 'unit-and-time' &&
                      column.id === configuration.fixedEffects.timeColumn
                    )
                      return
                    const fixedEffects: FixedEffects =
                      configuration.fixedEffects.kind === 'unit-and-time'
                        ? {
                            ...configuration.fixedEffects,
                            column: column.id,
                            name: column.name,
                          }
                        : {
                            kind: configuration.fixedEffects.kind === 'time' ? 'time' : 'unit',
                            column: column.id,
                            name: column.name,
                          }
                    configure({ ...configuration, fixedEffects })
                  }}
                >
                  {(configuration.fixedEffects.kind === 'time'
                    ? timeCandidates
                    : groupingCandidates
                  )
                    .filter(
                      (candidate) =>
                        configuration.fixedEffects.kind !== 'unit-and-time' ||
                        candidate.id !== configuration.fixedEffects.timeColumn,
                    )
                    .map((candidate) => (
                      <option key={candidate.id} value={candidate.id}>
                        {candidate.name}
                      </option>
                    ))}
                </Select>
              )}
            </div>
          )}
          {configuration.fixedEffects.kind === 'unit-and-time' && (
            <div className="mt-3">
              <ParameterLabel
                className={fieldLabel}
                label="Time column"
                help={ESTIMATION_PARAMETER_HELP.adjustedRegression.timeColumn}
              />
              {timeCandidates.filter(
                (candidate) =>
                  configuration.fixedEffects.kind === 'unit-and-time' &&
                  candidate.id !== configuration.fixedEffects.column,
              ).length === 0 ? (
                <p className={cn(fieldHint, 'max-w-[65ch]')}>
                  No prepared column is outside the design besides the unit column. Prepare the
                  dataset again with the time column selected, and leave that column out of the
                  graph.
                </p>
              ) : (
                <Select
                  aria-label="Time column"
                  className={field('text', 'mt-1 w-full max-w-xs')}
                  value={configuration.fixedEffects.timeColumn}
                  onChange={(event) => {
                    const column = timeCandidates.find(
                      (candidate) => candidate.id === event.target.value,
                    )
                    if (column === undefined || configuration.fixedEffects.kind !== 'unit-and-time')
                      return
                    configure({
                      ...configuration,
                      fixedEffects: {
                        ...configuration.fixedEffects,
                        timeColumn: column.id,
                        timeName: column.name,
                      },
                    })
                  }}
                >
                  {timeCandidates
                    .filter(
                      (candidate) =>
                        configuration.fixedEffects.kind === 'unit-and-time' &&
                        candidate.id !== configuration.fixedEffects.column,
                    )
                    .map((candidate) => (
                      <option key={candidate.id} value={candidate.id}>
                        {candidate.name}
                      </option>
                    ))}
                </Select>
              )}
            </div>
          )}
        </div>
      </SettingsStep>
      <SettingsStep
        number={categoricalChecklist === null ? undefined : 3}
        title="Report uncertainty"
      >
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Errors"
            help={ESTIMATION_PARAMETER_HELP.adjustedRegression.interval}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Error treatment"
            value={configuration.errors.kind}
            onChange={(kind) => {
              const unit =
                configuration.fixedEffects.kind === 'none' ? null : configuration.fixedEffects
              const candidate = panelUnit ?? timeCandidates[0]
              const clusterBy =
                configuration.errors.kind === 'cluster'
                  ? configuration.errors
                  : candidate === undefined
                    ? unit
                    : { column: candidate.id, name: candidate.name }
              if (kind === 'cluster') {
                if (clusterBy === null) return
                configure({
                  ...configuration,
                  errors: { kind, column: clusterBy.column, name: clusterBy.name },
                })
              } else
                configure({
                  ...configuration,
                  errors:
                    kind === 'arma'
                      ? { kind, p: 1, q: 0, maxIter: DEFAULT_ARMA_ITERATIONS }
                      : { kind },
                })
            }}
            options={
              configuration.fixedEffects.kind !== 'none'
                ? [
                    { value: 'classical', label: 'Classical' },
                    { value: 'hc1', label: 'Robust (HC1)' },
                    {
                      value: 'cluster',
                      label: 'Clustered',
                      disabled: timeCandidates.length === 0,
                      title:
                        timeCandidates.length === 0
                          ? 'Requires a cluster column outside the regression design.'
                          : undefined,
                    },
                  ]
                : [
                    { value: 'classical', label: 'Classical' },
                    { value: 'hc1', label: 'Robust (HC1)' },
                    {
                      value: 'cluster',
                      label: 'Clustered',
                      disabled: timeCandidates.length === 0,
                      title:
                        timeCandidates.length === 0
                          ? 'Requires a cluster column outside the regression design.'
                          : undefined,
                    },
                    { value: 'hac', label: 'Newey–West HAC' },
                    { value: 'arma', label: 'ARMA errors' },
                  ]
            }
          />
          <p className={cn(fieldHint, 'max-w-[65ch]')}>
            {describeErrorTreatment(configuration.errors, configuration.fixedEffects)}
          </p>
          {configuration.errors.kind === 'hc1' && (
            <div className="formula mt-1 max-w-[65ch] text-body">
              <Formula {...hc1Factor(configuration.fixedEffects.kind !== 'none')} />
            </div>
          )}
          {configuration.errors.kind === 'hac' && (
            <div className="formula mt-1 max-w-[65ch] text-body">
              <Formula {...HAC_BANDWIDTH} />
            </div>
          )}
          {configuration.errors.kind === 'cluster' && (
            <div className="mt-3">
              <ParameterLabel
                className={fieldLabel}
                label="Cluster column"
                help={ESTIMATION_PARAMETER_HELP.adjustedRegression.clusterColumn}
              />
              {timeCandidates.length === 0 ? (
                <p className={cn(fieldHint, 'max-w-[65ch]')}>
                  No prepared column is outside the design. Prepare the dataset again with the
                  cluster column selected, and leave that column out of the graph.
                </p>
              ) : (
                <Select
                  aria-label="Cluster column"
                  className={field('text', 'mt-1 w-full max-w-xs')}
                  value={configuration.errors.column}
                  onChange={(event) => {
                    const column = timeCandidates.find(
                      (candidate) => candidate.id === event.target.value,
                    )
                    if (column !== undefined)
                      configure({
                        ...configuration,
                        errors: { kind: 'cluster', column: column.id, name: column.name },
                      })
                  }}
                >
                  {timeCandidates.map((candidate) => (
                    <option key={candidate.id} value={candidate.id}>
                      {candidate.name}
                    </option>
                  ))}
                </Select>
              )}
            </div>
          )}
          {configuration.errors.kind === 'arma' && (
            <div className="mt-3 grid gap-3 @md/panel:grid-cols-3">
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Autoregressive order"
                  help={ESTIMATION_PARAMETER_HELP.adjustedRegression.autoregressiveOrder}
                />
                <NumberInput
                  aria-label="Autoregressive order"
                  min={0}
                  max={MAX_ARMA_ORDER}
                  className={field('text', 'mt-1')}
                  value={configuration.errors.p}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      errors: {
                        kind: 'arma',
                        p: Number(event.target.value),
                        q: configuration.errors.kind === 'arma' ? configuration.errors.q : 0,
                        maxIter:
                          configuration.errors.kind === 'arma'
                            ? configuration.errors.maxIter
                            : DEFAULT_ARMA_ITERATIONS,
                      },
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Moving-average order"
                  help={ESTIMATION_PARAMETER_HELP.adjustedRegression.movingAverageOrder}
                />
                <NumberInput
                  aria-label="Moving-average order"
                  min={0}
                  max={MAX_ARMA_ORDER}
                  className={field('text', 'mt-1')}
                  value={configuration.errors.q}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      errors: {
                        kind: 'arma',
                        p: configuration.errors.kind === 'arma' ? configuration.errors.p : 1,
                        q: Number(event.target.value),
                        maxIter:
                          configuration.errors.kind === 'arma'
                            ? configuration.errors.maxIter
                            : DEFAULT_ARMA_ITERATIONS,
                      },
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Optimiser iterations"
                  help={ESTIMATION_PARAMETER_HELP.adjustedRegression.armaIterations}
                />
                <NumberInput
                  aria-label="Optimiser iterations"
                  min={1}
                  className={field('text', 'mt-1')}
                  value={configuration.errors.maxIter}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      errors: {
                        kind: 'arma',
                        p: configuration.errors.kind === 'arma' ? configuration.errors.p : 1,
                        q: configuration.errors.kind === 'arma' ? configuration.errors.q : 0,
                        maxIter: Number(event.target.value),
                      },
                    })
                  }
                />
              </label>
            </div>
          )}
        </div>
      </SettingsStep>
    </div>
  )
}
