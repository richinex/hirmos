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

const DAY = new Intl.DateTimeFormat('en-GB', { day: 'numeric', month: 'short', year: 'numeric' })

/** Joins and lowercases the day period ("2:33 PM" becomes "2:33pm"); month names keep their capital. */
const normalisePeriod = (value: string): string =>
  value.replace(/\s?(am|pm)$/iu, (period) => period.trim().toLowerCase())

/** A complete timestamp for records that may be read after the current session. */
export const formatTimestamp = (value: string): string =>
  normalisePeriod(DATE_TIME.format(new Date(value)))

/** A compact time for lists where the date is already clear from the surrounding project. */
export const formatTime = (value: string): string => normalisePeriod(TIME.format(new Date(value)))

/** A calendar day, for figures and lists where the time of day carries no information. */
export const formatDay = (value: Date | number | string): string => DAY.format(new Date(value))
