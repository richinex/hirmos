import { Icon } from '@/components/Icon'
import { iconControl } from './recipes'

export const zoomControl = iconControl('quiet', 'rounded-none border-0')

/** Shared canvas actions; the owning chart supplies its viewport operations. Without `onFit` there is no fit button. */
export function ZoomButtons({ onIn, onOut, onFit, fitLabel = 'Fit the canvas' }: {
  readonly onIn: () => void
  readonly onOut: () => void
  readonly onFit?: () => void
  readonly fitLabel?: string
}) {
  return <>
    <button type="button" className={zoomControl} title="Zoom in" aria-label="Zoom in" onClick={onIn}><Icon name="add" size={14} /></button>
    <button type="button" className={zoomControl} title="Zoom out" aria-label="Zoom out" onClick={onOut}><Icon name="remove" size={14} /></button>
    {onFit !== undefined && <button type="button" className={zoomControl} title={fitLabel} aria-label={fitLabel} onClick={onFit}><Icon name="fit_screen" size={14} /></button>}
  </>
}
