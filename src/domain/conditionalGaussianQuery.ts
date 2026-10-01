import {z} from 'zod'
import type {InterventionQueryId} from './intervention'
import type {DagDocumentId,DagRevisionId} from './dag'
import type {PreparedDatasetVersionId} from './preprocessing'

const index=z.number().int().nonnegative()
const assignment=z.object({variable:index,value:z.number().finite()}).strict()
export const conditionalGaussianQuerySchema=z.object({
  rows:z.number().int().positive(),columns:z.number().int().positive(),names:z.array(z.string().trim().min(1)).min(1),
  edges:z.array(z.tuple([index,index])),variables:z.array(z.enum(['continuous','discrete'])).min(1),outcome:index,
  observations:z.array(assignment),interventions:z.array(assignment),
}).strict().superRefine((q,ctx)=>{
  const roles=[q.outcome,...q.observations.map(a=>a.variable),...q.interventions.map(a=>a.variable)]
  if(q.names.length!==q.columns||q.variables.length!==q.columns||new Set(q.names).size!==q.columns||q.variables[q.outcome]!=='continuous'||roles.some(i=>i>=q.columns)||new Set(roles).size!==roles.length||q.edges.some(([a,b])=>a>=q.columns||b>=q.columns||a===b))ctx.addIssue({code:'custom',message:'Choose one continuous outcome, distinct query roles and valid graph variables.'})
})
export type ConditionalGaussianQuery=z.infer<typeof conditionalGaussianQuerySchema>
export const conditionalGaussianEvidenceSchema=z.object({kind:z.literal('conditionalGaussianQuery'),observations:z.number().int().positive(),outcome:index,mean:z.number().finite(),std:z.number().finite().positive(),configurationRows:z.number().int().positive()}).strict()
export type ConditionalGaussianEvidence=z.infer<typeof conditionalGaussianEvidenceSchema>
export const conditionalGaussianArtifactSchema=z.object({kind:z.literal('conditional-gaussian-query'),id:z.string().min(1),dagDocument:z.string().min(1),dagRevision:z.string().min(1),preparedDataset:z.string().min(1),createdAt:z.string().min(1),specification:conditionalGaussianQuerySchema,result:conditionalGaussianEvidenceSchema}).strict().superRefine((a,ctx)=>{
  if(a.result.observations!==a.specification.rows||a.result.outcome!==a.specification.outcome||a.result.configurationRows>a.specification.rows)ctx.addIssue({code:'custom',message:'The saved Gaussian result does not match its query.'})
})
export interface ConditionalGaussianArtifact {
  readonly kind:'conditional-gaussian-query';readonly id:InterventionQueryId;readonly dagDocument:DagDocumentId;readonly dagRevision:DagRevisionId;readonly preparedDataset:PreparedDatasetVersionId;readonly createdAt:string
  readonly specification:ConditionalGaussianQuery;readonly result:ConditionalGaussianEvidence
}
export function describeConditionalGaussianQuery(q:ConditionalGaussianQuery):string {
  const show=(a:ConditionalGaussianQuery['observations'][number])=>`${q.names[a.variable]} = ${a.value}`
  const given=[...q.interventions.map(a=>`do(${show(a)})`),...q.observations.map(show)]
  return `${q.names[q.outcome]}${given.length?` given ${given.join(', ')}`:''}`
}

/** Short symbols keep a query readable in an inspector. Names remain plain text in the key. */
export function conditionalGaussianQueryFormula(q:ConditionalGaussianQuery){
  const conditions=[...q.interventions,...q.observations]
  const valueTex=(value:number)=>String(value).replace(/e([+-]?\d+)$/i,(_,exponent:string)=>`\\times 10^{${Number(exponent)}}`)
  // Keep a variable's symbol fixed when its role changes between runs.
  const assignments=conditions.map(a=>`X_{${a.variable+1}}=${valueTex(a.value)}`)
  const actions=assignments.slice(0,q.interventions.length)
  const observed=assignments.slice(q.interventions.length)
  const given=[...(actions.length?[`\\operatorname{do}(${actions.join(',\\allowbreak ')})`]:[]),...observed]
  return {
    plain:describeConditionalGaussianQuery(q),
    tex:`p(Y${given.length?`\\mid ${given.join(',\\allowbreak ')}`:''})`,
    variables:[{tex:'Y',plain:'Y',name:q.names[q.outcome]},...[...conditions].sort((a,b)=>a.variable-b.variable).map(a=>({tex:`X_{${a.variable+1}}`,plain:`X${a.variable+1}`,name:q.names[a.variable]}))],
  }
}
