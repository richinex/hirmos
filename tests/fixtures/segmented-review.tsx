import { useState } from 'react'
import { createRoot } from 'react-dom/client'
import { SegmentedControl } from '../../src/components/ui/SegmentedControl'

export function mount() {
  document.getElementById('root')?.remove()
  const host = document.createElement('main')
  host.style.cssText = 'padding:32px 20px;max-width:640px;margin:auto;min-height:1800px'
  document.body.append(host)
  function Example() {
    const [value, setValue] = useState('levels')
    const [changes, count] = useState(0)
    const [disabled, disable] = useState(false)
    const options = [{ value: 'levels', label: 'Keep levels' }, { value: 'difference', label: 'First difference', disabled }, { value: 'detrend', label: 'Linear detrend' }]
    const change = (next: string) => { setValue(next); count(n => n + 1) }
    return <div className="space-y-6">
      <h1 className="text-xl font-medium">Preparation settings</h1>
      <SegmentedControl ariaLabel="Transform" fill value={value} options={options} onChange={change} />
      <output data-testid="selection">{value}</output><output data-testid="changes">{changes}</output>
      <button onClick={() => disable(current => !current)}>Toggle unavailable option</button>
      <SegmentedControl ariaLabel="Chart" variant="line" value={value} options={options} onChange={change} />
      <SegmentedControl ariaLabel="Catalogue" wrap value={value} options={options} onChange={change} />
      <SegmentedControl ariaLabel="Long labels" fill value="one" onChange={() => {}} options={[{ value: 'one', label: 'Complete contiguous interval' }, { value: 'two', label: 'Lag-aware sample exclusion' }]} />
    </div>
  }
  createRoot(host).render(<Example />)
}
