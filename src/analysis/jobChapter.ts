import type { ChapterId } from '@/domain/navigation'

const PREFIXES: readonly (readonly [string, ChapterId])[] = [
  ['time-series:', 'time-series'],
  ['count-regression', 'time-series'],
  ['root-cause:', 'root-cause'],
  ['effects:', 'root-cause'],
  ['influence:', 'root-cause'],
  ['intervention:', 'dag'],
  ['graph-check:', 'dag'],
  ['swig:', 'dag'],
  ['identification', 'study'],
  ['raw-balance', 'study'],
  ['estimation', 'estimation'],
  ['sensitivity', 'sensitivity'],
  ['counterfactual', 'counterfactual'],
  ['discovery', 'discovery'],
  ['survival', 'survival'],
  ['power-planning', 'data'],
  ['preparation', 'data'],
  ['stationarity', 'data'],
  ['series-structure', 'data'],
  ['redundancy', 'data'],
  ['granger', 'data'],
]

export function chapterOfJob(key: string): ChapterId | null {
  return PREFIXES.find(([prefix]) => key === prefix || key.startsWith(prefix))?.[1] ?? null
}
