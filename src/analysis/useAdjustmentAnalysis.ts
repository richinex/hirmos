import { useEffect, useState } from 'react'
import { validateAdjustmentSets } from './client'
import {
  adjustmentValidationDesignSchema,
  type AdjustmentResponse,
  type AdjustmentValidationDesign,
} from '@/domain/adjustmentValidation'

export type AdjustmentAnalysisState =
  | { readonly kind: 'pending' }
  | { readonly kind: 'unavailable' }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'ready'; readonly value: AdjustmentResponse }

export function useAdjustmentAnalysis(
  design: AdjustmentValidationDesign | null,
): AdjustmentAnalysisState {
  const key = design === null ? null : JSON.stringify(design)
  const [completed, setCompleted] = useState<{
    readonly key: string
    readonly state: AdjustmentAnalysisState
  } | null>(null)
  useEffect(() => {
    if (key === null) return
    let active = true
    const input = adjustmentValidationDesignSchema.parse(JSON.parse(key))
    void validateAdjustmentSets(input)
      .then((result) => {
        if (!active) return
        setCompleted({
          key,
          state: result.ok
            ? { kind: 'ready', value: result.value }
            : {
                kind: 'failed',
                detail: 'Adjustment analysis could not be completed. No recommendation is shown.',
              },
        })
      })
      .catch(() => {
        if (active)
          setCompleted({
            key,
            state: {
              kind: 'failed',
              detail: 'Adjustment analysis could not be completed. No recommendation is shown.',
            },
          })
      })
    return () => {
      active = false
    }
  }, [key])
  return key === null
    ? { kind: 'unavailable' }
    : completed?.key === key
      ? completed.state
      : { kind: 'pending' }
}
