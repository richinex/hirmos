import { DUCKDB_PACKAGE_VERSION } from './dataEngine'
import type { SourceFingerprint } from './dataset'
import { brand, type Brand } from './dop'
import { declarationsKey, type FileReading } from './fileReading'

export type KeptName = Brand<string, 'KeptName'>

export type KeptDecision =
  { readonly kind: 'keep'; readonly name: KeptName } | { readonly kind: 'read-file' }

const hash = (text: string): string => {
  let value = 0x811c9dc5
  for (let index = 0; index < text.length; index += 1) {
    value ^= text.charCodeAt(index)
    value = Math.imul(value, 0x01000193) >>> 0
  }
  return value.toString(16).padStart(8, '0')
}

export const keptName = (fingerprint: SourceFingerprint, reading: FileReading): KeptName =>
  brand<string, 'KeptName'>(
    `kept-${DUCKDB_PACKAGE_VERSION.replaceAll(/[^A-Za-z0-9.]/g, '_')}-${fingerprint}-${hash(
      reading.format === 'parquet' ? '' : declarationsKey(reading.declared),
    )}`,
  )

export const keptDecision = (
  fingerprint: SourceFingerprint,
  reading: FileReading,
  textColumn?: string,
): KeptDecision =>
  reading.format === 'parquet' || textColumn !== undefined
    ? { kind: 'read-file' }
    : { kind: 'keep', name: keptName(fingerprint, reading) }

export const KEPT_TABLE = 'data'
export const KEPT_META = 'meta'
