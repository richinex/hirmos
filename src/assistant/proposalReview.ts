import { z } from 'zod'

const reviewLine = z
  .string()
  .trim()
  .min(1)
  .max(400)
  .regex(/^[^\r\n]+$/)

export const proposalReviewSchema = z.object({
  summary: reviewLine,
  checks: z.array(reviewLine).min(1).max(12).readonly(),
})

export type ProposalReview = Readonly<z.infer<typeof proposalReviewSchema>>
