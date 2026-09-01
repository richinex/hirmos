import type { AnchorHTMLAttributes, MouseEvent } from 'react'
import { navigate } from '@/lib/router'

type InternalPath = `/${string}`

type InternalLinkProps = Omit<AnchorHTMLAttributes<HTMLAnchorElement>, 'href'> & {
  readonly href: InternalPath
}

/** Preserve ordinary link behavior for new tabs and downloads; keep plain left-click navigation
 * inside the current document so the landing page and workbench share one React root. */
export function InternalLink({ href, onClick, target, download, ...props }: InternalLinkProps) {
  const handleClick = (event: MouseEvent<HTMLAnchorElement>) => {
    onClick?.(event)
    if (
      event.defaultPrevented
      || event.button !== 0
      || event.metaKey
      || event.ctrlKey
      || event.shiftKey
      || event.altKey
      || (target !== undefined && target !== '_self')
      || download !== undefined
    ) return

    event.preventDefault()
    navigate(href)
  }

  return <a {...props} href={href} target={target} download={download} onClick={handleClick} />
}
