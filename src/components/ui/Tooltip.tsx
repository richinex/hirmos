import * as RadixTooltip from '@radix-ui/react-tooltip'
import type { ReactNode } from 'react'

/**
 * A short note that belongs to one element: shown on hover and on keyboard focus, read by assistive
 * technology as the trigger's description. Radix supplies the timing, positioning and focus rules;
 * the surface is the house pop surface. Keep the text to one sentence; anything longer is copy.
 */
export function Tooltip({ text, children }: { readonly text: string; readonly children: ReactNode }) {
  return (
    <RadixTooltip.Provider delayDuration={250}>
      <RadixTooltip.Root>
        <RadixTooltip.Trigger asChild>{children}</RadixTooltip.Trigger>
        <RadixTooltip.Portal>
          <RadixTooltip.Content
            side="top"
            align="end"
            sideOffset={6}
            collisionPadding={8}
            className="pop float z-(--z-dialog) max-w-[28ch] rounded-md border border-line bg-panel px-2.5 py-1.5 text-label leading-snug text-ink"
          >
            {text}
            <RadixTooltip.Arrow className="fill-[var(--color-line)]" width={10} height={5} />
          </RadixTooltip.Content>
        </RadixTooltip.Portal>
      </RadixTooltip.Root>
    </RadixTooltip.Provider>
  )
}
