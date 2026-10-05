import { clsx, type ClassValue } from 'clsx'
import { extendTailwindMerge } from 'tailwind-merge'

/** twMerge must be TAUGHT the type ramp. It cannot see index.css, so an unknown `text-*` class is assumed
 *  to be a colour, which puts a size and a colour in one conflict group and silently drops the size.
 *  Declaring every ramp tier as a font size restores size beats size, colour beats colour. */
const twMerge = extendTailwindMerge({
  extend: {
    classGroups: {
      'font-size': [
        {
          text: [
            'metric',
            'display',
            'display-sub',
            'heading',
            'title',
            'subtitle',
            'body',
            'label',
            'micro',
          ],
        },
      ],
    },
  },
})

/** Tailwind-aware className combiner (the shadcn/recursis convention). */
export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs))
}
