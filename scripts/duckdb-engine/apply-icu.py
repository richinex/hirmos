import sys, shutil, pathlib
root = pathlib.Path(sys.argv[1])
glue = pathlib.Path(sys.argv[2])
if 'duckdb_web_icu' in (root / 'lib/CMakeLists.txt').read_text():
    print('already patched', root)
    sys.exit(0)
def edit(rel, reps):
    p = root / rel; s = p.read_text()
    for a, b in reps:
        assert s.count(a) == 1, (rel, a[:60]); s = s.replace(a, b)
    p.write_text(s)
edit('lib/CMakeLists.txt', [
 ("  add_library(\n    duckdb_web_json\n", "  add_library(\n    duckdb_web_icu\n    ${CMAKE_SOURCE_DIR}/src/extensions/icu_extension.cc)\n\n  add_library(\n    duckdb_web_json\n"),
 ("  target_link_libraries(duckdb_web duckdb duckdb_web_parquet ${DUCKDB_WEB_JSON} ${DUCKDB_WEB_TPCH} duckdb_nanoarrow rapidjson ${THREAD_LIBS})\n  target_link_libraries(duckdb_web_parquet duckdb duckdb_parquet)\n",
  "  target_link_libraries(duckdb_web duckdb duckdb_web_parquet duckdb_web_icu ${DUCKDB_WEB_JSON} ${DUCKDB_WEB_TPCH} duckdb_nanoarrow rapidjson ${THREAD_LIBS})\n  target_link_libraries(duckdb_web_parquet duckdb duckdb_parquet)\n  target_link_libraries(duckdb_web_icu duckdb duckdb_icu)\n"),
])
edit('lib/cmake/duckdb.cmake', [
 ('set(DUCKDB_EXTENSIONS "json;core_functions")', 'set(DUCKDB_EXTENSIONS "json;core_functions;icu")'),
 ("    <INSTALL_DIR>/lib/libjson_extension.a", "    <INSTALL_DIR>/lib/libjson_extension.a\n    <INSTALL_DIR>/lib/libicu_extension.a"),
 ("add_library(duckdb_json STATIC IMPORTED)", "add_library(duckdb_icu STATIC IMPORTED)\nset_property(TARGET duckdb_icu PROPERTY IMPORTED_LOCATION ${install_dir}/lib/libicu_extension.a)\ntarget_include_directories(duckdb_icu INTERFACE ${DUCKDB_SOURCE_DIR}/extension/icu/include)\n\nadd_library(duckdb_json STATIC IMPORTED)"),
 ("add_dependencies(duckdb_json duckdb_ep)", "add_dependencies(duckdb_json duckdb_ep)\nadd_dependencies(duckdb_icu duckdb_ep)"),
])
edit('lib/src/webdb.cc', [
 ('#include "duckdb/web/extensions/parquet_extension.h"\n', '#include "duckdb/web/extensions/parquet_extension.h"\n#include "duckdb/web/extensions/icu_extension.h"\n'),
 ("        duckdb_web_parquet_init(db.get());\n", "        duckdb_web_parquet_init(db.get());\n        duckdb_web_icu_init(db.get());\n"),
])
s = (root / 'scripts/wasm_build_lib.sh').read_text()
s = s.replace("    -DCMAKE_C_COMPILER_LAUNCHER=ccache \\\n    -DCMAKE_CXX_COMPILER_LAUNCHER=ccache \\\n", "")
(root / 'scripts/wasm_build_lib.sh').write_text(s)
assert 'ccache' not in s, 'ccache launcher still referenced'
(root / 'lib/src/extensions').mkdir(parents=True, exist_ok=True)
(root / 'lib/include/duckdb/web/extensions').mkdir(parents=True, exist_ok=True)
shutil.copy(glue / 'icu_extension.cc', root / 'lib/src/extensions/icu_extension.cc')
shutil.copy(glue / 'icu_extension.h', root / 'lib/include/duckdb/web/extensions/icu_extension.h')
print('patched', root)
