import { createContext, useContext, useEffect, useState } from 'react'
import { Icon } from '@/components/Icon'
import { iconControl } from '@/components/ui/recipes'
import type { DeskWindowRef } from '@/domain/resultsDesk'
import { putOnDesk } from './deskStore'

/** True inside a desk window, where a result is already on the desk. */
export const OnDesk = createContext(false)

export function PutOnDesk({
  deskRef,
  className,
  iconSize = 16,
}: {
  readonly deskRef: DeskWindowRef
  readonly className?: string
  readonly iconSize?: number
}) {
  const onDesk = useContext(OnDesk)
  const [placed, setPlaced] = useState(false)
  useEffect(() => {
    if (!placed) return
    const timer = setTimeout(() => setPlaced(false), 1600)
    return () => clearTimeout(timer)
  }, [placed])
  const label = placed ? 'On the Results desk' : 'Put on the Results desk'
  if (onDesk) return null
  return (
    <button
      type="button"
      className={iconControl('quiet', className)}
      aria-label={label}
      title={label}
      onClick={(event) => {
        event.preventDefault()
        event.stopPropagation()
        putOnDesk(deskRef)
        setPlaced(true)
      }}
    >
      <Icon name={placed ? 'check' : 'keep'} size={iconSize} />
    </button>
  )
}
