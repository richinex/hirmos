import {surrogatePathEvidenceSchema} from './surrogatePath'
import {surrogateHorizonRecordsMatch,sameSurrogateHorizons,surrogateHorizonsSchema,noSurrogateHorizons} from './surrogateHorizons'
import {surrogateDiagnosticEvidenceSchema,sameSurrogateRestriction} from './surrogateDiagnostics'
import { z } from 'zod'
import { brand } from './dop'
import type { DatasetProfile } from './dataset'
import { surrogateEvidenceSchema } from './surrogate'
import { surrogateSelectionColumns, surrogateSelectionSchema } from './surrogateSamples'

const indices = z.array(z.number().int().safe().nonnegative()).readonly()
export const surrogateRunSchema = z.object({
  kind: z.literal('surrogate-run'),
  id: z.string().uuid().transform(v => brand<string, 'SurrogateRunId'>(v)),
  createdAt: z.string().datetime(), sourceFingerprint: z.string().regex(/^[a-f0-9]{64}$/),
  sourceRows: z.number().int().safe().positive(),
  selection: surrogateSelectionSchema,
  rationale: z.string().trim().min(1),
  rows: z.object({ experimental: indices, observational: indices, excluded: indices }).strict(),
  evidence: surrogateEvidenceSchema,
  paths:z.object({specification:surrogateHorizonsSchema,results:z.array(surrogatePathEvidenceSchema).readonly()}).strict().readonly().default({specification:noSurrogateHorizons,results:[]}),
  diagnostics: z.array(surrogateDiagnosticEvidenceSchema).readonly().default([]),
}).strict().superRefine((r, ctx) => {
  const s = r.selection, e = r.evidence
  if(!sameSurrogateHorizons(s.horizons,r.paths.specification) || !surrogateHorizonRecordsMatch(s,e,r.paths.results))ctx.addIssue({code:'custom',message:'Saved horizon results must match the requested periods, surrogate groups, samples and estimator.'})
  const checks = s.checks
  const validation = r.diagnostics.filter(d => d.analysis.kind === 'validation')
  const bounds = r.diagnostics.filter(d => d.analysis.kind === 'biasBounds')
  if (validation.length !== (checks.validation === 'none' ? 0 : 1) || bounds.length !== (checks.biasBounds.kind === 'none' ? 0 : 1) ||
      r.diagnostics.some(d => d.experimentalRows !== e.experimentalRows || d.observationalRows !== e.observationalRows ||
        d.surrogateColumns !== e.surrogateColumns || d.baselineColumns !== e.baselineColumns ||
        d.analysis.kind === 'biasBounds' && (checks.biasBounds.kind === 'none' || !sameSurrogateRestriction(d.analysis.restriction,checks.biasBounds))))
    ctx.addIssue({code:'custom',message:'Saved diagnostics must match the requested checks and sample dimensions.'})
  const all = [...r.rows.experimental, ...r.rows.observational, ...r.rows.excluded]
  const uncertaintyMatches = s.uncertainty.kind === 'none' ? e.uncertainty.kind === 'none' :
    e.uncertainty.kind === 'bootstrapStandardError' && s.uncertainty.repetitions === e.uncertainty.repetitions && s.uncertainty.seed === e.uncertainty.seed
  if (all.length !== r.sourceRows || new Set(all).size !== all.length || all.some(row => row >= r.sourceRows) ||
      e.experimentalRows !== r.rows.experimental.length || e.observationalRows !== r.rows.observational.length ||
      e.surrogateColumns !== s.surrogates.length || e.baselineColumns !== (s.adjustment.kind === 'none' ? 0 : s.adjustment.columns.length) ||
      e.estimator !== s.estimator || !uncertaintyMatches)
    ctx.addIssue({ code: 'custom', message: 'The saved surrogate estimate, sample rows and specification must agree.' })
}).readonly()
export type SurrogateRun = z.infer<typeof surrogateRunSchema>
export type SurrogateRunId = SurrogateRun['id']

export function surrogateRunMatchesProfile(run: SurrogateRun, profile: DatasetProfile): boolean {
  return surrogateRunSchema.safeParse(run).success && run.sourceFingerprint === profile.source.fingerprint &&
    run.sourceRows === profile.rowCount && surrogateSelectionColumns(run.selection).every(id => profile.columns.some(c => c.id === id))
}
