import { StrictMode, useLayoutEffect } from 'react'
import { createRoot } from 'react-dom/client'
import App from '@/App'
import { Landing } from '@/landing/Landing'
import { useLocation } from '@/lib/router'
import { useDocTheme } from '@/components/ui/useDocTheme'
import { updateFavicon } from '@/lib/brand'
import { useScrollActivity } from '@/lib/useScrollActivity'
// The three faces: Roboto Slab for headings, Lato in its two weights for everything read or operated,
// Fira Code for what is read character by character. Self-hosted; each stack in index.css names a
// web-safe face after it.
import '@fontsource-variable/roboto-slab'
import '@fontsource/lato/400.css'
import '@fontsource/lato/700.css'
import '@fontsource/lato/400-italic.css'
import '@fontsource/lato/700-italic.css'
import '@fontsource-variable/fira-code'
import 'xterm/css/xterm.css'
import 'material-symbols/sharp.css'
import '@/index.css'
import '@/landing/landing.css'

function Root() {
  useScrollActivity()
  const theme = useDocTheme()
  useLayoutEffect(updateFavicon, [theme])
  const location = useLocation()
  const landing = location.pathname === '/'

  // Landing is a scrolling document; the workbench is viewport locked. Commit that distinction before
  // paint whenever client-side navigation swaps the two surfaces.
  useLayoutEffect(() => {
    document.documentElement.dataset.surface = landing ? 'landing' : 'workbench'
  }, [landing])

  return landing ? <Landing /> : <App />
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Root />
  </StrictMode>,
)
