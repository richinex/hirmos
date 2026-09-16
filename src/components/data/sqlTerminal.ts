import { embed } from '@duckdb/duckdb-wasm-shell'
import { Terminal, type ITheme, type ITerminalAddon } from 'xterm'
import shellModule from '@duckdb/duckdb-wasm-shell/dist/shell_bg.wasm?url'

const token = (name: string, fallback: string): string => getComputedStyle(document.documentElement).getPropertyValue(name).trim() || fallback

/**
 * The window quotes macOS Terminal in its chrome; its text takes the page's tokens, because Apple's
 * palette fails 4.5:1 on our grounds (green, yellow and cyan on paper; red, blue and magenta on black)
 * and every token clears it in both themes. The sixteen terminal colours the shell prints through map
 * onto the page's own tones, so a keyword or an error reads as it would anywhere else on the page.
 */
const consoleGround = (): string => token('--color-stage', '#000000')

export const consoleTheme = (): ITheme => {
  const stage = consoleGround()
  const ink = token('--color-ink', '#F2F2F0')
  const muted = token('--color-muted', '#A1A1A1')
  const signal = token('--color-signal', '#BEF264')
  const ok = token('--color-ok', signal)
  const info = token('--color-info', muted)
  const warn = token('--color-warn', signal)
  const danger = token('--color-danger', ink)
  return {
    background: stage,
    foreground: ink,
    cursor: ink,
    cursorAccent: stage,
    selectionBackground: token('--color-raised', '#1A1A1A'),
    selectionForeground: ink,
    black: stage, brightBlack: muted,
    red: danger, brightRed: danger,
    green: ok, brightGreen: ok,
    yellow: warn, brightYellow: warn,
    blue: info, brightBlue: info,
    magenta: signal, brightMagenta: signal,
    cyan: ok, brightCyan: ok,
    white: ink, brightWhite: ink,
  }
}

/** Capture the terminal created by the shell so its text and selection colours can follow the page theme. */
const embedOnce = async (host: HTMLDivElement, resolveDatabase: () => Promise<Awaited<ReturnType<Parameters<typeof embed>[0]['resolveDatabase']>>>): Promise<Terminal | null> => {
  let caught: Terminal | null = null
  const open = Terminal.prototype.open
  const attach = Terminal.prototype.attachCustomKeyEventHandler
  const loadAddon = Terminal.prototype.loadAddon
  Terminal.prototype.loadAddon = function (this: Terminal, addon: ITerminalAddon) {
    const owned = addons.get(this) ?? []
    owned.push(addon)
    addons.set(this, owned)
    return loadAddon.call(this, addon)
  }
  Terminal.prototype.open = function (this: Terminal, parent: HTMLElement) {
    if (parent === host) caught = this
    return open.call(this, parent)
  }
  // Android keyboards compose text: every key first arrives as a keydown named "Unidentified" (keyCode 229)
  // and the characters follow through composition events. The shell must not see those keydowns, or it
  // advances the cursor on them; xterm turns the composed text into data, and the bridge replays that.
  Terminal.prototype.attachCustomKeyEventHandler = function (this: Terminal, handler: (event: KeyboardEvent) => boolean) {
    return attach.call(this, (event: KeyboardEvent) => (event.isComposing || event.keyCode === 229 || event.key === 'Unidentified' || event.key === 'Process') ? true : handler(event))
  }
  try {
    await embed({
      shellModule,
      container: host,
      resolveDatabase,
      backgroundColor: consoleGround(),
      fontFamily: token('--font-mono', 'ui-monospace, monospace'),
    })
  } catch (cause) {
    if (caught !== null) disposeTerminal(caught)
    host.onresize = null
    host.replaceChildren()
    throw cause
  } finally {
    Terminal.prototype.open = open
    Terminal.prototype.attachCustomKeyEventHandler = attach
    Terminal.prototype.loadAddon = loadAddon
  }
  if (caught !== null) (caught as Terminal).options.theme = consoleTheme()
  return caught
}

const addons = new WeakMap<Terminal, ITerminalAddon[]>()
export function disposeTerminal(terminal: Terminal) {
  // WebGL restores the default renderer on disposal, before xterm tears it down.
  for (const addon of (addons.get(terminal) ?? []).reverse()) addon.dispose()
  addons.delete(terminal)
  terminal.dispose()
}

// The upstream shell installs process-wide WASM bindings and xterm handlers.
// Serialize initialization so a closing project's embed cannot overwrite its replacement.
let embedding: Promise<unknown> = Promise.resolve()
export const embedThemed: typeof embedOnce = (host, resolveDatabase) => {
  const next = embedding.then(() => embedOnce(host, resolveDatabase))
  embedding = next.catch(() => undefined)
  return next
}

/**
 * The shell reads keystrokes only through xterm's custom key handler, which sees key names on keydown.
 * A phone's soft keyboard delivers text through input and composition events, which xterm turns into
 * data, and a paste arrives the same way; neither reaches the shell. The bridge replays that data to the
 * shell as key events. A physical key the shell consumes never produces data, so nothing is replayed
 * twice, and the flag guards the one path where it could.
 */
export const bridgeSoftKeyboard = (terminal: Terminal): (() => void) => {
  let replaying = false
  const keyFor = (char: string): string => (char === '\r' || char === '\n' ? 'Enter' : char === '\x7f' || char === '\b' ? 'Backspace' : char)
  const subscription = terminal.onData((data) => {
    const textarea = terminal.textarea
    if (replaying || textarea === undefined) return
    replaying = true
    try {
      for (const char of data) textarea.dispatchEvent(new KeyboardEvent('keydown', { key: keyFor(char), bubbles: true, cancelable: true }))
    } finally {
      replaying = false
    }
  })
  return () => subscription.dispose()
}

/** Subscribe to the page theme and the system theme used when the page has no explicit selection. */
export const onThemeChange = (listener: () => void): (() => void) => {
  const observer = new MutationObserver(listener)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme', 'class'] })
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  media.addEventListener('change', listener)
  return () => { observer.disconnect(); media.removeEventListener('change', listener) }
}
