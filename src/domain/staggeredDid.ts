import { z } from 'zod'
import type { ColumnId } from './dataset'
import { err, ok, type Result } from './dop'
import type { PanelLongMatrix } from './panel'

const integer = z.number().int().safe()
const finite = z.number().finite()
const uniqueColumns = z.array(z.string().min(1)).refine(values => new Set(values).size === values.length)
export const staggeredInferenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('analytical') }).strict(),
  z.object({ kind: z.literal('bootstrapPointwise'), iterations: integer.positive(), seed: integer.min(0).max(0xffffffff) }).strict(),
  z.object({ kind: z.literal('bootstrapSimultaneous'), iterations: integer.positive(), seed: integer.min(0).max(0xffffffff) }).strict(),
])
export const staggeredSpecificationSchema = z.object({
  controls: z.enum(['never-treated', 'not-yet-treated']), baseline: z.enum(['varying', 'universal']),
  anticipation: integer.min(0).max(0xffffffff), firstEvent: integer.nullable(), lastEvent: integer.nullable(), balance: integer.min(0).max(0xffffffff).nullable(),
  confidence: finite.gt(0).lt(1), inference: staggeredInferenceSchema,
}).strict().refine(s => s.firstEvent === null || s.lastEvent === null || s.firstEvent <= s.lastEvent, 'The first event time must not exceed the last event time.')
export type StaggeredSpecification = z.infer<typeof staggeredSpecificationSchema>
export const defaultStaggeredSpecification: StaggeredSpecification = { controls:'never-treated', baseline:'varying', anticipation:0, firstEvent:null, lastEvent:null, balance:null, confidence:0.95, inference:{kind:'bootstrapSimultaneous', iterations:999, seed:731} }
const clusteringSchema = z.discriminatedUnion('kind', [
  z.object({kind:z.literal('unit')}).strict(),
  z.object({kind:z.literal('column'),column:z.string().min(1)}).strict(),
])
export type StaggeredClustering = { readonly kind:'unit' } | { readonly kind:'column'; readonly column:ColumnId }
const legacyConfigurationSchema = z.object({kind:z.literal('panel-intervention'),primary:z.literal('staggered'),covariates:uniqueColumns,specification:staggeredSpecificationSchema}).strict()
// Missing clustering is accepted only as the historical, unit-clustered format.
export const staggeredConfigurationSchema = z.union([
  legacyConfigurationSchema,
  legacyConfigurationSchema.extend({clustering:clusteringSchema}).strict(),
]).refine(c => !('clustering' in c && c.clustering.kind === 'column' && c.specification.inference.kind === 'analytical'), 'Additional clustering requires bootstrap inference.')
export interface StaggeredConfiguration { readonly kind:'panel-intervention'; readonly primary:'staggered'; readonly covariates:readonly ColumnId[]; readonly specification:StaggeredSpecification; readonly clustering?:StaggeredClustering }

export const staggeredIntervalSchema = z.discriminatedUnion('kind', [
  z.object({kind:z.literal('reference')}).strict(),
  z.object({kind:z.literal('unavailable'),estimate:finite}).strict(),
  z.object({kind:z.literal('estimated'),estimate:finite,standardError:finite.positive(),lower:finite,upper:finite}).strict().refine(v => v.lower <= v.estimate && v.estimate <= v.upper),
])
export type StaggeredInterval = z.infer<typeof staggeredIntervalSchema>
const coverage = z.discriminatedUnion('kind', [
  z.object({kind:z.literal('pointwise')}).strict(),
  z.object({kind:z.literal('simultaneous'),critical:finite.positive(),largeCritical:z.boolean()}).strict().refine(v => v.largeCritical === (v.critical >= 7)),
])
function family<K extends z.ZodType>(key: K) {
  return z.object({ keys:z.array(key).min(1), intervals:z.array(staggeredIntervalSchema).min(1), coverage, analyticalCovariance:z.array(z.array(finite)) }).strict().superRefine((value,ctx) => {
    const n=value.keys.length
    if(value.intervals.length!==n || value.analyticalCovariance.length!==n || value.analyticalCovariance.some(row=>row.length!==n) || new Set(value.keys.map(k=>JSON.stringify(k))).size!==n) ctx.addIssue({code:'custom',message:'Effect keys, intervals and covariance dimensions must agree.'})
  })
}
const change = z.discriminatedUnion('kind', [
  z.object({kind:z.literal('beyondObservedWindow'),unit:z.string(),adoption:integer}).strict(),
  z.object({kind:z.literal('latestCohortAsComparison'),unit:z.string(),adoption:integer}).strict(),
  z.object({kind:z.literal('noUntreatedComparison'),period:integer}).strict(),
  z.object({kind:z.literal('noPreTreatment'),unit:z.string()}).strict(),
])
const fitStatus = z.discriminatedUnion('kind', [z.object({kind:z.literal('converged'),iterations:integer.min(0)}).strict(),z.object({kind:z.literal('iterationLimit')}).strict()])
export const staggeredEvidenceSchema = z.object({
  kind:z.literal('staggeredDid'),version:z.literal(1),observations:integer.positive(),retainedObservations:integer.positive(),
  units:z.array(z.string().min(1)).min(2), times:z.array(integer).min(2),covariates:integer.min(0),weighted:z.boolean(),clusterCount:integer.min(1),specification:staggeredSpecificationSchema,
  changes:z.array(change),smallCohorts:z.array(z.object({adoption:integer.nullable(),count:integer.positive(),required:integer.positive()}).strict()),
  fits:z.array(z.object({cohort:integer,period:integer,status:fitStatus}).strict()),
  events:family(integer),cohorts:family(integer),calendar:family(integer),cells:family(z.tuple([integer,integer])),
  support:z.array(z.object({event:integer,cohorts:z.array(integer),treatedUnits:integer.min(0),referenceCohorts:z.array(integer)}).strict()),
  overall:z.object({dynamic:staggeredIntervalSchema,group:staggeredIntervalSchema,calendar:staggeredIntervalSchema,simple:staggeredIntervalSchema}).strict(),
}).strict().superRefine((e,ctx) => {
  const issue=(message:string)=>ctx.addIssue({code:'custom',message})
  if(e.units.length*e.times.length!==e.retainedObservations || e.retainedObservations>e.observations || new Set(e.units).size!==e.units.length || e.times.some((t,i)=>i>0 && t<=e.times[i-1]!) || e.clusterCount>e.units.length) issue('Retained panel dimensions or identities are inconsistent.')
  if(e.support.length!==e.events.keys.length || e.support.some((s,i)=>s.event!==e.events.keys[i] || s.referenceCohorts.some(g=>!s.cohorts.includes(g)))) issue('Event support must align with the reported effects.')
  if(Object.values(e.overall).some(i=>i.kind==='reference')) issue('An overall ATT cannot be a normalized baseline.')
  const expected=e.specification.inference.kind==='bootstrapSimultaneous'?'simultaneous':'pointwise'
  if([e.events,e.cohorts,e.calendar,e.cells].some(f=>f.coverage.kind!==expected)) issue('Interval coverage does not match the requested inference.')
})
export type StaggeredEvidence = z.infer<typeof staggeredEvidenceSchema>
export type StaggeredFamily = StaggeredEvidence['events']
export const staggeredRequestSchema = z.object({rows:integer.positive(),columns:integer.positive(),units:z.array(z.string().min(1)),times:z.array(integer),adoption:z.array(integer.nullable()),weights:z.array(finite.nonnegative()).nullable(),clusters:z.array(z.string().min(1)).nullable(),specification:staggeredSpecificationSchema}).strict().superRefine((r,ctx)=>{
  if([r.units,r.times,r.adoption,...(r.weights===null?[]:[r.weights]),...(r.clusters===null?[]:[r.clusters])].some(v=>v.length!==r.rows)) ctx.addIssue({code:'custom',message:'Every row needs aligned keys and values.'})
})
export type StaggeredRequest = z.infer<typeof staggeredRequestSchema>

/** Derive cohorts from absorbing binary assignment, never from the outcome. */
export function staggeredInput(matrix: PanelLongMatrix, specification:StaggeredSpecification, clustering:StaggeredClustering = {kind:'unit'}): Result<{readonly values:Float64Array; readonly model:StaggeredRequest},string> {
  const rows=matrix.rowCount, columns=matrix.values.length/rows
  if(!Number.isInteger(columns)||columns<2||matrix.units.length!==rows||matrix.periodCodes.length!==rows) return err('The panel values and keys are not aligned.')
  let clusters: readonly string[] | null = null
  if (clustering.kind === 'column') {
    if (specification.inference.kind === 'analytical') return err('Additional clustering requires bootstrap inference. Choose pointwise or simultaneous bootstrap.')
    if (matrix.cluster === undefined || matrix.cluster.column !== clustering.column || matrix.cluster.labels.length !== rows) return err('Cluster labels must be read with the panel values from the selected column.')
    clusters = matrix.cluster.labels
    const byUnit = new Map<string,string>()
    for (let i=0;i<rows;i++) {
      const label=clusters[i]!,unit=matrix.units[i]!
      if (label.length === 0) return err('Every panel row needs an observed cluster label.')
      if (byUnit.has(unit) && byUnit.get(unit)!==label) return err(`Cluster membership changes for ${unit}. Each unit must remain in one cluster.`)
      byUnit.set(unit,label)
    }
    if (new Set(clusters).size < 2) return err('Clustered inference requires at least two clusters.')
  } else if (matrix.cluster !== undefined) return err('The materialized cluster column does not match unit-level clustering.')
  const byUnit=new Map<string,Map<number,number>>()
  for(let i=0;i<rows;i++) {
    const unit=matrix.units[i]!,time=matrix.periodCodes[i]!,treatment=matrix.values[rows+i]!
    if(treatment!==0&&treatment!==1) return err('Staggered DiD requires a binary treatment indicator in each period.')
    const periods=byUnit.get(unit)??new Map<number,number>()
    if(periods.has(time)) return err('The panel contains a repeated unit-period key.')
    periods.set(time,treatment);byUnit.set(unit,periods)
  }
  const adoption=new Map<string,number|null>()
  for(const [unit,periods] of byUnit) {
    if(periods.size!==matrix.periods.length || matrix.periods.some(p=>!periods.has(p.code))) return err('Staggered DiD requires a balanced, complete panel.')
    let first:number|null=null
    for(const [time,treatment] of [...periods].sort(([a],[b])=>a-b)) {
      if(treatment===1&&first===null) first=time
      if(treatment===0&&first!==null) return err(`Treatment switches off for ${unit}. This method requires treatment to remain on after adoption.`)
    }
    adoption.set(unit,first)
  }
  if([...adoption.values()].every(g=>g===null)) return err('No unit adopts treatment in the observed panel.')
  const values=new Float64Array(rows*(columns-1))
  values.set(matrix.values.subarray(0,rows))
  values.set(matrix.values.subarray(2*rows),rows)
  if(values.some(v=>!Number.isFinite(v))) return err('Outcomes and covariates must be observed for every unit and period.')
  const parsed=staggeredRequestSchema.safeParse({rows,columns:columns-1,units:[...matrix.units],times:[...matrix.periodCodes],adoption:matrix.units.map(unit=>adoption.get(unit)!),weights:null,clusters,specification})
  return parsed.success?ok({values,model:parsed.data}):err(z.prettifyError(parsed.error))
}
export function sameStaggeredSpecification(a:StaggeredSpecification,b:StaggeredSpecification):boolean {
  return a.controls===b.controls&&a.baseline===b.baseline&&a.anticipation===b.anticipation&&a.firstEvent===b.firstEvent&&a.lastEvent===b.lastEvent&&a.balance===b.balance&&a.confidence===b.confidence
    && a.inference.kind===b.inference.kind && (a.inference.kind==='analytical' || (b.inference.kind!=='analytical'&&a.inference.iterations===b.inference.iterations&&a.inference.seed===b.inference.seed))
}
export function staggeredRecordMatches(raw:unknown,rawStudy:unknown):boolean {
  const target=z.object({kind:z.literal('average-treatment-effect-on-treated'),scale:z.literal('additive'),treatedValue:z.literal(1)})
  const interval=z.discriminatedUnion('kind',[z.object({kind:z.literal('confidence'),level:finite,lower:finite,upper:finite}),z.object({kind:z.literal('none'),reason:z.string()})])
  const run=z.object({study:z.string(),configuration:staggeredConfigurationSchema,evidence:staggeredEvidenceSchema,columns:z.array(z.object({column:z.string()})),timeLabels:z.array(z.string()),sourcePeriods:z.array(z.object({code:integer,label:z.string().min(1)}).strict()).min(1),estimate:z.object({estimand:target,effect:z.object({kind:z.literal('additive'),value:finite}),standardError:finite.nullable(),interval})}).safeParse(raw)
  const study=z.object({id:z.string(),estimand:target,outcome:z.object({column:z.string()}),treatment:z.object({column:z.string()})}).safeParse(rawStudy)
  if(!run.success||!study.success) return false
  const {configuration:c,evidence:e,columns,estimate}=run.data,headline=e.overall.dynamic
  const clustered = 'clustering' in c && c.clustering.kind === 'column'
  if (clustered ? e.clusterCount < 2 || e.specification.inference.kind === 'analytical' : e.clusterCount !== e.units.length) return false
  return run.data.study===study.data.id&&sameStaggeredSpecification(c.specification,e.specification)&&c.covariates.length===e.covariates&&columns.length===2+c.covariates.length
    && columns[0]?.column===study.data.outcome.column&&columns[1]?.column===study.data.treatment.column&&c.covariates.every((id,i)=>columns[i+2]?.column===id&&id!==study.data.outcome.column&&id!==study.data.treatment.column)
    && headline.kind!=='reference'&&estimate.effect.value===headline.estimate&&estimate.standardError===(headline.kind==='estimated'?headline.standardError:null)
    && run.data.timeLabels.length===e.times.length
    && new Set(run.data.sourcePeriods.map(p=>p.code)).size===run.data.sourcePeriods.length
    && e.times.every((code,i)=>run.data.sourcePeriods.find(p=>p.code===code)?.label===run.data.timeLabels[i])
    && e.changes.every(change=>!('period' in change)||run.data.sourcePeriods.some(p=>p.code===change.period))
    && (headline.kind==='estimated'
      ? estimate.interval.kind==='confidence'&&estimate.interval.level===e.specification.confidence&&estimate.interval.lower===headline.lower&&estimate.interval.upper===headline.upper
      : estimate.interval.kind==='none')
}
