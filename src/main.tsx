import { StrictMode, useLayoutEffect } from 'react'
import { createRoot } from 'react-dom/client'
import App from '@/App'
import { Landing } from '@/landing/Landing'
import { useLocation } from '@/lib/router'
import '@fontsource-variable/geist'
import '@fontsource-variable/geist-mono'
import 'material-symbols/sharp.css'
import '@/index.css'
import '@/landing/landing.css'

function Root() {
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
