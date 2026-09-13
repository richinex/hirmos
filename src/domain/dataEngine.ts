/** Versions used to parse new sources; older recorded profiles keep their version. */
export const DUCKDB_PACKAGE_VERSION = '1.33.1-dev57.0'
export const DUCKDB_ENGINE_VERSION = 'v1.5.4'
export const DATA_PARSER_VERSIONS = ['1.30.0', DUCKDB_PACKAGE_VERSION] as const
export type DataParserVersion = typeof DATA_PARSER_VERSIONS[number]
