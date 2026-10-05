import { ThemeMark } from '@/components/ThemeMark'
import { iconControl } from '@/components/ui/recipes'
import { useTheme, THEME_LABELS } from '@/components/ui/useTheme'

/** The theme button owns the choice, so switching redraws only this button; the palette rides on `<html data-theme>`. */
export function ThemeControl() {
  const theme = useTheme()
  return (
    <button
      type="button"
      onClick={theme.cycle}
      aria-label="Change theme"
      title={`Theme: ${THEME_LABELS[theme.choice]}. Switch to ${THEME_LABELS[theme.next]}`}
      className={iconControl('quiet', 'text-muted')}
    >
      <ThemeMark choice={theme.next} size={16} />
    </button>
  )
}
