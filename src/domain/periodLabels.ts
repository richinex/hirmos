/** Display source labels without interpreting internal period codes as dates. */
export function periodLabel(code: number, labels: ReadonlyMap<number, string>): string {
  const label = labels.get(code)
  if (label === undefined) throw new Error(`No source label was recorded for period ${code}.`)
  return label
}
