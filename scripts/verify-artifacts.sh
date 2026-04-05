#!/bin/bash
# Verify all expected native artifacts exist after cross-compilation.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CROSS_DIR="$SCRIPT_DIR/../native/target/cross"
FAIL=0

echo "=== Verifying desktop artifacts ==="

# Linux: .a (static) + .so (shared)
for plat in linux-x86_64 linux-aarch64; do
  for lib in libtree_sitter_all.a libtree_sitter_all.so; do
    if [ ! -f "$CROSS_DIR/$plat/$lib" ]; then
      echo "MISSING: $plat/$lib"
      FAIL=1
    fi
  done
done

# macOS: .a (static) + .dylib (shared)
for plat in macos-x86_64 macos-aarch64; do
  for lib in libtree_sitter_all.a libtree_sitter_all.dylib; do
    if [ ! -f "$CROSS_DIR/$plat/$lib" ]; then
      echo "MISSING: $plat/$lib"
      FAIL=1
    fi
  done
done

# Windows: .lib (static) + .dll (shared)
for plat in windows-x86_64 windows-aarch64; do
  for lib in tree_sitter_all.lib tree_sitter_all.dll; do
    if [ ! -f "$CROSS_DIR/$plat/$lib" ]; then
      echo "MISSING: $plat/$lib"
      FAIL=1
    fi
  done
done

# Symbol verification (on macOS native builds)
echo ""
echo "=== Verifying exported symbols (macOS) ==="
for plat in macos-aarch64 macos-x86_64; do
  dylib="$CROSS_DIR/$plat/libtree_sitter_all.dylib"
  if [ -f "$dylib" ]; then
    # Check core API symbols
    for sym in _ts_parser_new _ts_parser_delete _ts_parser_parse_string _ts_tree_root_node \
               _ts_query_new _ts_node_type _ts_language_symbol_count; do
      if ! nm -gU "$dylib" | grep -q "$sym"; then
        echo "MISSING SYMBOL: $sym in $dylib"
        FAIL=1
      fi
    done
    # Check grammar symbols
    for lang in rust python javascript java c cpp go typescript scala bash html css json yaml; do
      if ! nm -gU "$dylib" | grep -q "_tree_sitter_${lang}\$"; then
        echo "MISSING SYMBOL: tree_sitter_${lang} in $dylib"
        FAIL=1
      fi
    done
    echo "  Symbol check passed for $plat"
  else
    echo "  Skipping symbol check for $plat (no dylib)"
  fi
done

# Size report
echo ""
echo "=== Size report ==="
for plat in macos-aarch64 linux-x86_64 windows-x86_64; do
  dir="$CROSS_DIR/$plat"
  if [ -d "$dir" ]; then
    echo "  $plat:"
    for f in "$dir"/*; do
      if [ -f "$f" ]; then
        size=$(du -h "$f" | cut -f1)
        echo "    $(basename "$f"): $size"
      fi
    done
  fi
done

echo ""
if [ "$FAIL" -eq 1 ]; then
  echo "::error::Artifact verification failed!"
  exit 1
fi
echo "=== All verifications passed ==="
