import type { DatasetProfile } from './dataset'
import type { DiscoveryConfiguration } from './discovery'
import type { PreparedDatasetArtifact } from './preprocessing'
import { assertNever } from './dop'

export type DiscoverySampleReview =
  | { readonly kind: 'ready' }
  | { readonly kind: 'blocked'; readonly detail: string }
  | { readonly kind: 'review'; readonly detail: string }

export function explainSampleFailure(detail: string): string {
  switch (detail) {
    case 'PCMCI+ sample construction refused: no valid constructed samples remain':
    case 'LPCMCI sample construction refused: no valid constructed samples remain':
      return 'No usable samples remain for a required lagged test after missing-value exclusions. Review the selected variables, maximum lag and exclusion settings. Preparing a dataset does not guarantee enough observations for every test.'
    default: return detail
  }
}

/** Preparation may retain empty columns; that does not make them estimable. */
export function reviewDiscoverySamples(configuration: DiscoveryConfiguration, prepared: PreparedDatasetArtifact, profile: DatasetProfile): DiscoverySampleReview {
  if (prepared.missingness.kind !== 'lag-aware-exclusion') return { kind: 'ready' }
  const selected = new Set(prepared.columns)
  const empty = profile.columns.filter((column) => selected.has(column.id) && column.nullCount === profile.rowCount).map((column) => column.name)
  if (empty.length === 0) return { kind: 'ready' }
  const detail = `No observed values in: ${empty.join(', ')}.`
  switch (configuration.kind) {
    case 'pcmci-plus':
    case 'lpcmci': return { kind: 'blocked', detail: `${detail} Lag-aware exclusion cannot construct tests involving these variables. Deselect them in Data studio and prepare a new version, or supply observed data.` }
    case 'grace': return { kind: 'blocked', detail: `${detail} GRACE cannot initialise imputation for an entirely missing variable. Deselect these variables in Data studio and prepare a new version, or supply observed data.` }
    case 'cdnots':
    case 'cdnots-plus':
      switch (configuration.missing) {
        case 'pairwiseComplete': return { kind: 'review', detail: `${detail} CD-NOTS returns p = 1 and statistic = 0 for tests with too few complete samples. A missing link for these variables is not evidence of independence.` }
        case 'varEm': return { kind: 'blocked', detail: `${detail} VAR-EM cannot initialise imputation for an entirely missing variable. Deselect these variables in Data studio and prepare a new version, or supply observed data.` }
        default: return assertNever(configuration.missing)
      }
    case 'direct-lingam': case 'pc-stable': case 'fci': case 'jpcmci-plus': case 'rpcmci': case 'dynotears': case 'var-lingam': case 'ocse': case 'cmlp': case 'clstm': return { kind: 'ready' }
    default: return assertNever(configuration)
  }
}
