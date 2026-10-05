import { useMemo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { impactEffectPathOption } from '@/charts/estimation/impactEffectPath'
import { useChartTheme } from '@/charts/theme'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import {
  impactCumulativePath,
  impactPointwisePath,
  type CausalImpactEvidence,
} from '@/domain/estimation'

type View = 'pointwise' | 'cumulative'

/** The pointwise and cumulative differences, for either inference route. */
export function ImpactEffectPanel({
  evidence,
  stepLabel,
}: {
  readonly evidence: CausalImpactEvidence
  readonly stepLabel: string
}) {
  const theme = useChartTheme()
  const [view, setView] = useState<View>('pointwise')
  const option = useMemo(() => {
    const points =
      view === 'pointwise' ? impactPointwisePath(evidence) : impactCumulativePath(evidence)
    return impactEffectPathOption(
      {
        label: view === 'pointwise' ? 'Pointwise effect' : 'Cumulative effect',
        points,
        stepLabel,
        evaluatedFrom: evidence.nPre + 1,
        evaluatedTo: evidence.postEnd,
      },
      theme,
    )
  }, [evidence, view, stepLabel, theme])
  return (
    <div className="mt-3 grid gap-3">
      <SegmentedControl
        ariaLabel="Impact effect plot"
        value={view}
        onChange={setView}
        options={[
          { value: 'pointwise', label: 'Pointwise' },
          { value: 'cumulative', label: 'Cumulative' },
        ]}
      />
      <ExpandableChart
        option={option}
        label="Impact effects"
        className="h-[280px]"
        testId="impact-effects"
      />
    </div>
  )
}
