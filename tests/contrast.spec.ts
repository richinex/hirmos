import { expect, test } from '@playwright/test'

/**
 * The colour tokens carry their own contrast contracts as comments in index.css: `--color-faint` is the
 * smallest text allowed and must hold 4.5:1 on every ground, `--color-dim` is non-text only, and
 * `--color-control` marks an input boundary at 3:1. This encodes those sentences so a nudged hex fails
 * here instead of in someone's eyes. Values are read from the live cascade rather than parsed out of the
 * stylesheet, so `var()` chains and per-theme overrides resolve exactly as the browser resolves them.
 */

const THEMES = ['dark', 'original-light'] as const

/** Every surface a reader can meet text on. */
const GROUNDS = ['--color-stage', '--color-panel', '--color-column', '--color-well', '--color-raised'] as const

/** The text ramp, floor last. */
const TEXT = ['--color-ink', '--color-bone', '--color-muted', '--color-faint'] as const

const TOKENS = [...GROUNDS, ...TEXT, '--color-dim', '--color-control', '--color-signal', '--color-signal-ink', '--color-ok', '--color-info', '--color-warn', '--color-danger'] as const

/** WCAG 2.1 relative luminance; the channel curve is the sRGB one, not a gamma approximation. */
const luminance = ([r, g, b]: readonly [number, number, number]): number => {
  const channel = (value: number): number => {
    const scaled = value / 255
    return scaled <= 0.04045 ? scaled / 12.92 : ((scaled + 0.055) / 1.055) ** 2.4
  }
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

const ratio = (first: readonly [number, number, number], second: readonly [number, number, number]): number => {
  const a = luminance(first)
  const b = luminance(second)
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05)
}

/** The browser hands back `rgb(r, g, b)` or `rgba(r, g, b, a)`; anything with alpha would need compositing. */
const parse = (value: string): { readonly rgb: readonly [number, number, number]; readonly alpha: number } => {
  const parts = value.match(/[\d.]+/g)
  if (parts === null || parts.length < 3) throw new Error(`unreadable colour: ${value}`)
  return { rgb: [Number(parts[0]), Number(parts[1]), Number(parts[2])], alpha: parts.length > 3 ? Number(parts[3]) : 1 }
}

test('every text token holds its contrast floor on every ground, in every theme', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Token contract runs once')
  await page.goto('/app')

  const palettes = await page.evaluate(async ({ themes, tokens }) => {
    const root = document.documentElement
    const original = root.dataset['theme']
    const probe = document.createElement('span')
    document.body.append(probe)
    const read: Record<string, Record<string, string>> = {}
    for (const theme of themes) {
      root.dataset['theme'] = theme
      const style = getComputedStyle(root)
      const palette: Record<string, string> = {}
      for (const token of tokens) {
        // Round-trip through `color` so a hex literal comes back as rgb() like everything else.
        probe.style.color = style.getPropertyValue(token).trim()
        palette[token] = getComputedStyle(probe).color
      }
      read[theme] = palette
    }
    probe.remove()
    if (original === undefined) delete root.dataset['theme']
    else root.dataset['theme'] = original
    return read
  }, { themes: [...THEMES], tokens: [...TOKENS] })

  const failures: string[] = []
  const report: string[] = []

  for (const theme of THEMES) {
    const palette = palettes[theme]
    expect(palette, `theme ${theme} resolved no palette`).toBeTruthy()

    for (const token of [...TEXT, '--color-dim', '--color-control'] as const) {
      const { alpha } = parse(palette[token] ?? '')
      // A translucent text token would need compositing before it could be measured honestly.
      if (alpha !== 1 && token !== '--color-dim') failures.push(`${theme}: ${token} is translucent (${palette[token]}); the checker measures opaque colours only`)
    }

    for (const text of TEXT) {
      for (const ground of GROUNDS) {
        const value = ratio(parse(palette[text] ?? '').rgb, parse(palette[ground] ?? '').rgb)
        report.push(`${theme} ${text} on ${ground} = ${value.toFixed(2)}`)
        if (value < 4.5) failures.push(`${theme}: ${text} on ${ground} is ${value.toFixed(2)}:1, below the 4.5:1 floor`)
      }
    }

    // "text on signal", per the token's own comment.
    const onSignal = ratio(parse(palette['--color-signal-ink'] ?? '').rgb, parse(palette['--color-signal'] ?? '').rgb)
    report.push(`${theme} --color-signal-ink on --color-signal = ${onSignal.toFixed(2)}`)
    if (onSignal < 4.5) failures.push(`${theme}: --color-signal-ink on --color-signal is ${onSignal.toFixed(2)}:1, below the 4.5:1 floor`)

    // An input boundary is a UI component, so WCAG 1.4.11 asks for 3:1, not 4.5:1.
    const boundary = ratio(parse(palette['--color-control'] ?? '').rgb, parse(palette['--color-well'] ?? '').rgb)
    report.push(`${theme} --color-control on --color-well = ${boundary.toFixed(2)}`)
    if (boundary < 3) failures.push(`${theme}: --color-control on --color-well is ${boundary.toFixed(2)}:1, below the 3:1 boundary floor`)
  }

  await testInfo.attach('contrast-ratios', { body: report.join('\n'), contentType: 'text/plain' })
  expect(failures, failures.join('\n')).toEqual([])
})

test('the status ramp is readable wherever it is set as text', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Token contract runs once')
  await page.goto('/app')

  const palettes = await page.evaluate(async ({ themes, tokens }) => {
    const root = document.documentElement
    const original = root.dataset['theme']
    const probe = document.createElement('span')
    document.body.append(probe)
    const read: Record<string, Record<string, string>> = {}
    for (const theme of themes) {
      root.dataset['theme'] = theme
      const style = getComputedStyle(root)
      const palette: Record<string, string> = {}
      for (const token of tokens) {
        probe.style.color = style.getPropertyValue(token).trim()
        palette[token] = getComputedStyle(probe).color
      }
      read[theme] = palette
    }
    probe.remove()
    if (original === undefined) delete root.dataset['theme']
    else root.dataset['theme'] = original
    return read
  }, { themes: [...THEMES], tokens: [...TOKENS] })

  const failures: string[] = []
  const report: string[] = []
  const status = ['--color-ok', '--color-info', '--color-warn', '--color-danger'] as const

  for (const theme of THEMES) {
    const palette = palettes[theme]
    for (const token of status) {
      // Status inks label machine state in running text, so they answer to the text floor on panel grounds.
      for (const ground of ['--color-panel', '--color-well'] as const) {
        const value = ratio(parse(palette[token] ?? '').rgb, parse(palette[ground] ?? '').rgb)
        report.push(`${theme} ${token} on ${ground} = ${value.toFixed(2)}`)
        if (value < 4.5) failures.push(`${theme}: ${token} on ${ground} is ${value.toFixed(2)}:1, below the 4.5:1 floor`)
      }
    }
  }

  await testInfo.attach('status-contrast-ratios', { body: report.join('\n'), contentType: 'text/plain' })
  expect(failures, failures.join('\n')).toEqual([])
})
