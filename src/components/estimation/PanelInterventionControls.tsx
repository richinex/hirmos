import { useJob } from '@/analysis/JobsProvider'
import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { button, field, fieldHint, fieldLabel, fieldRow, stepsStack } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { conventionalDidPlaceboUnavailable } from '@/domain/conventionalDid'
import type { ColumnId, DatasetProfile, PhysicalColumnProfile } from '@/domain/dataset'
import { type EstimatorConfiguration } from '@/domain/estimation'
import { type PanelInterventionPreflight } from '@/domain/panel'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'
import { type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { defaultStaggeredSpecification } from '@/domain/staggeredDid'
import { type StudySpecification } from '@/domain/study'
import { defaultSunAbraham } from '@/domain/sunAbraham'
import { cn } from '@/lib/utils'
import { type ReactNode } from 'react'
import { AdjustedDidControls } from './AdjustedDidControls'
import { ConventionalDidControls } from './ConventionalDidControls'
import type { DidCovariateCheck } from './DidCovariateChecklist'
import { StaggeredDidControls } from './StaggeredDidControls'
import { SunAbrahamControls } from './SunAbrahamControls'
import { useSunAbrahamPanel } from './useSunAbrahamPanel'

type Configuration = Extract<EstimatorConfiguration, { kind: 'panel-intervention' }>

type Props = {
  readonly configuration: Configuration
  readonly configure: (next: Configuration) => void
  readonly prepared: PreparedDatasetArtifact
  readonly study: StudySpecification | null
  readonly onOpenBacon: ((outcome: ColumnId, treatment: ColumnId) => void) | undefined
  readonly job: ReturnType<typeof useJob>['job']
  readonly session: Pick<ReturnType<typeof useJob>, 'blocked'>
  readonly sunPanel: ReturnType<typeof useSunAbrahamPanel>
  readonly didCandidates: readonly PhysicalColumnProfile[]
  readonly didRestrictions: ReadonlyMap<ColumnId, string>
  readonly didCheck: DidCovariateCheck
  readonly profile: DatasetProfile
  readonly panelPreflight: PanelInterventionPreflight
}

export function PanelInterventionControls({
  configuration,
  configure,
  prepared,
  study,
  onOpenBacon,
  job,
  session,
  sunPanel,
  didCandidates,
  didRestrictions,
  didCheck,
  profile,
  panelPreflight,
}: Props): ReactNode {
  return (
    <div className={stepsStack}>
      <SettingsStep number={1} title="Choose the method" className="[column-span:all]">
        <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
          Choose the method before fitting. Two-period regression and DR require one period before
          and one after adoption. Conventional DiD can also compare averages across multiple pre-
          and post-periods.
        </p>
        <SegmentedControl
          className="justify-self-start"
          ariaLabel="Panel method"
          value={
            configuration.primary === 'adjusted'
              ? configuration.specification.kind
              : (configuration.primary ?? 'syntheticDid')
          }
          onChange={(primary) =>
            configure(
              primary === 'sunAbraham'
                ? defaultSunAbraham
                : primary === 'staggered'
                  ? {
                      kind: 'panel-intervention',
                      primary: 'staggered',
                      covariates: [],
                      specification: defaultStaggeredSpecification,
                      clustering: { kind: 'unit' },
                    }
                  : primary === 'regression' || primary === 'doublyRobust'
                    ? {
                        kind: 'panel-intervention',
                        primary: 'adjusted',
                        covariates: [],
                        specification:
                          primary === 'regression'
                            ? { kind: 'regression' }
                            : {
                                kind: 'doublyRobust',
                                folds: 2,
                                seed: 1234,
                                trimming: 0.01,
                                normalization: 'in-sample',
                              },
                      }
                    : {
                        kind: 'panel-intervention',
                        primary,
                        placeboReplications: 100,
                        seed: 0,
                      },
            )
          }
          options={[
            { value: 'did', label: 'Conventional' },
            { value: 'regression', label: 'Two-period regression' },
            { value: 'doublyRobust', label: 'Two-period DR' },
            { value: 'syntheticDid', label: 'Synthetic' },
            { value: 'staggered', label: 'Staggered adoption' },
            { value: 'sunAbraham', label: 'Sun–Abraham' },
          ]}
        />
        {prepared.kind === 'prepared-panel' && study !== null && onOpenBacon !== undefined && (
          <div className="space-y-1">
            <button
              type="button"
              className={button('quiet')}
              disabled={job.kind === 'running' || session.blocked}
              onClick={() => onOpenBacon(study.outcome.column, study.treatment.column)}
            >
              Decompose the TWFE coefficient
            </button>
            <p className={fieldHint}>
              Inspect the weighted comparisons in a two-way fixed-effects regression. This
              diagnostic does not estimate a separate ATT.
            </p>
          </div>
        )}
      </SettingsStep>
      {configuration.primary === 'sunAbraham' && (
        <SunAbrahamControls configuration={configuration} onChange={configure} panel={sunPanel} />
      )}
      {configuration.primary === 'staggered' && (
        <SettingsStep number={2} title="Compare cohorts">
          <StaggeredDidControls
            section="comparison"
            configuration={configuration}
            candidates={didCandidates}
            restrictions={didRestrictions}
            check={didCheck}
            clusterCandidates={profile.columns.filter(
              (c) =>
                c.id !== study?.treatment.column &&
                c.id !== study?.outcome.column &&
                (prepared.kind !== 'prepared-panel' ||
                  (c.id !== prepared.panel.unitColumn && c.id !== prepared.panel.timeColumn)),
            )}
            onChange={configure}
          />
        </SettingsStep>
      )}
      {configuration.primary === 'staggered' && (
        <SettingsStep number={3} title="Report the result">
          <StaggeredDidControls
            section="reporting"
            configuration={configuration}
            candidates={didCandidates}
            clusterCandidates={profile.columns.filter(
              (c) =>
                c.id !== study?.treatment.column &&
                c.id !== study?.outcome.column &&
                (prepared.kind !== 'prepared-panel' ||
                  (c.id !== prepared.panel.unitColumn && c.id !== prepared.panel.timeColumn)),
            )}
            onChange={configure}
          />
        </SettingsStep>
      )}
      {configuration.primary === 'adjusted' && (
        <SettingsStep number={2} title="Adjust for covariates">
          <AdjustedDidControls
            configuration={configuration}
            candidates={didCandidates}
            onChange={configure}
            restrictions={didRestrictions}
            check={didCheck}
          />
        </SettingsStep>
      )}
      {configuration.primary === 'did' && (
        <SettingsStep number={2} title="Report uncertainty">
          <ConventionalDidControls
            inference={configuration.inference}
            placeboUnavailable={
              panelPreflight.kind === 'ready'
                ? conventionalDidPlaceboUnavailable({
                    treatedUnits: panelPreflight.layout.treated.length,
                    controlUnits: panelPreflight.layout.controls.length,
                  })
                : null
            }
            onChange={(inference) => configure({ ...configuration, inference })}
          />
        </SettingsStep>
      )}
      {configuration.primary !== 'did' &&
        configuration.primary !== 'adjusted' &&
        configuration.primary !== 'staggered' &&
        configuration.primary !== 'sunAbraham' && (
          <SettingsStep number={2} title="Report uncertainty">
            <div className={fieldRow.two}>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Placebo replications"
                  help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboReplications}
                />
                <NumberInput
                  min={2}
                  max={2000}
                  aria-label="Panel placebo replications"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.placeboReplications}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      placeboReplications: Math.max(
                        2,
                        Math.min(2000, Math.floor(Number(event.target.value) || 2)),
                      ),
                    })
                  }
                />
              </label>
              <label className="block">
                <ParameterLabel
                  className={fieldLabel}
                  label="Placebo seed"
                  help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboSeed}
                />
                <NumberInput
                  min={0}
                  max={0xffff_ffff}
                  aria-label="Panel placebo seed"
                  className={field('text', 'mt-1 w-full')}
                  value={configuration.seed}
                  onChange={(event) =>
                    configure({
                      ...configuration,
                      seed: Math.max(
                        0,
                        Math.min(0xffff_ffff, Math.floor(Number(event.target.value) || 0)),
                      ),
                    })
                  }
                />
              </label>
            </div>
          </SettingsStep>
        )}
    </div>
  )
}
