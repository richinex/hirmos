import { NumberInput } from '@/components/ui/NumberInput'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { SettingsDisclosure } from '@/components/ui/SettingsDisclosure'
import { field, fieldHint, fieldLabel, fieldRow, settingsStack } from '@/components/ui/recipes'
import { CAUSAL_FOREST_COPY, type CausalForestConfiguration } from '@/domain/causalForest'

export function CausalForestControls({
  configuration,
  onChange,
}: {
  readonly configuration: CausalForestConfiguration
  readonly onChange: (configuration: CausalForestConfiguration) => void
}) {
  const update = (patch: Partial<CausalForestConfiguration>) =>
    onChange({ ...configuration, ...patch })
  const number = (
    label: string,
    value: number,
    change: (value: number) => void,
    help: string,
    min: number,
    step = 1,
    max?: number,
  ) => (
    <label key={label}>
      <ParameterLabel className={fieldLabel} label={label} help={help} />
      <NumberInput
        aria-label={label}
        className={field('text', 'mt-1 w-full')}
        value={Number.isNaN(value) ? '' : value}
        min={min}
        max={max}
        step={step}
        onChange={(event) => change(event.target.value === '' ? NaN : Number(event.target.value))}
      />
    </label>
  )
  return (
    <div className={settingsStack} data-testid="causal-forest-controls">
      <div className={fieldRow.three}>
        {number(
          'Trees',
          configuration.trees,
          (trees) => update({ trees }),
          CAUSAL_FOREST_COPY.trees,
          2,
        )}
        {number(
          'Forest seed',
          configuration.seed,
          (seed) => update({ seed }),
          'The seed makes the forest sampling reproducible for the same data and settings.',
          0,
          1,
          0xffff_ffff,
        )}
        {number(
          'Confidence level',
          configuration.confidenceLevel,
          (confidenceLevel) => update({ confidenceLevel }),
          CAUSAL_FOREST_COPY.intervals,
          0.01,
          0.01,
          0.999,
        )}
      </div>
      <div>
        <ParameterLabel
          className={fieldLabel}
          label="Parameter tuning"
          help="Select forest parameters using out-of-bag prediction errors. Tuning does not change the number of trees in the final forest."
        />
        <SegmentedControl
          className="mt-1"
          ariaLabel="Forest parameter tuning"
          value={configuration.tuning.kind}
          onChange={(kind) =>
            update({
              tuning:
                kind === 'disabled' ? { kind } : { kind, trees: 200, repetitions: 50, draws: 1000 },
            })
          }
          options={[
            { value: 'disabled', label: 'Specified settings' },
            { value: 'all', label: 'Automatic tuning' },
          ]}
        />
      </div>
      <SettingsDisclosure
        title="Forest settings"
        items={[
          { icon: 'account_tree', text: `${configuration.trees} trees` },
          {
            icon: 'call_split',
            text:
              configuration.honesty.kind === 'enabled' ? 'Honest splitting' : 'Honesty disabled',
          },
        ]}
      >
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Honesty"
            help={CAUSAL_FOREST_COPY.honesty}
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Forest honesty"
            value={configuration.honesty.kind}
            onChange={(kind) =>
              update({
                honesty: kind === 'disabled' ? { kind } : { kind, fraction: 0.5, prune: true },
              })
            }
            options={[
              { value: 'enabled', label: 'Separate samples' },
              { value: 'disabled', label: 'Same sample' },
            ]}
          />
        </div>
        <div className={fieldRow.three}>
          {number(
            'Sample fraction',
            configuration.sampleFraction,
            (sampleFraction) => update({ sampleFraction }),
            'The fraction of observations sampled for each tree. Grouped variance estimation requires a fraction no greater than 0.5.',
            0.01,
            0.01,
            0.5,
          )}
          {number(
            'Minimum node size',
            configuration.minimumNodeSize,
            (minimumNodeSize) => update({ minimumNodeSize }),
            'The target minimum number of observations in a leaf. Some leaves may contain fewer observations.',
            1,
          )}
          {number(
            'Trees per variance group',
            configuration.groupSize,
            (groupSize) => update({ groupSize }),
            'The number of trees grown on each group subsample. Confidence intervals require at least 2.',
            2,
          )}
        </div>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Variables per split"
            help="Automatic uses the smaller of the covariate count and the rounded-up square root of that count plus 20."
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Forest variables per split"
            value={configuration.variablesPerSplit.kind}
            onChange={(kind) =>
              update({ variablesPerSplit: kind === 'automatic' ? { kind } : { kind, count: 1 } })
            }
            options={[
              { value: 'automatic', label: 'Automatic' },
              { value: 'specified', label: 'Specify count' },
            ]}
          />
          {configuration.variablesPerSplit.kind === 'specified' && (
            <div className="mt-3 max-w-xs">
              {number(
                'Variable count',
                configuration.variablesPerSplit.count,
                (count) => update({ variablesPerSplit: { kind: 'specified', count } }),
                'The number of candidate covariates considered for a split.',
                1,
              )}
            </div>
          )}
        </div>
        {configuration.honesty.kind === 'enabled' && (
          <div className={fieldRow.two}>
            {number(
              'Splitting fraction',
              configuration.honesty.fraction,
              (fraction) => {
                if (configuration.honesty.kind === 'enabled')
                  update({ honesty: { ...configuration.honesty, fraction } })
              },
              'The fraction of each sampled set used to choose splits. The remainder estimates leaf effects.',
              0.01,
              0.01,
              0.99,
            )}
            <div>
              <ParameterLabel
                className={fieldLabel}
                label="Empty leaves"
                help="Pruning removes empty estimation leaves. Without pruning, a tree with an empty leaf does not contribute to that prediction."
              />
              <SegmentedControl
                className="mt-1"
                ariaLabel="Forest empty leaves"
                value={configuration.honesty.prune ? 'prune' : 'retain'}
                onChange={(value) => {
                  if (configuration.honesty.kind === 'enabled')
                    update({ honesty: { ...configuration.honesty, prune: value === 'prune' } })
                }}
                options={[
                  { value: 'prune', label: 'Prune' },
                  { value: 'retain', label: 'Retain' },
                ]}
              />
            </div>
          </div>
        )}
        <div className={fieldRow.two}>
          {number(
            'Split balance',
            configuration.alpha,
            (alpha) => update({ alpha }),
            'The constraint on the imbalance of candidate splits.',
            0,
            0.01,
            0.49,
          )}
          {number(
            'Imbalance penalty',
            configuration.imbalancePenalty,
            (imbalancePenalty) => update({ imbalancePenalty }),
            'The penalty applied to imbalanced splits.',
            0,
            0.1,
          )}
        </div>
        <div>
          <ParameterLabel
            className={fieldLabel}
            label="Treatment-aware split balance"
            help="Account for treatment values when assessing split balance."
          />
          <SegmentedControl
            className="mt-1"
            ariaLabel="Treatment-aware split balance"
            value={configuration.stabilizeSplits ? 'enabled' : 'disabled'}
            onChange={(value) => update({ stabilizeSplits: value === 'enabled' })}
            options={[
              { value: 'enabled', label: 'Enabled' },
              { value: 'disabled', label: 'Disabled' },
            ]}
          />
        </div>
        {configuration.tuning.kind === 'all' && (
          <p className={fieldHint}>
            Automatic tuning selects the sample fraction, variables per split, node size and
            split-balance settings. With honesty enabled, it also selects the splitting fraction and
            pruning setting. The result records the selected values.
          </p>
        )}
      </SettingsDisclosure>
      <SettingsDisclosure
        title="Importance-selected refit"
        items={[
          {
            icon: 'tune',
            text:
              configuration.refit === undefined ? 'Not requested' : 'Reuse nuisance predictions',
          },
        ]}
      >
        <p className={fieldHint}>
          Fit outcome and treatment forests using all adjustment covariates. An initial causal
          forest selects covariates with importance above the mean. The final causal forest uses
          those covariates and retains the original nuisance predictions. This does not remove
          variables from the causal adjustment set.
        </p>
        <SegmentedControl
          className="justify-self-start"
          ariaLabel="Forest importance refit"
          value={configuration.refit === undefined ? 'off' : 'on'}
          options={[
            { value: 'off', label: 'Not requested' },
            { value: 'on', label: 'Select and refit' },
          ]}
          onChange={(value) => {
            if (value === 'off') {
              const { refit: _, ...rest } = configuration
              onChange(rest)
            } else
              update({
                refit: {
                  nuisanceTrees: 2000,
                  initialTrees: 2000,
                  outcomeSeed: 1,
                  treatmentSeed: 2,
                  initialSeed: 3,
                },
              })
          }}
        />
        {configuration.refit !== undefined && (
          <>
            <p className={fieldHint}>
              Preliminary forests use their default splitting settings. The controls above apply to
              the final forest. Choose automatic tuning to tune that final fit.
            </p>
            <div className={fieldRow.two}>
              {(
                [
                  'nuisanceTrees',
                  'initialTrees',
                  'outcomeSeed',
                  'treatmentSeed',
                  'initialSeed',
                ] as const
              ).map((key) =>
                number(
                  {
                    nuisanceTrees: 'Nuisance trees',
                    initialTrees: 'Initial forest trees',
                    outcomeSeed: 'Outcome forest seed',
                    treatmentSeed: 'Treatment forest seed',
                    initialSeed: 'Initial forest seed',
                  }[key],
                  configuration.refit![key],
                  (value) => {
                    if (configuration.refit !== undefined)
                      update({ refit: { ...configuration.refit, [key]: value } })
                  },
                  'Saved with this two-stage fitting specification.',
                  key === 'initialTrees' ? 2 : key === 'nuisanceTrees' ? 1 : 0,
                ),
              )}
            </div>
          </>
        )}
      </SettingsDisclosure>
      {configuration.tuning.kind === 'all' && (
        <SettingsDisclosure
          title="Tuning budget"
          items={[{ icon: 'tune', text: `${configuration.tuning.repetitions} candidate forests` }]}
        >
          <div className={fieldRow.three}>
            {number(
              'Trees per candidate',
              configuration.tuning.trees,
              (trees) => {
                if (configuration.tuning.kind === 'all')
                  update({ tuning: { ...configuration.tuning, trees } })
              },
              'The number of trees in each candidate forest used to fit the tuning model.',
              1,
            )}
            {number(
              'Candidate forests',
              configuration.tuning.repetitions,
              (repetitions) => {
                if (configuration.tuning.kind === 'all')
                  update({ tuning: { ...configuration.tuning, repetitions } })
              },
              'The number of forests used to fit the tuning model.',
              1,
            )}
            {number(
              'Parameter draws',
              configuration.tuning.draws,
              (draws) => {
                if (configuration.tuning.kind === 'all')
                  update({ tuning: { ...configuration.tuning, draws } })
              },
              'The number of parameter combinations evaluated by the fitted tuning model.',
              1,
            )}
          </div>
        </SettingsDisclosure>
      )}
    </div>
  )
}
