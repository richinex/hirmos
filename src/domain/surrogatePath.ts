import { z } from 'zod'
import { surrogateRequestSchema, surrogateEvidenceSchema, type SurrogateEvidence } from './surrogate'

const finite = z.number().finite()
const count = z.number().int().safe().positive()
const label = z.string().min(1).refine(s => s.trim() === s && s.length > 0, 'Use a non-empty label without surrounding spaces.')
const labels = z.array(label).min(1).readonly().refine(v => new Set(v).size === v.length, 'Period labels must be distinct.')
const coefficient = z.object({estimate:finite,standardError:finite.positive(),tStatistic:finite,residualDegreesOfFreedom:count}).strict()
function sameUncertainty(a:SurrogateEvidence['uncertainty'],b:SurrogateEvidence['uncertainty']):boolean {
  return a.kind==='none' ? b.kind==='none' : b.kind==='bootstrapStandardError' && a.repetitions===b.repetitions && a.seed===b.seed
}

/** Column order is explicit. No chronology is inferred from a column name. */
export const surrogatePathRequestSchema = z.discriminatedUnion('kind', [
  z.object({kind:z.literal('observedOutcomes'),outcomes:z.array(z.array(finite).min(1).readonly()).min(3).readonly(),
    treatment:z.array(z.union([z.literal(0),z.literal(1)])).min(3).readonly(),labels,uncertainty:surrogateRequestSchema.unwrap().shape.uncertainty}).strict(),
  z.object({kind:z.literal('surrogateWindows'),model:surrogateRequestSchema,
    windows:z.array(z.object({label,surrogateColumns:count}).strict().readonly()).min(1).readonly()}).strict(),
]).superRefine((r,ctx)=>{
  if(r.kind==='observedOutcomes') {
    if(r.outcomes.length!==r.treatment.length || r.outcomes.some(row=>row.length!==r.labels.length) || new Set(r.treatment).size!==2)
      ctx.addIssue({code:'custom',message:'Every experimental participant needs all selected outcomes, and both treatment groups must be present.'})
  } else {
    if(r.windows.length===0) return
    if(new Set(r.windows.map(w=>w.label)).size!==r.windows.length ||
       r.windows.some((w,i)=>i>0 && w.surrogateColumns<=r.windows[i-1]!.surrogateColumns) ||
       r.windows.at(-1)!.surrogateColumns!==r.model.experimental.surrogates[0]?.length)
      ctx.addIssue({code:'custom',message:'Surrogate windows require distinct labels and increasing column counts, ending with all selected surrogates.'})
  }
}).readonly()
export type SurrogatePathRequest = z.infer<typeof surrogatePathRequestSchema>

export const surrogatePathEvidenceSchema = z.discriminatedUnion('kind',[
  z.object({kind:z.literal('observedOutcomes'),experimentalRows:count,treatedRows:count,controlRows:count,
    periods:z.array(z.object({label,controlMean:finite,treatedMean:finite,contrast:finite,cumulativeMeanContrast:finite}).strict().readonly()).min(1).readonly(),
    participantBenchmark:coefficient,
    // Older saved paths did not record path uncertainty, even when the index was bootstrapped.
    cumulativeUncertainty:z.discriminatedUnion('kind',[
      z.object({kind:z.literal('notRecorded')}).strict(),
      z.object({kind:z.literal('none')}).strict(),
      z.object({kind:z.literal('bootstrapStandardErrors'),standardErrors:z.array(finite.nonnegative()).min(1).readonly(),repetitions:count.min(2),seed:z.number().int().min(0).max(0xffffffff)}).strict(),
    ]).readonly().default({kind:'notRecorded'})}).strict(),
  z.object({kind:z.literal('surrogateWindows'),windows:z.array(z.object({label,evidence:surrogateEvidenceSchema}).strict().readonly()).min(1).readonly()}).strict(),
]).superRefine((e,ctx)=>{
  if(e.kind==='observedOutcomes') {
    if(e.cumulativeUncertainty.kind==='bootstrapStandardErrors' && e.cumulativeUncertainty.standardErrors.length!==e.periods.length)
      ctx.addIssue({code:'custom',message:'Every cumulative horizon needs a bootstrap standard error.'})
    if(e.treatedRows+e.controlRows!==e.experimentalRows || e.participantBenchmark.residualDegreesOfFreedom!==e.experimentalRows-2 || new Set(e.periods.map(p=>p.label)).size!==e.periods.length)
      ctx.addIssue({code:'custom',message:'Observed outcome paths must retain the same participants at every period.'})
  } else {
    if(e.windows.length===0) return
    const first=e.windows[0]!.evidence
    if(new Set(e.windows.map(w=>w.label)).size!==e.windows.length || e.windows.some((w,i)=>
       w.evidence.experimentalRows!==first.experimentalRows || w.evidence.observationalRows!==first.observationalRows ||
       w.evidence.baselineColumns!==first.baselineColumns || w.evidence.estimator!==first.estimator ||
       !sameUncertainty(w.evidence.uncertainty,first.uncertainty) ||
       i>0 && w.evidence.surrogateColumns<=e.windows[i-1]!.evidence.surrogateColumns))
      ctx.addIssue({code:'custom',message:'Window results must retain the estimator, samples, baseline adjustment and uncertainty settings.'})
  }
}).readonly()
export type SurrogatePathEvidence = z.infer<typeof surrogatePathEvidenceSchema>

export function surrogatePathMatches(e:SurrogatePathEvidence,r:SurrogatePathRequest):boolean {
  switch(r.kind) {
    case 'observedOutcomes': return e.kind==='observedOutcomes' && e.experimentalRows===r.treatment.length &&
      e.treatedRows===r.treatment.filter(w=>w===1).length && e.periods.length===r.labels.length && e.periods.every((p,i)=>p.label===r.labels[i]) &&
      (r.uncertainty.kind==='none' ? e.cumulativeUncertainty.kind==='none' : e.cumulativeUncertainty.kind==='bootstrapStandardErrors' && e.cumulativeUncertainty.repetitions===r.uncertainty.repetitions && e.cumulativeUncertainty.seed===r.uncertainty.seed)
    case 'surrogateWindows': return e.kind==='surrogateWindows' && e.windows.length===r.windows.length && e.windows.every((w,i)=>{
      const requested=r.windows[i]!
      const value=w.evidence, model=r.model
      return w.label===requested.label && value.surrogateColumns===requested.surrogateColumns &&
        value.estimator===model.estimator && value.experimentalRows===model.experimental.treatment.length &&
        value.observationalRows===model.observational.outcome.length &&
        value.baselineColumns===(model.adjustment.kind==='none'?0:model.adjustment.experimental[0]!.length) &&
        (model.uncertainty.kind==='none' ? value.uncertainty.kind==='none' :
          value.uncertainty.kind==='bootstrapStandardError' && value.uncertainty.repetitions===model.uncertainty.repetitions && value.uncertainty.seed===model.uncertainty.seed)
    })
  }
}
