/** Versions used to parse new sources; older recorded profiles keep their version. */
export const DUCKDB_PACKAGE_VERSION = '2.0.0-hirmos.1'
export const DUCKDB_ENGINE_VERSION = 'v2.0.0'
export const DATA_PARSER_VERSIONS = ['1.30.0', '1.33.1-dev57.0', DUCKDB_PACKAGE_VERSION] as const
export type DataParserVersion = (typeof DATA_PARSER_VERSIONS)[number]
