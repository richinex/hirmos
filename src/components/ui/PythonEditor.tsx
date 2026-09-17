import { useEffect, useRef, useState, type CSSProperties } from 'react'
import { Icon } from '@/components/Icon'
import { iconControl } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands'
import { python } from '@codemirror/lang-python'
import { bracketMatching, HighlightStyle, indentOnInput, indentUnit, syntaxHighlighting } from '@codemirror/language'
import { Compartment, EditorState } from '@codemirror/state'
import { drawSelection, EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers, placeholder as placeholderOf } from '@codemirror/view'
import { tags } from '@lezer/highlight'

const houseHighlight = HighlightStyle.define([
  { tag: [tags.keyword, tags.controlKeyword, tags.operatorKeyword, tags.definitionKeyword, tags.moduleKeyword], color: 'var(--color-info)' },
  { tag: [tags.string, tags.special(tags.string)], color: 'var(--color-ok)' },
  { tag: [tags.number, tags.bool, tags.null, tags.atom], color: 'var(--color-warn)' },
  { tag: [tags.comment, tags.lineComment, tags.blockComment], color: 'var(--color-faint)', fontStyle: 'italic' },
  { tag: [tags.function(tags.variableName), tags.function(tags.propertyName)], color: 'var(--color-ink)' },
  { tag: [tags.definition(tags.variableName), tags.className, tags.typeName], color: 'var(--color-ink)', fontWeight: '500' },
  { tag: [tags.operator, tags.punctuation, tags.bracket], color: 'var(--color-bone)' },
  { tag: tags.self, color: 'var(--color-info)' },
  { tag: tags.invalid, color: 'var(--color-danger)' },
])

const houseTheme = EditorView.theme({
  '&': { minHeight: 'var(--editor-min-height)', backgroundColor: 'var(--color-well)', color: 'var(--color-ink)', fontSize: '12px', borderRadius: '6px', border: '1px solid var(--color-control)' },
  '&.cm-focused': { outline: 'none', borderColor: 'color-mix(in srgb, var(--color-signal) 60%, transparent)' },
  '.cm-scroller': { fontFamily: 'var(--font-mono)', lineHeight: '1.6', scrollbarGutter: 'stable' },
  '.cm-content.cm-lineWrapping .cm-line': { paddingRight: '28px' },
  '.cm-content': { padding: '8px 0', caretColor: 'var(--color-ink)' },
  '.cm-line': { padding: '0 10px' },
  '.cm-gutters': { backgroundColor: 'transparent', color: 'var(--color-faint)', border: 'none', borderRight: '1px solid var(--color-line)', minWidth: '2.4em' },
  '.cm-lineNumbers .cm-gutterElement': { padding: '0 8px 0 6px', fontVariantNumeric: 'tabular-nums' },
  '.cm-activeLine': { backgroundColor: 'var(--color-highlight)' },
  '.cm-activeLineGutter': { backgroundColor: 'transparent', color: 'var(--color-muted)' },
  '.cm-cursor, .cm-dropCursor': { borderLeftColor: 'var(--color-ink)' },
  '&.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground, .cm-selectionBackground': { backgroundColor: 'color-mix(in srgb, var(--color-signal) 22%, transparent)' },
  '.cm-matchingBracket': { backgroundColor: 'color-mix(in srgb, var(--color-signal) 18%, transparent)', outline: '1px solid var(--color-signal-line)' },
  '.cm-placeholder': { color: 'var(--color-faint)', fontStyle: 'italic' },
})

/**
 * A small CodeMirror 6 editor for Python: line numbers, indentation on Enter, Tab to indent, bracket
 * matching and syntax colours from the house tokens. No autocomplete, no search panel, no fold gutter;
 * the block scripts are a few lines over pandas and the inspector column is narrow. Long lines wrap,
 * as Hex and JupyterLab do in a narrow pane, with a control to turn wrapping off and scroll instead.
 */
export function PythonEditor({ value, onChange, label, placeholder, minHeight = '14rem', className }: {
  readonly value: string
  readonly onChange: (value: string) => void
  /** The accessible name of the editor's text area. */
  readonly label: string
  readonly placeholder?: string
  readonly minHeight?: string
  readonly className?: string
}) {
  const host = useRef<HTMLDivElement>(null)
  const view = useRef<EditorView | null>(null)
  const latestOnChange = useRef(onChange)
  useEffect(() => { latestOnChange.current = onChange }, [onChange])
  const [attributes] = useState(() => new Compartment())
  const [wrapSlot] = useState(() => new Compartment())
  const [wrapEnabled, setWrapEnabled] = useState(true)

  useEffect(() => {
    const parent = host.current
    if (parent === null) return
    const state = EditorState.create({
      doc: value,
      extensions: [
        lineNumbers(),
        highlightActiveLineGutter(),
        highlightActiveLine(),
        history(),
        drawSelection(),
        bracketMatching(),
        indentOnInput(),
        indentUnit.of('    '),
        python(),
        syntaxHighlighting(houseHighlight),
        houseTheme,
        keymap.of([indentWithTab, ...defaultKeymap, ...historyKeymap]),
        attributes.of(EditorView.contentAttributes.of({ 'aria-label': label })),
        wrapSlot.of(EditorView.lineWrapping),
        placeholder === undefined ? [] : placeholderOf(placeholder),
        EditorView.updateListener.of((update) => { if (update.docChanged) latestOnChange.current(update.state.doc.toString()) }),
      ],
    })
    const created = new EditorView({ state, parent })
    view.current = created
    return () => { created.destroy(); view.current = null }
  }, [])

  useEffect(() => {
    const current = view.current
    if (current === null) return
    current.dispatch({ effects: attributes.reconfigure(EditorView.contentAttributes.of({ 'aria-label': label })) })
  }, [attributes, label])

  useEffect(() => {
    const current = view.current
    if (current === null) return
    current.dispatch({ effects: wrapSlot.reconfigure(wrapEnabled ? EditorView.lineWrapping : []) })
  }, [wrapEnabled, wrapSlot])

  useEffect(() => {
    const current = view.current
    if (current === null) return
    const shown = current.state.doc.toString()
    if (shown === value) return
    current.dispatch({ changes: { from: 0, to: shown.length, insert: value } })
  }, [value])

  return (
    <div className={cn('relative', className)}>
      <div ref={host} style={{ '--editor-min-height': minHeight } as CSSProperties} data-testid="python-editor" />
      <button
        type="button"
        className={iconControl(wrapEnabled ? 'selected' : 'quiet', 'absolute right-1.5 top-1.5 h-6 w-6 rounded-md')}
        aria-pressed={wrapEnabled}
        aria-label="Wrap long lines"
        title={wrapEnabled ? 'Long lines wrap. Click to scroll them instead.' : 'Long lines scroll. Click to wrap them.'}
        onClick={() => setWrapEnabled((current) => !current)}
      >
        <Icon name="wrap_text" size={14} />
      </button>
    </div>
  )
}
