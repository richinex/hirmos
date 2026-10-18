import { StrictMode, Suspense, use, useEffect, useLayoutEffect } from 'react'
import { createRoot } from 'react-dom/client'
import { useLocation } from '@/lib/router'
import { useDocTheme } from '@/components/ui/useDocTheme'
import { updateFavicon } from '@/lib/brand'
import { useScrollActivity } from '@/lib/useScrollActivity'
import { recoverFromStalePreload } from '@/lib/preloadRecovery'
import { preloadable } from '@/lib/preloadable'
import '@xyflow/react/dist/style.css'
import '@fontsource/carlito/400.css'
import '@fontsource/carlito/400-italic.css'
import '@fontsource/carlito/700.css'
import '@fontsource/carlito/700-italic.css'
import '@fontsource/archivo-black'
import '@fontsource-variable/fira-code'
import 'xterm/css/xterm.css'
import '@/index.css'
import '@/landing/landing.css'

function Root() {
  useScrollActivity()
  const theme = useDocTheme()
  // The favicon reads computed styles, so it waits for idle time rather than delaying the first paint.
  useEffect(() => {
    if (typeof window.requestIdleCallback !== 'function') {
      const handle = window.setTimeout(updateFavicon, 300)
      return () => window.clearTimeout(handle)
    }
    const handle = window.requestIdleCallback(updateFavicon)
    return () => window.cancelIdleCallback(handle)
  }, [theme])
  const location = useLocation()
  const landing = location.pathname === '/'

  // Landing is a scrolling document; the workbench is viewport locked. Commit that distinction before
  // paint whenever client-side navigation swaps the two surfaces.
  useLayoutEffect(() => {
    document.documentElement.dataset.surface = landing ? 'landing' : 'workbench'
  }, [landing])

  return (
    <Suspense fallback={null}>{landing ? <LandingSurface /> : <AppSurface />}</Suspense>
  )
}

const loadLanding = preloadable(() => import('@/landing/Landing'))
const loadApp = preloadable(() => import('@/App'))

function LandingSurface() {
  const { Landing } = use(loadLanding())
  return <Landing />
}

function AppSurface() {
  const { default: App } = use(loadApp())
  return <App />
}

recoverFromStalePreload()

const startsOnLanding = window.location.pathname === '/'
void (startsOnLanding ? loadLanding() : loadApp()).then(() => {
  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <Root />
    </StrictMode>,
  )
  if (startsOnLanding) ('requestIdleCallback' in window ? requestIdleCallback : setTimeout)(() => void loadApp())
})
