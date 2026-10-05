import { z } from 'zod'
import { swigVariable as variable, swigTermSchema, swigEquationSchema } from './swigMechanisms'
export const panelRoleSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('confounder') }).strict(),
  z.object({ kind: z.literal('disturbance'), of: variable }).strict(),
  z.object({ kind: z.literal('covariate'), period: z.number().int().min(0).max(32) }).strict(),
  z.object({ kind: z.literal('treatment'), period: z.number().int().min(1).max(32) }).strict(),
  z.object({ kind: z.literal('outcome'), period: z.number().int().min(0).max(32) }).strict(),
])
export type SwigPanelRole = z.infer<typeof panelRoleSchema>
export const didDesignSchema = z
  .object({
    panelRoles: z.array(panelRoleSchema).min(1).max(256),
    untreated: z.array(swigEquationSchema).min(2).max(33),
    alpha: swigTermSchema,
    assumptions: z
      .object({
        absorbingBinaryTreatment: z.boolean(),
        consistency: z.boolean(),
        independentDisturbances: z.boolean(),
      })
      .strict(),
    adoption: z.number().int().min(1).max(32),
    outcome: z.number().int().min(0).max(32),
    comparison: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('neverTreated') }).strict(),
      z
        .object({ kind: z.literal('notYetTreated'), through: z.number().int().min(0).max(32) })
        .strict(),
    ]),
    measured: z.array(variable).max(256),
    selected: z.array(variable).max(256),
  })
  .strict()
export type SwigDidDesign = z.infer<typeof didDesignSchema>
const blocker = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unmeasured'), variable }).strict(),
  z
    .object({
      kind: z.literal('unestablishedAssignment'),
      variable,
      treatment: variable,
      required: z.number().finite(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('conflictingAssignment'),
      variable,
      treatment: variable,
      required: z.number().finite(),
      actual: z.number().finite(),
    })
    .strict(),
])
export const didAssessmentSchema = z
  .object({
    decision: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('baselineIdentity') }).strict(),
      z
        .object({
          kind: z.literal('supportedSubjectToOverlap'),
          required: z.array(variable),
          available: z.array(variable),
          selectedSupported: z.boolean(),
        })
        .strict(),
      z
        .object({
          kind: z.literal('notEstablished'),
          potentialSet: z.array(variable),
          treatedBlockers: z.array(blocker),
          comparisonBlockers: z.array(blocker),
        })
        .strict(),
    ]),
    noWithinPeriodTreatmentCovariateEffect: z.boolean(),
    noFutureTreatmentCovariateEffect: z.boolean(),
    noDirectCovariateOutcomeDynamics: z.boolean(),
    noWithinPeriodCovariateOutcomeEffect: z.boolean(),
  })
  .strict()
export type SwigDidAssessment = z.infer<typeof didAssessmentSchema>
