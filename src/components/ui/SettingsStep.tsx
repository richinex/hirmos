import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'
import { ParameterHelp } from './ParameterLabel'
import { settingsStack, stepTitle } from './recipes'

export function SettingsStep({ number, title, help, children, className }: {
  readonly number?: number
  readonly title: string
  /** What the whole step does, behind an info icon beside the title. */
  readonly help?: string
  readonly children: ReactNode
  readonly className?: string
}) {
  return (
    <fieldset className={cn('m-0 min-w-0 border-0 p-0', className)}>
      <legend className={cn(stepTitle, 'p-0')}>
        {number !== undefined && <span className="mr-1.5 text-faint">{number}</span>}
        <span>{title}</span>
        {help !== undefined && <span className="ml-1.5 inline-flex align-middle"><ParameterHelp label={title} help={help} /></span>}
      </legend>
      <div className={cn(settingsStack, 'mt-4')}>{children}</div>
    </fieldset>
  )
}
