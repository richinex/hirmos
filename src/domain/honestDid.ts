import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { EstimationRunArtifact } from './estimation'
import type { TimeSeriesRun } from './timeSeries'
import type { PreparedDatasetVersionId } from './preprocessing'

const finite=z.number().finite()
const positive=z.number().int().positive()
const vector=z.array(finite).min(1)
export const honestEventStudySchema=z.object({pre:positive,post:positive,events:z.array(z.number().int()),coefficients:vector,covariance:z.array(vector),contrast:vector}).strict().superRefine((v,ctx)=>{
  const n=v.pre+v.post
  if(n>100)ctx.addIssue({code:'custom',message:'This implementation supports at most 100 event estimates.'})
  if(v.events.length!==n||v.coefficients.length!==n||v.covariance.length!==n||v.covariance.some(row=>row.length!==n)||v.contrast.length!==v.post||v.contrast.every(x=>x===0))ctx.addIssue({code:'custom',message:'Event estimates, covariance and target weights must have matching pre/post dimensions.'})
  if(v.events.some((event,i)=>event!==(i<v.pre?i-v.pre-1:i-v.pre)))ctx.addIssue({code:'custom',message:'Use consecutive event periods with period -1 omitted as the zero reference.'})
  if(v.covariance.some((row,i)=>(row[i]??0)<=0||row.some((value,j)=>v.covariance[j]?.[i]===undefined||Math.abs(value-v.covariance[j]![i]!)>1e-10*Math.max(1,Math.abs(value)))))ctx.addIssue({code:'custom',message:'The event-study covariance must be symmetric with positive variances.'})
})
export type HonestEventStudy=z.infer<typeof honestEventStudySchema>
const grid=z.discriminatedUnion('kind',[
  z.object({kind:z.literal('referenceDefault'),points:positive.min(2).max(10001)}).strict(),
  z.object({kind:z.literal('explicit'),lower:finite,upper:finite,points:positive.min(2).max(10001)}).strict().refine(g=>g.lower<g.upper,'Grid limits must increase.'),
])
export const honestConfigurationSchema=z.object({confidence:finite.gt(0).lt(1),bounds:z.array(finite.nonnegative()).min(1).max(25).refine(v=>v.every((x,i)=>i===0||x>v[i-1]!),'Sensitivity bounds must be increasing.'),seed:z.number().int().min(0).max(4294967295),restriction:z.discriminatedUnion('kind',[
  z.object({kind:z.literal('smoothness'),points:positive.min(2).max(10001)}).strict(),
  z.object({kind:z.literal('relativeMagnitude'),method:z.discriminatedUnion('kind',[
    z.object({kind:z.literal('conditional')}).strict(),
    z.object({kind:z.literal('leastFavorableHybrid'),kappa:finite.gt(0).lt(1)}).strict(),
  ]),moments:z.enum(['postTreatment','allPeriods']),grid}).strict(),
])}).strict().superRefine((v,ctx)=>{if(v.restriction.kind==='relativeMagnitude'&&v.restriction.method.kind==='leastFavorableHybrid'&&v.restriction.method.kappa>=1-v.confidence)ctx.addIssue({code:'custom',message:'Hybrid size must be smaller than one minus the confidence level.'})})
export type HonestConfiguration=z.infer<typeof honestConfigurationSchema>
export const honestRequestSchema=z.object({eventStudy:honestEventStudySchema,configuration:honestConfigurationSchema}).strict().superRefine((r,ctx)=>{
  if(r.configuration.restriction.kind==='relativeMagnitude'&&r.eventStudy.post===1&&r.eventStudy.contrast[0]!==1)ctx.addIssue({code:'custom',message:'The single-post-period conditional route requires a unit target weight.'})
})
export type HonestRequest=z.infer<typeof honestRequestSchema>
const interval=z.discriminatedUnion('kind',[
  z.object({kind:z.literal('fixedLength'),lower:finite,upper:finite,coefficients:vector,halfLength:finite.nonnegative(),search:z.enum(['derivativeBisection','grid']),accuracy:z.enum(['solved','residualQualified','referenceInaccurate'])}).strict().refine(v=>v.lower<=v.upper),
  z.object({kind:z.literal('confidenceGrid'),grid:vector,accepted:z.array(z.boolean()),openLower:z.boolean(),openUpper:z.boolean()}).strict().superRefine((v,ctx)=>{if(v.grid.length!==v.accepted.length||v.grid.some((x,i)=>i>0&&x<=v.grid[i-1]!)||v.openLower!==v.accepted[0]||v.openUpper!==v.accepted.at(-1))ctx.addIssue({code:'custom',message:'Confidence-grid values, acceptance flags and endpoints disagree.'})}),
])
export const honestEvidenceSchema=z.object({kind:z.literal('honestDid'),version:z.literal(1),request:honestRequestSchema,estimate:finite,standardError:finite.positive(),conventional:z.tuple([finite,finite]),results:z.array(z.object({bound:finite.nonnegative(),interval}).strict()).min(1)}).strict().superRefine((e,ctx)=>{
  if(e.results.length!==e.request.configuration.bounds.length||e.results.some((r,i)=>r.bound!==e.request.configuration.bounds[i]||r.interval.kind!==(e.request.configuration.restriction.kind==='smoothness'?'fixedLength':'confidenceGrid')))ctx.addIssue({code:'custom',message:'Sensitivity results must match every recorded bound and restriction.'})
  const {pre,post,coefficients,covariance,contrast}=e.request.eventStudy
  const estimate=contrast.reduce((sum,w,i)=>sum+w*coefficients[pre+i]!,0)
  const variance=contrast.reduce((sum,a,i)=>sum+contrast.reduce((s,b,j)=>s+a*b*covariance[pre+i]![pre+j]!,0),0)
  const close=(a:number,b:number)=>Math.abs(a-b)<=1e-9*Math.max(1,Math.abs(a),Math.abs(b))
  if(!close(e.estimate,estimate)||!close(e.standardError**2,variance)||e.conventional[0]>e.conventional[1]||!close((e.conventional[0]+e.conventional[1])/2,estimate))ctx.addIssue({code:'custom',message:'The reported target and uncertainty must match the recorded event estimates and covariance.'})
  if(e.results.some(r=>r.interval.kind==='fixedLength'?r.interval.coefficients.length!==pre+post:r.interval.grid.length!==(e.request.configuration.restriction.kind==='relativeMagnitude'?e.request.configuration.restriction.grid.points:0)))ctx.addIssue({code:'custom',message:'The numerical evidence dimensions must match the specification.'})
})
export type HonestEvidence=z.infer<typeof honestEvidenceSchema>
export const honestSourceSchema=z.discriminatedUnion('kind',[
  z.object({kind:z.literal('estimation'),run:z.string().uuid()}).strict(),
  z.object({kind:z.literal('regressionDesign'),run:z.string().uuid()}).strict(),
])
export const honestRunSchema=z.object({kind:z.literal('honest-did-run'),id:z.string().uuid().transform(v=>brand<string,'SensitivityRunId'>(v)),preparedDataset:z.string().uuid().transform(v=>brand<string,'PreparedDatasetVersionId'>(v)),createdAt:z.iso.datetime(),source:honestSourceSchema,evidence:honestEvidenceSchema}).strict()
export type HonestRun=z.infer<typeof honestRunSchema>
export const defaultHonestConfiguration:HonestConfiguration={confidence:0.95,bounds:[0,0.5,1,1.5,2],seed:0,restriction:{kind:'relativeMagnitude',method:{kind:'conditional'},moments:'postTreatment',grid:{kind:'referenceDefault',points:1001}}}
export type HonestCandidate={readonly source:z.infer<typeof honestSourceSchema>;readonly label:string;readonly createdAt:string;readonly preparedDataset:PreparedDatasetVersionId;readonly input:Result<HonestEventStudy,string>}
function eventInput(events:number[],coefficients:number[],covariance:number[][]):Result<HonestEventStudy,string>{
  const pre=events.filter(t=>t<0).length,post=events.length-pre
  const parsed=honestEventStudySchema.safeParse({pre,post,events,coefficients,covariance,contrast:Array.from({length:post},(_,i)=>i===0?1:0)})
  return parsed.success?ok(parsed.data):err(z.prettifyError(parsed.error))
}
export function honestCandidates(estimates:readonly EstimationRunArtifact[],designs:readonly TimeSeriesRun[]):readonly HonestCandidate[]{
  const result:HonestCandidate[]=[]
  for(const run of estimates){
    if(run.kind!=='panel-intervention-run'||run.configuration.primary!=='staggered'||run.evidence.kind!=='staggeredDid')continue
    const e=run.evidence,selected=e.events.keys.map((event,i)=>({event,i})).filter(({event})=>event!==-1).sort((a,b)=>a.event-b.event)
    const input=e.specification.baseline!=='universal'||e.specification.anticipation!==0?err('Choose a universal pre-treatment baseline with zero anticipation. This route requires level contrasts against period -1.'):eventInput(selected.map(s=>s.event),selected.map(s=>{const v=e.events.intervals[s.i]!;return v.kind==='reference'?0:v.estimate}),selected.map(a=>selected.map(b=>e.events.analyticalCovariance[a.i]![b.i]!)))
    result.push({source:{kind:'estimation',run:run.id},label:'Staggered event study',createdAt:run.createdAt,preparedDataset:run.preparedDataset,input})
  }
  for(const run of designs){
    if(run.kind!=='panel-regression'||run.specification.specification.kind!=='eventStudy')continue
    const window=run.specification.specification.window,e=run.evidence,selected=[...e.events].sort((a,b)=>a.period-b.period)
    const index=(term:number)=>e.terms.findIndex(t=>t.index===term)
    const input=window.reference!==-1||window.tails!=='reference'?err('Use period -1 as the reference and unbinned event periods.'):selected.some(s=>index(s.index)<0)?err('An event coefficient is missing from the saved regression evidence.'):eventInput(selected.map(s=>s.period),selected.map(s=>e.terms[index(s.index)]!.estimate),selected.map(a=>selected.map(b=>e.covariance[index(a.index)]![index(b.index)]!)))
    result.push({source:{kind:'regressionDesign',run:run.id},label:`Regression event study for ${run.outcome.name}`,createdAt:run.createdAt,preparedDataset:run.preparedDataset,input})
  }
  return result
}
export function honestRunMatches(run:HonestRun,estimates:readonly EstimationRunArtifact[],designs:readonly TimeSeriesRun[]):boolean{
  if(!honestRunSchema.safeParse(run).success)return false
  const candidate=honestCandidates(estimates,designs).find(c=>c.source.kind===run.source.kind&&c.source.run===run.source.run)
  if(candidate===undefined||candidate.preparedDataset!==run.preparedDataset||!candidate.input.ok)return false
  const {contrast: _sourceContrast,...source}=candidate.input.value
  const {contrast: _targetContrast,...recorded}=run.evidence.request.eventStudy
  return JSON.stringify(source)===JSON.stringify(recorded)
}
export function honestEnvelope(interval:HonestEvidence['results'][number]['interval']):{readonly lower:number;readonly upper:number}|null{
  if(interval.kind==='fixedLength')return {lower:interval.lower,upper:interval.upper}
  const accepted=interval.grid.filter((_,i)=>interval.accepted[i])
  return accepted.length===0?null:{lower:accepted[0]!,upper:accepted.at(-1)!}
}
