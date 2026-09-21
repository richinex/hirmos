import * as RadixPopover from '@radix-ui/react-popover'
import { useState } from 'react'
import { DayPicker, type DateRange } from 'react-day-picker'
import { Icon } from '@/components/Icon'
import { field } from '@/components/ui/recipes'
import { describeCalendarDay, type CalendarDay } from '@/domain/pipeline'
import { cn } from '@/lib/utils'

/**
 * The reference year the calendar shows. The window is a day and month at each end and repeats
 * every year, so the year on screen is only a place to click: a leap year, so 29 February can be
 * chosen, and its successor, so a window over the new year is picked as one range.
 */
const YEAR = 2024

const toDate = (day: CalendarDay, year: number): Date => new Date(year, day.month - 1, day.day)
const toDay = (date: Date): CalendarDay => ({ day: date.getDate(), month: date.getMonth() + 1 })
const key = (day: CalendarDay): number => day.month * 100 + day.day

/** A window of the year as a range on the reference calendar: an end before its start lands in the year after. */
const rangeOf = (from: CalendarDay, to: CalendarDay): DateRange => ({ from: toDate(from, YEAR), to: toDate(to, key(to) < key(from) ? YEAR + 1 : YEAR) })

const MONTH_ONLY = new Intl.DateTimeFormat('en-GB', { month: 'long' })

/**
 * Picks the two ends of a window of the year on a two-month calendar: click the first day, then the
 * last. The first click always starts a new window rather than stretching the old one, which is the
 * library's own range behaviour, so the clicked days are read from the selection callback and the
 * pending start is held here. The field shows the window as it is stored, day and month only.
 */
export function CalendarWindowPicker({ from, to, onChange, className }: {
  readonly from: CalendarDay
  readonly to: CalendarDay
  readonly onChange: (window: { readonly from: CalendarDay; readonly to: CalendarDay }) => void
  readonly className?: string
}) {
  const [pending, setPending] = useState<Date | null>(null)
  const selected = pending === null ? rangeOf(from, to) : { from: pending, to: pending }
  return (
    <RadixPopover.Root onOpenChange={(open) => { if (!open) setPending(null) }}>
      <RadixPopover.Trigger asChild>
        <button type="button" className={cn(field('text', 'flex items-center gap-2 text-left'), className)} aria-label="Window of the year">
          <Icon name="date_range" size={16} className="text-faint" />
          <span className="flex-1">{describeCalendarDay(from)} to {describeCalendarDay(to)}</span>
          <Icon name="expand_more" size={16} className="text-faint" />
        </button>
      </RadixPopover.Trigger>
      <RadixPopover.Portal>
        <RadixPopover.Content side="bottom" align="start" sideOffset={6} collisionPadding={8} className="pop float z-(--z-dialog) rounded-md border border-line bg-panel p-3 text-ink">
          <DayPicker
            mode="range"
            required
            selected={selected}
            onSelect={(_, day) => {
              if (pending === null) { setPending(day); return }
              onChange({ from: toDay(pending), to: toDay(day) })
              setPending(null)
            }}
            defaultMonth={rangeOf(from, to).from}
            numberOfMonths={2}
            startMonth={new Date(YEAR, 0)}
            endMonth={new Date(YEAR + 1, 11)}
            max={366}
            weekStartsOn={1}
            formatters={{ formatCaption: (month) => MONTH_ONLY.format(month) }}
          />
          <p className="m-0 mt-2 text-label text-faint">Click the first day, then the last. The window repeats every year; one over the new year is picked as one range.</p>
        </RadixPopover.Content>
      </RadixPopover.Portal>
    </RadixPopover.Root>
  )
}
