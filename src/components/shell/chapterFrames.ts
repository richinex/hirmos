import type { ChapterId } from '@/domain/navigation'

export interface ChapterHead {
  readonly title: string
  readonly intro: string
  readonly introLayout: 'columns' | 'single'
  /** A help button sits beside the heading. */
  readonly help: boolean
}

export const CHAPTER_HEADS = {
  dag: {
    title: 'DAG workspace',
    intro:
      'A directed acyclic graph (DAG) represents assumptions about how data is generated. Nodes represent variables, and each arrow states an assumed direct causal relationship. In this section, build the graph for your causal question and record the reason for each arrow. Discovery results can contribute evidence, but they do not determine the graph.',
    introLayout: 'columns',
    help: false,
  },
  study: {
    title: 'Study design',
    intro:
      'A causal question specifies the treatment, outcome, intervention contrast, effect measure, and target population. In this section, you represent the data-generating process with a DAG, enumerate measured back-door adjustment sets, and run the ID algorithm to determine whether the interventional distribution can be written using observed probabilities.',
    introLayout: 'columns',
    help: false,
  },
  estimation: {
    title: 'Estimation',
    intro:
      'Identification determines how to express the causal question using observed data. Estimation applies a statistical method to that expression. In this section, you choose a compatible estimator and examine the effect estimate, its uncertainty, and the method-specific diagnostics.',
    introLayout: 'columns',
    help: true,
  },
  sensitivity: {
    title: 'Sensitivity',
    intro:
      "A sensitivity analysis examines how an estimate changes when a specified part of the analysis is perturbed. In this section, apply procedures supported by the selected estimator and interpret each result against that procedure's reference value. Please note that stability under one perturbation does not assess the remaining assumptions.",
    introLayout: 'columns',
    help: false,
  },
  counterfactual: {
    title: 'Counterfactuals',
    intro:
      'Counterfactual reasoning examines what would have occurred if an action or condition had differed, given the actual outcome. It involves considering a parallel scenario for the same individual, unit, or system. In this context, a causal model is used to estimate the counterfactual outcome. The result depends on the chosen model and its underlying assumptions.',
    introLayout: 'columns',
    help: false,
  },
  decisions: {
    title: 'Decision lab',
    intro:
      'A policy sets the action to take at each decision, using only what is known when the decision is made. In this section, you mark the decisions on a causal graph in the order they are made, choose what each decision may read, and choose the objective. The checks report whether recorded data identify the value of every such policy. You can then compare what the log says about each action with its natural bounds and with randomised rows.',
    introLayout: 'columns',
    help: true,
  },
  'root-cause': {
    title: 'Causal model analysis',
    intro:
      'A causal model encodes assumptions about how variables influence one another and the mechanisms that generate their values. It distinguishes between observing a value and intervening to set a value. In this section, we fit the mechanisms to the data and examine how interventions or changes propagate through the system. The results depend on the graph structure, the fitted mechanisms, and the underlying assumptions.',
    introLayout: 'columns',
    help: false,
  },
  'time-series': {
    title: 'Time-series analysis',
    intro:
      'Analyse how observations change over time and how current values relate to earlier values. Estimate short-run dynamics and long-run relationships between series.',
    introLayout: 'single',
    help: false,
  },
  survival: {
    title: 'Survival analysis',
    intro:
      'Use survival analysis when the outcome is the time until an event. Fit an event-time distribution, compare two observed groups during follow-up, or estimate transitions between states. A row with no observed event by the end of follow-up is right-censored at its recorded duration. These analyses do not require a DAG and are not added to the causal-estimation ledger.',
    introLayout: 'columns',
    help: false,
  },
} as const satisfies Partial<Record<ChapterId, ChapterHead>>

type WorkbenchChapter = keyof typeof CHAPTER_HEADS

/** What fills the stage under the heading: a form card, the graph canvas, or numbered steps on the stage itself. */
export type StageBody = 'card' | 'canvas' | 'steps'

export interface PaneFrame {
  readonly title: string
  readonly trigger?: { readonly label: string; readonly icon: string }
}

export interface BottomFrame extends PaneFrame {
  readonly defaultSize?: number
  readonly defaultCollapsed?: boolean
}

/** The layout a chapter mounts into, known before its code arrives. */
export type ChapterFrame =
  | {
      readonly kind: 'workbench'
      readonly layoutId: string
      readonly head: ChapterHead
      readonly body: StageBody
      readonly inspector: PaneFrame
      readonly bottom: BottomFrame
    }
  | { readonly kind: 'desk' }
  | { readonly kind: 'page' }

const requirements = (title: string): PaneFrame => ({
  title,
  trigger: { label: 'Requirements', icon: 'contract' },
})

const history = (title: string, extra: Partial<BottomFrame> = {}): BottomFrame => ({
  title,
  trigger: { label: 'History', icon: 'history' },
  defaultSize: 150,
  ...extra,
})

const untitled = (title: string): BottomFrame => ({ title, defaultSize: 150 })

const workbench = (
  chapter: WorkbenchChapter,
  body: StageBody,
  inspector: PaneFrame,
  bottom: BottomFrame,
  layoutId: string = chapter,
): ChapterFrame => ({
  kind: 'workbench',
  layoutId,
  head: CHAPTER_HEADS[chapter],
  body,
  inspector,
  bottom,
})

const FRAMES: Partial<Record<ChapterId, ChapterFrame>> = {
  dag: workbench('dag', 'canvas', { title: 'Inspector' }, untitled('Arrows')),
  study: workbench(
    'study',
    'card',
    requirements('Graph, roles, and method requirements'),
    history('Studies'),
  ),
  estimation: workbench(
    'estimation',
    'card',
    requirements('Study and method requirements'),
    history('Runs'),
  ),
  sensitivity: workbench(
    'sensitivity',
    'card',
    requirements('Estimate and method requirements'),
    history('Probes'),
  ),
  counterfactual: workbench(
    'counterfactual',
    'card',
    requirements('Study and method requirements'),
    history('Runs'),
  ),
  decisions: workbench(
    'decisions',
    'steps',
    requirements('Decision and method requirements'),
    history('Runs'),
  ),
  'root-cause': workbench(
    'root-cause',
    'card',
    requirements('Data and method requirements'),
    history('Run history', { defaultSize: undefined, defaultCollapsed: true }),
  ),
  'time-series': workbench(
    'time-series',
    'steps',
    requirements('Data and method requirements'),
    history('Count-model runs'),
    'time-series-count',
  ),
  survival: workbench(
    'survival',
    'card',
    requirements('Data and method requirements'),
    history('Survival runs'),
  ),
  results: { kind: 'desk' },
}

export const chapterFrame = (chapter: ChapterId): ChapterFrame =>
  FRAMES[chapter] ?? { kind: 'page' }
