/**
 * CSV serialisation for exported evidence.
 *
 * Values are escaped for the spreadsheet that opens them, not only for the CSV grammar: a cell
 * beginning with a formula character is executed on open by Excel, LibreOffice and Sheets, so a
 * variable named `=rate` in a discovery run would run as a formula in the reader's spreadsheet.
 */

/** Excel, LibreOffice and Sheets all treat a leading one of these as the start of a formula. */
const FORMULA_LEAD = /^[=+\-@\t\r]/

/**
 * Escape one value for a CSV cell, neutralising any leading formula character.
 *
 * Only text is guarded. A number is already typed, and a negative one would otherwise be exported
 * as text by its leading minus, which would break every figure the reader tries to compute with.
 */
export function csvCell(value: string | number): string {
  if (typeof value === 'number') return String(value)
  // A leading apostrophe is the conventional spreadsheet escape: it forces text and is not displayed.
  const guarded = FORMULA_LEAD.test(value) ? `'${value}` : value
  return /[",\n\r]/.test(guarded) ? `"${guarded.replaceAll('"', '""')}"` : guarded
}

/** Join a header row and body rows into CSV text with CRLF line endings. */
export function toCsv(
  headers: readonly string[],
  rows: readonly (readonly (string | number)[])[],
): string {
  const lines = [headers.map(csvCell).join(','), ...rows.map((row) => row.map(csvCell).join(','))]
  return `${lines.join('\r\n')}\r\n`
}
