declare module '@duckdb/duckdb-wasm-shell' {
  import type * as duckdb from '@duckdb/duckdb-wasm'

  export interface ShellProps {
    readonly shellModule: RequestInfo | URL | Response | BufferSource | WebAssembly.Module
    readonly container: HTMLDivElement
    readonly resolveDatabase: (
      progress: duckdb.InstantiationProgressHandler,
    ) => Promise<duckdb.AsyncDuckDB>
    readonly backgroundColor?: string
    readonly fontFamily?: string
  }

  export function embed(props: ShellProps): Promise<void>
}
