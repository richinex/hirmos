/**
 * React for tests that mount a component inside the page.
 *
 * Vite resolves these imports, so no test has to name the optimised-dependency path. That path
 * carries a hash Vite mints each time it rebuilds dependencies, and it refuses the path without the
 * current hash, so a test that hard-codes one fails as soon as dependencies are rebuilt.
 */
export { createElement, default as React } from 'react'
export { createRoot } from 'react-dom/client'
export { flushSync } from 'react-dom'
