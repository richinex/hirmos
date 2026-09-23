import { z } from 'zod'
import type { ColumnId } from './dataset'
const index=z.number().int().nonnegative()
const finite=z.number().finite()
const term=z.object({column:index,lag:index,name:z.string().min(1)}).strict()
const keys=z.array(z.tuple([index,z.number().int()])).min(1)
const cohort={keys,adoption:z.array(z.tuple([index,z.number().int().nullable()])).min(1),cohort:z.number().int(),covariates:z.array(term)}
export const countRegressionRequestSchema=z.object({
  rows:z.number().int().positive(),columns:z.number().int().positive(),outcome:index,
  family:z.discriminatedUnion('kind',[
    z.object({kind:z.literal('negativeBinomial'),exposure:index.nullable()}).strict(),
    z.object({kind:z.literal('binomial'),trials:index}).strict(),
  ]),
  design:z.discriminatedUnion('kind',[
    z.object({kind:z.literal('lags'),keys,terms:z.array(term).min(1),sum:z.array(index).min(1)}).strict(),
    z.object({kind:z.literal('events'),...cohort,window:z.discriminatedUnion('kind',[z.object({kind:z.literal('all')}).strict(),z.object({kind:z.literal('finite'),first:z.number().int().max(-1),last:z.number().int().nonnegative()}).strict()])}).strict(),
    z.object({kind:z.literal('summary'),...cohort}).strict(),
    z.object({kind:z.literal('interrupted'),intervention:index,horizon:index,bandwidth:index,covariates:z.array(term)}).strict(),
  ]),confidence:finite.gt(0).lt(1),iterations:z.number().int().positive(),tolerance:finite.positive(),
}).strict().superRefine((r,ctx)=>{
  const fail=(message:string)=>ctx.addIssue({code:'custom',message})
  const denominator=r.family.kind==='binomial'?r.family.trials:r.family.exposure
  if(r.outcome>=r.columns||denominator!==null&&(denominator>=r.columns||denominator===r.outcome))fail('Choose distinct outcome and denominator columns.')
  const terms=r.design.kind==='lags'?r.design.terms:r.design.covariates
  if(terms.some(t=>t.column>=r.columns||t.column===r.outcome||t.column===denominator)||new Set(terms.map(t=>`${t.column}:${t.lag}`)).size!==terms.length)fail('Predictor roles must be distinct and inside the model matrix.')
  if(r.design.kind!=='lags'&&(r.family.kind!=='negativeBinomial'||terms.some(t=>t.lag!==0)))fail('This model requires NB2 with contemporaneous covariates.')
  if(r.design.kind==='interrupted'){
    if(r.design.intervention<2||r.design.intervention>=r.rows||r.design.horizon>=r.rows-r.design.intervention||r.design.bandwidth>=r.rows)fail('Choose a valid intervention row, post-event horizon and HAC bandwidth.')
  }else{
    if(r.design.keys.length!==r.rows||new Set(r.design.keys.map(k=>JSON.stringify(k))).size!==r.rows)fail('Panel rows need unique unit-period keys.')
    if(r.design.kind==='lags'&&(r.design.sum.some(i=>i>=terms.length)||new Set(r.design.sum).size!==r.design.sum.length))fail('Choose distinct terms for the lag-sum contrast.')
    if(r.design.kind!=='lags'&&new Set(r.design.adoption.map(([u])=>u)).size!==r.design.adoption.length)fail('Each team needs one adoption record.')
  }
})
export type CountRegressionRequest=z.infer<typeof countRegressionRequestSchema>
const estimate=z.object({name:z.string(),estimate:finite,standardError:finite.nonnegative(),lower:finite,upper:finite,ratio:finite.nonnegative(),ratioLower:finite.nonnegative(),ratioUpper:finite.nonnegative()}).strict().refine(e=>e.lower<=e.estimate&&e.estimate<=e.upper&&e.ratioLower<=e.ratio&&e.ratio<=e.ratioUpper,'Invalid coefficient interval.')
export const countRegressionEvidenceSchema=z.object({
  request:countRegressionRequestSchema,observations:z.number().int().positive(),inputRows:z.number().int().positive(),parameters:z.number().int().positive(),
  retained:z.array(index),omitted:z.array(index),terms:z.array(estimate).min(1),contrasts:z.array(estimate),fitted:z.array(finite),observed:z.array(finite),covariance:z.array(z.array(finite)),eventPeriods:z.array(z.tuple([index,z.number().int()])),
  status:z.discriminatedUnion('kind',[
    z.object({kind:z.literal('converged'),iterations:index}).strict(),z.object({kind:z.literal('iterationLimit'),iterations:index}).strict(),z.object({kind:z.literal('lineSearchFailed'),iterations:index}).strict(),
  ]),
}).strict().superRefine((e,ctx)=>{
  const fail=(message:string)=>ctx.addIssue({code:'custom',message})
  const partition=[...e.retained,...e.omitted].sort((a,b)=>a-b)
  if(e.inputRows!==e.request.rows||partition.length!==e.inputRows||partition.some((v,i)=>v!==i)||e.retained.length!==e.observations||e.fitted.length!==e.observations||e.observed.length!==e.observations)fail('The result rows do not match the requested data.')
  if(e.parameters!==e.terms.length+(e.request.family.kind==='negativeBinomial'?1:0)||e.covariance.length!==e.parameters||e.covariance.some(row=>row.length!==e.parameters))fail('The coefficient covariance has invalid dimensions.')
  if(e.contrasts.length!==(e.request.design.kind==='events'?0:1))fail('The requested model contrast is missing.')
  const design=e.request.design
  const expected=design.kind==='events'? [...new Set(design.keys.filter(([u])=>design.adoption.some(([unit,t])=>unit===u&&t===design.cohort)).map(([,t])=>t-design.cohort))].filter(t=>t!==-1&&(design.window.kind==='all'||t>=design.window.first&&t<=design.window.last)).sort((a,b)=>a-b):[]
  if(e.eventPeriods.length!==expected.length||e.eventPeriods.some(([j,t],i)=>j!==i+1||j>=e.terms.length||t!==expected[i]))fail('The event coefficients do not match their requested periods.')
})
export type CountRegressionEvidence=z.infer<typeof countRegressionEvidenceSchema>
export function sameCountRequest(a:CountRegressionRequest,b:CountRegressionRequest):boolean{return JSON.stringify(a)===JSON.stringify(b)}
export type CountRegressionDraft={
  readonly outcome:ColumnId|null
  readonly family:{readonly kind:'negativeBinomial';readonly exposure:ColumnId|null}|{readonly kind:'binomial';readonly trials:ColumnId|null}
  readonly model:{readonly kind:'lags';readonly predictor:ColumnId|null;readonly lags:string}|{readonly kind:'events';readonly onset:ColumnId|null;readonly cohort:string;readonly window:{readonly kind:'all'}|{readonly kind:'finite';readonly first:string;readonly last:string}}|{readonly kind:'summary';readonly onset:ColumnId|null;readonly cohort:string}|{readonly kind:'interrupted';readonly intervention:string;readonly horizon:string;readonly bandwidth:string}
  readonly covariates:readonly ColumnId[]
  readonly confidence:string
  readonly iterations:string
  readonly tolerance:string
}
export const initialCountRegression=(panel:boolean):CountRegressionDraft=>({outcome:null,family:{kind:'negativeBinomial',exposure:null},model:panel?{kind:'lags',predictor:null,lags:'1, 2, 3, 4'}:{kind:'interrupted',intervention:'',horizon:'0',bandwidth:'3'},covariates:[],confidence:'0.95',iterations:'1000',tolerance:'0.00001'})
