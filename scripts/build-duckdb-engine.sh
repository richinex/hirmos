#!/usr/bin/env bash
# Builds the DuckDB engine Hirmos ships: DuckDB-Wasm at a pinned commit, DuckDB at the v2.0.0 tag,
# with Parquet and ICU linked in, so no extension is downloaded at run time. Everything lands in
# .build/ (gitignored): the toolchain, the source and the engine. The bundled worker is copied into
# src/data/engine/, which is tracked.
#   scripts/build-duckdb-engine.sh            # base: single-threaded, the bundle every browser runs
#   scripts/build-duckdb-engine.sh threads    # threads: needs a cross-origin isolated page
set -euo pipefail

FEATURES="${1:-base}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD="$ROOT/.build"
SOURCE="$BUILD/duckdb-wasm"
DUCKDB_WASM_COMMIT=1f85a503e0c25f0c0980f205915de328c923f476
DUCKDB_TAG=v2.0.0
EMSCRIPTEN_VERSION=6.0.9
DUCKDB_WASM_VERSION=1.33.1-dev110.0
CORES="${CORES:-$(sysctl -n hw.ncpu 2>/dev/null || nproc)}"

mkdir -p "$BUILD"

if [ ! -d "$BUILD/emsdk" ]; then
  git clone --depth 1 https://github.com/emscripten-core/emsdk.git "$BUILD/emsdk"
fi
"$BUILD/emsdk/emsdk" install "$EMSCRIPTEN_VERSION"
"$BUILD/emsdk/emsdk" activate "$EMSCRIPTEN_VERSION"
# shellcheck disable=SC1091
source "$BUILD/emsdk/emsdk_env.sh"

if [ ! -d "$SOURCE/.git" ]; then
  mkdir -p "$SOURCE"
  git -C "$SOURCE" init -q
  git -C "$SOURCE" remote add origin https://github.com/duckdb/duckdb-wasm.git
fi
git -C "$SOURCE" fetch -q --depth 1 origin "$DUCKDB_WASM_COMMIT"
git -C "$SOURCE" checkout -q --force FETCH_HEAD
git -C "$SOURCE" submodule update --init --depth 1 --recommend-shallow
git -C "$SOURCE/submodules/duckdb" fetch -q --depth 1 origin "refs/tags/$DUCKDB_TAG:refs/tags/$DUCKDB_TAG"
if [ "$(git -C "$SOURCE/submodules/duckdb" rev-parse HEAD)" != "$(git -C "$SOURCE/submodules/duckdb" rev-parse "$DUCKDB_TAG^{commit}")" ]; then
  echo "The pinned DuckDB-Wasm commit does not carry DuckDB $DUCKDB_TAG." >&2
  exit 1
fi

make -C "$SOURCE" apply_patches
python3 "$ROOT/scripts/duckdb-engine/apply-icu.py" "$SOURCE" "$ROOT/scripts/duckdb-engine"

if [ ! -x "$BUILD/tools/node_modules/.bin/js-beautify" ]; then
  mkdir -p "$BUILD/tools"
  npm install --prefix "$BUILD/tools" --no-audit --no-fund js-beautify
fi
export PATH="$BUILD/tools/node_modules/.bin:$PATH"
export CORES OVERRIDE_GIT_DESCRIBE="$DUCKDB_TAG" DUCKDB_WASM_VERSION
unset DUCKDB_WASM_LOADABLE_EXTENSIONS

rm -rf "$SOURCE/build/relsize/$FEATURES"
(cd "$SOURCE" && ./scripts/wasm_build_lib.sh relsize "$FEATURES")

node "$ROOT/scripts/duckdb-engine/bundle.mjs" "$FEATURES"
