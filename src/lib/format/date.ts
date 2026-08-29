const DATE_TIME = new Intl.DateTimeFormat('en-GB', {
  day: 'numeric',
  month: 'long',
  year: 'numeric',
  hour: 'numeric',
  minute: '2-digit',
  hour12: true,
})

const TIME = new Intl.DateTimeFormat('en-GB', {
  hour: 'numeric',
  minute: '2-digit',
  hour12: true,
})

const normalisePeriod = (value: string): string => value.replace(/\s(am|pm)$/u, '$1').toLowerCase()

/** A complete timestamp for records that may be read after the current session. */
export const formatTimestamp = (value: string): string => normalisePeriod(DATE_TIME.format(new Date(value)))

/** A compact time for lists where the date is already clear from the surrounding project. */
export const formatTime = (value: string): string => normalisePeriod(TIME.format(new Date(value)))
