import { formatSetPercent, formatSetting } from '@/lib/format/number'
import type {
  CounterfactualConfiguration,
  DynamicCounterfactualBlockLength,
  DynamicCounterfactualUncertainty,
  DynamicInterventionSchedule,
  DynamicLinearScmConfiguration,
} from './counterfactual'
import { assertNever } from './dop'
import type { ConfigurationSetting } from './estimation'

export const counterfactualModelLabels: Readonly<
  Record<CounterfactualConfiguration['kind'], string>
> = {
  'linear-scm': 'Row-wise SCM',
  'dynamic-linear-scm': 'Dynamic SCM',
}

export const interventionScheduleLabels: Readonly<
  Record<DynamicInterventionSchedule['kind'], string>
> = {
  point: 'One time',
  persistent: 'Persistent',
}

export const disturbanceInferenceLabels: Readonly<Record<'exact' | 'noise', string>> = {
  exact: 'Exactly',
  noise: 'Allow observation noise',
}

export const counterfactualUncertaintyLabels: Readonly<
  Record<DynamicCounterfactualUncertainty['kind'], string>
> = {
  blockBootstrap: 'Block-bootstrap interval',
  none: 'Point estimate only',
}

export const blockLengthLabels: Readonly<Record<DynamicCounterfactualBlockLength['kind'], string>> =
  {
    cubeRoot: 'Automatic',
    fixed: 'Fixed',
  }

export const firstInterventionLabel = (treatment: string | null): string =>
  `Set ${treatment ?? 'treatment'} to`

const setting = (key: string, label: string, value: string): ConfigurationSetting => ({
  key,
  label,
  value,
})

const number = (value: number): string => formatSetting(value).text

/** A counterfactual configuration in the words the Counterfactuals page uses, for proposals and records. */
export function counterfactualSettings(
  configuration: CounterfactualConfiguration,
  treatment: string | null,
): readonly ConfigurationSetting[] {
  const shared = [
    setting('kind', 'Structural model', counterfactualModelLabels[configuration.kind]),
    setting(
      'interventions.0',
      firstInterventionLabel(treatment),
      number(configuration.interventions[0]),
    ),
    setting('interventions.1', 'and to', number(configuration.interventions[1])),
  ]
  switch (configuration.kind) {
    case 'linear-scm': {
      const { observationNoise } = configuration
      return [
        ...shared,
        setting(
          'observationNoise',
          'Infer disturbance terms',
          disturbanceInferenceLabels[observationNoise === null ? 'exact' : 'noise'],
        ),
        ...(observationNoise === null
          ? []
          : [setting('observationNoise.scale', 'Noise scale', number(observationNoise))]),
      ]
    }
    case 'dynamic-linear-scm':
      return [...shared, ...dynamicSettings(configuration)]
    default:
      return assertNever(configuration)
  }
}

function dynamicSettings(
  configuration: DynamicLinearScmConfiguration,
): readonly ConfigurationSetting[] {
  const { schedule, uncertainty } = configuration
  return [
    setting('schedule', 'Intervention schedule', interventionScheduleLabels[schedule.kind]),
    schedule.kind === 'point'
      ? setting('schedule.row', 'Intervention row', number(schedule.row))
      : setting('schedule.startRow', 'First intervention row', number(schedule.startRow)),
    setting('steps', 'Horizon points', number(configuration.steps)),
    setting(
      'uncertainty',
      'Sampling uncertainty',
      counterfactualUncertaintyLabels[uncertainty.kind],
    ),
    ...(uncertainty.kind === 'blockBootstrap'
      ? [
          setting('uncertainty.samples', 'Bootstrap refits', number(uncertainty.samples)),
          setting(
            'uncertainty.blockLength',
            'Block length',
            blockLengthLabels[uncertainty.blockLength.kind],
          ),
          ...(uncertainty.blockLength.kind === 'fixed'
            ? [
                setting(
                  'uncertainty.blockLength.length',
                  'Points per block',
                  number(uncertainty.blockLength.length),
                ),
              ]
            : []),
          setting(
            'uncertainty.confidenceLevel',
            'Confidence level',
            formatSetPercent(uncertainty.confidenceLevel).text,
          ),
          setting('uncertainty.seed', 'Seed', number(uncertainty.seed)),
        ]
      : []),
  ]
}
