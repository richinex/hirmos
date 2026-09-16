import { expect, test } from '@playwright/test'
import type { Job } from '../src/analysis/jobs'

test('causal model actions distinguish cancellation from failure', async ({ page }) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string) => import(new URL(path, location.href).href)
    const { default: React } = await load('/node_modules/.vite/deps/react.js')
    const { default: { createRoot } } = await load('/node_modules/.vite/deps/react-dom_client.js')
    const { ActionRow } = await load('/src/components/root-cause/ActionRow.tsx')
    const render = (job: Job, action: 'analysis' | 'checks') =>
      React.createElement(ActionRow, { job, action, onRun() {}, onCancel() {} })
    const cases = {
      cancelled: render({ kind: 'cancelled', action: 'analysis' }, 'analysis'),
      checks: render({ kind: 'cancelled', action: 'checks' }, 'checks'),
      other: render({ kind: 'cancelled', action: 'checks' }, 'analysis'),
      failed: render({ kind: 'failed', action: 'analysis', detail: 'Invalid input.' }, 'analysis'),
    }
    const host = document.createElement('div')
    document.body.append(host)
    createRoot(host).render(React.createElement(React.Fragment, null, ...Object.entries(cases).map(([id, child]) => React.createElement('section', { key: id, 'data-testid': id }, child))))
  })
  await expect(page.getByTestId('cancelled').getByRole('alert')).toContainText('The analysis was cancelled.')
  await expect(page.getByTestId('cancelled').getByRole('alert')).toHaveClass(/color-info/)
  await expect(page.getByTestId('cancelled').getByRole('button', { name: 'Run analysis' })).toBeEnabled()
  await expect(page.getByTestId('checks').getByRole('alert')).toContainText('The model check was cancelled.')
  await expect(page.getByTestId('other').getByRole('alert')).toHaveCount(0)
  await expect(page.getByTestId('failed').getByRole('alert')).toHaveClass(/text-danger/)
  await expect(page.getByTestId('failed').getByRole('alert')).toContainText('Invalid input.')
})
