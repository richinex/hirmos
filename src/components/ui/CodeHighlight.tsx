import { Fragment, memo, useEffect, useMemo, useState } from 'react'
import { jsx, jsxs } from 'react/jsx-runtime'
import { toJsxRuntime } from 'hast-util-to-jsx-runtime'
import type { HighlighterCore } from 'shiki/core'
import { cn } from '@/lib/utils'
import { CopyButton } from './CopyButton'
import { useDocTheme } from './useDocTheme'

/**
 * Syntax-highlighted code.
 *
 * This reverses the standing rule that code stays monochrome, because colour already means a node's
 * category on the plot and `--color-signal` means "this is live". Agent Python is now the main content of
 * the execution log, the sub-agent panel and the chat trace, and weight alone cannot mark a string.
 *
 * It uses Shiki's JavaScript regex engine, not oniguruma, which is a ~1MB WASM payload. Everything loads
 * on demand, and a block renders as plain text in the same chrome until the highlighter arrives, so it
 * does not reflow when the colour lands.
 */

// The languages an agent writes here. A fixed list, because Shiki's full registry is most of its weight.
const LANGS = ['python', 'json', 'sql'] as const
type Lang = (typeof LANGS)[number]
const asLang = (l: string): Lang =>
  (LANGS as readonly string[]).includes(l) ? (l as Lang) : 'python'

/** One Shiki theme per appearance. */
const themeFor = (theme: string): 'github-light' | 'tokyo-night' =>
  theme === 'dark' ? 'tokyo-night' : 'github-light'

/** Created once per tab and shared: the grammars and themes are the same for every block on screen. */
let shared: HighlighterCore | null = null
let loading: Promise<HighlighterCore> | null = null
const load = (): Promise<HighlighterCore> => {
  loading ??= (async () => {
    // The engine and the core are imported here, not at module scope. Statically they added 224kB to the
    // main bundle for a component most sessions never mount.
    const [
      { createHighlighterCore },
      { createJavaScriptRegexEngine },
      python,
      json,
      sql,
      light,
      dark,
    ] = await Promise.all([
      import('shiki/core'),
      import('shiki/engine/javascript'),
      import('@shikijs/langs/python'),
      import('@shikijs/langs/json'),
      import('@shikijs/langs/sql'),
      import('@shikijs/themes/github-light'),
      import('@shikijs/themes/tokyo-night'),
    ])
    shared = await createHighlighterCore({
      langs: [python.default, json.default, sql.default],
      themes: [light.default, dark.default],
      engine: createJavaScriptRegexEngine(),
    })
    return shared
  })()
  return loading
}

export const CodeHighlight = memo(function CodeHighlight({
  code,
  lang = 'python',
  className,
  copyable = false,
  copyLabel,
}: {
  code: string
  lang?: string
  className?: string
  /**
   * Offer a copy button over the block.
   *
   * Opt-in, because this component also renders one-line previews and truncated fragments, where a button
   * offering half a line is worse than no button.
   */
  copyable?: boolean
  copyLabel?: string
}) {
  const theme = useDocTheme()
  const [hl, setHl] = useState<HighlighterCore | null>(shared)
  useEffect(() => {
    if (hl) return
    let live = true
    void load()
      .then((h) => {
        if (live) setHl(h)
      })
      .catch(() => {
        /* plain text is a fine outcome */
      })
    return () => {
      live = false
    }
  }, [hl])

  const rendered = useMemo(() => {
    if (!hl) return null
    try {
      // Real React elements rather than dangerouslySetInnerHTML: this is model-authored text, and it must
      // never be parsed as markup on its way to the screen.
      return toJsxRuntime(hl.codeToHast(code, { lang: asLang(lang), theme: themeFor(theme) }), {
        Fragment,
        jsx,
        jsxs,
      })
    } catch {
      return null // an unknown grammar is not a reason to show the reader nothing
    }
  }, [hl, code, lang, theme])

  if (!copyable) {
    return (
      <div className={cn('shiki-host', className)}>
        {rendered ?? (
          <pre className="shiki-plain">
            <code>{code}</code>
          </pre>
        )}
      </div>
    )
  }
  // The button appears on hover or keyboard focus, so a dense log is not a grid of icons at rest.
  // `focus-within` keeps it reachable without a pointer, and the right padding stops it covering line one.
  return (
    <div className={cn('group/code relative', className)}>
      <div className="shiki-host pr-7">
        {rendered ?? (
          <pre className="shiki-plain">
            <code>{code}</code>
          </pre>
        )}
      </div>
      <CopyButton
        text={code}
        label={copyLabel ?? 'Copy code'}
        className="absolute right-0 top-0 opacity-0 pointer-coarse:opacity-100 transition-opacity group-hover/code:opacity-100 group-focus-within/code:opacity-100 focus-visible:opacity-100"
      />
    </div>
  )
})
