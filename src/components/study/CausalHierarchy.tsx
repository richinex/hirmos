import { label } from '@/components/ui/recipes'

/** One shared explanation of what changes between association, intervention and counterfactual questions. */
export function CausalHierarchy() {
  return (
    <details className="rounded-xl border border-hair bg-panel px-4 py-3 text-body">
      <summary className="cursor-pointer text-ink">Causal hierarchy and the data-generating process</summary>
      <p className="mb-3 mt-2 max-w-[70ch] text-muted">The data-generating process is the set of mechanisms assumed to produce the observed variables. A DAG records claims about those mechanisms. Discovery results provide statistical evidence about possible structure; they do not establish the DAG by themselves.</p>
      <dl className="m-0 grid gap-3 @lg/panel:grid-cols-3">
        <div><dt className={label('text-faint')}>1 · Association</dt><dd className="m-0 mt-1 text-muted">Describes or predicts observed relationships. Granger tests and discovery algorithms operate here unless additional causal assumptions justify a stronger interpretation.</dd></div>
        <div><dt className={label('text-faint')}>2 · Intervention</dt><dd className="m-0 mt-1 text-muted">Asks how an outcome distribution changes under do(T=t). Identification combines the DAG and design assumptions with an observed-data functional before an estimator is fitted.</dd></div>
        <div><dt className={label('text-faint')}>3 · Counterfactual</dt><dd className="m-0 mt-1 text-muted">Compares alternative treatment values for the same unit. This requires a structural model that links the unit across treatment worlds, in addition to assumptions used for intervention effects.</dd></div>
      </dl>
    </details>
  )
}
