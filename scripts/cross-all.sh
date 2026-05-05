#!/bin/bash
# Cross-compile tree-sitter-all for all 6 desktop targets + collect artifacts.
# Must be run on macOS (native Apple targets, zig for Linux/Windows cross).
#
# Prerequisites: cargo, zig, cargo-zigbuild, cargo-xwin
#   cargo install cargo-zigbuild cargo-xwin

set -euo pipefail

# tree-sitter 0.19/0.20 (used by old grammar crates) has pointer type mismatches
# that Clang 22+ treats as errors. Downgrade to warning.
export CFLAGS="${CFLAGS:-} -Wno-error=incompatible-pointer-types"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
NATIVE_DIR="$SCRIPT_DIR/../native"
CROSS_DIR="$NATIVE_DIR/target/cross"

DESKTOP_TARGETS=(
  "x86_64-apple-darwin:macos-x86_64"
  "aarch64-apple-darwin:macos-aarch64"
  "x86_64-unknown-linux-gnu:linux-x86_64"
  "aarch64-unknown-linux-gnu:linux-aarch64"
  "x86_64-pc-windows-msvc:windows-x86_64"
  "aarch64-pc-windows-msvc:windows-aarch64"
)

echo "=== Building all 6 desktop targets ==="

for entry in "${DESKTOP_TARGETS[@]}"; do
  IFS=: read -r rust_target classifier <<< "$entry"
  echo ""
  echo "--- $classifier ($rust_target) ---"

  case "$rust_target" in
    *-apple-darwin)
      cargo build --release --target "$rust_target" --manifest-path "$NATIVE_DIR/Cargo.toml"
      ;;
    *-linux-gnu)
      cargo zigbuild --release --target "$rust_target" --manifest-path "$NATIVE_DIR/Cargo.toml"
      ;;
    *-windows-msvc)
      cargo xwin build --release --target "$rust_target" --manifest-path "$NATIVE_DIR/Cargo.toml"
      ;;
  esac
done

echo ""
echo "=== Collecting artifacts ==="

rm -rf "$CROSS_DIR"

for entry in "${DESKTOP_TARGETS[@]}"; do
  IFS=: read -r rust_target classifier <<< "$entry"
  src_dir="$NATIVE_DIR/target/$rust_target/release"
  dest_dir="$CROSS_DIR/$classifier"
  mkdir -p "$dest_dir"

  # Static archives (.a / .lib)
  for f in libtree_sitter_all.a tree_sitter_all.lib; do
    [ -f "$src_dir/$f" ] && cp "$src_dir/$f" "$dest_dir/"
  done

  # Shared libraries (.dylib / .so / .dll)
  for f in libtree_sitter_all.dylib libtree_sitter_all.so tree_sitter_all.dll tree_sitter_all.dll.lib; do
    [ -f "$src_dir/$f" ] && cp "$src_dir/$f" "$dest_dir/"
  done

  echo "  $classifier: $(ls "$dest_dir" | wc -l | tr -d ' ') files"
done

echo ""
echo "=== Done: artifacts in $CROSS_DIR ==="
