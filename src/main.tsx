import { StrictMode, useLayoutEffect } from 'react'
import { createRoot } from 'react-dom/client'
import App from '@/App'
import { Landing } from '@/landing/Landing'
import { useLocation } from '@/lib/router'
import { useDocTheme } from '@/components/ui/useDocTheme'
import { updateFavicon } from '@/lib/brand'
// Arimo is fetched only where neither Helvetica nor Arial is installed: the stack names it third.
import '@fontsource/arimo/400.css'
import '@fontsource/arimo/700.css'
import '@fontsource/arimo/400-italic.css'
import '@fontsource-variable/jetbrains-mono'
import '@fontsource-variable/jetbrains-mono/wght-italic.css'
import 'xterm/css/xterm.css'
import 'material-symbols/sharp.css'
import '@/index.css'
import '@/landing/landing.css'

function Root() {
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
