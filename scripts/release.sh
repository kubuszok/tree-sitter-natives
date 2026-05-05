#!/usr/bin/env bash
# Build all artifacts locally and create a GitHub release.
#
# This script replaces CI — it builds on the local machine which has enough
# RAM for large grammar WASM compilation (SQL needs >10GB).
#
# Usage:
#   scripts/release.sh <version>
#   scripts/release.sh 0.26.8
#
# Prerequisites:
#   - Rust toolchain with all 6 desktop targets installed
#   - cargo-zigbuild, cargo-xwin (for cross-compilation)
#   - tree-sitter-cli (npm install -g tree-sitter-cli)
#   - Node.js + npm
#   - gh CLI (authenticated)
#   - git (clean working tree on master)
#
# What it does:
#   1. Safety checks (no existing tag/release, clean tree)
#   2. Cross-compile native libraries for 6 desktop platforms
#   3. Build WASM for all grammars
#   4. Collect highlight query files
#   5. Verify all artifacts are present
#   6. Create release archives
#   7. Create git tag
#   8. Create GitHub release with archives

set -euo pipefail

# Use rustup's cargo (not Homebrew's) for cross-compilation support
export PATH="$HOME/.cargo/bin:/opt/homebrew/opt/llvm/bin:$PATH"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION="${1:?Usage: release.sh <version>}"

echo "=== tree-sitter-natives release $VERSION ==="
echo ""

# ── Safety checks ──────────────────────────────────────────────────────

echo "--- Safety checks ---"

# Clean working tree
if [ -n "$(git -C "$ROOT_DIR" status --porcelain)" ]; then
  echo "ERROR: Working tree is not clean. Commit or stash changes first."
  exit 1
fi
echo "  ✓ Working tree clean"

# On master
BRANCH=$(git -C "$ROOT_DIR" rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "master" ]; then
  echo "ERROR: Not on master branch (currently on: $BRANCH)"
  exit 1
fi
echo "  ✓ On master branch"

# Tag doesn't exist
if git -C "$ROOT_DIR" rev-parse "$VERSION" >/dev/null 2>&1; then
  echo "ERROR: Tag '$VERSION' already exists. Delete it first with:"
  echo "  git tag -d $VERSION && git push origin :refs/tags/$VERSION"
  exit 1
fi
echo "  ✓ Tag '$VERSION' does not exist"

# Release doesn't exist
if gh release view "$VERSION" --repo kubuszok/tree-sitter-natives >/dev/null 2>&1; then
  echo "ERROR: GitHub release '$VERSION' already exists. Delete it first with:"
  echo "  gh release delete $VERSION --repo kubuszok/tree-sitter-natives --yes"
  exit 1
fi
echo "  ✓ GitHub release '$VERSION' does not exist"

# Required tools
for cmd in cargo tree-sitter node npm gh; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "ERROR: Required command '$cmd' not found"
    exit 1
  fi
done
echo "  ✓ All required tools available"

echo ""

# ── Step 1: Build native libraries ────────────────────────────────────

echo "--- Step 1: Cross-compile native libraries (6 desktop targets) ---"
bash "$SCRIPT_DIR/cross-all.sh"
echo ""

# ── Step 2: Build WASM ────────────────────────────────────────────────

echo "--- Step 2: Build WASM grammars ---"
bash "$SCRIPT_DIR/build-wasm.sh"
echo ""

# ── Step 3: Collect queries ───────────────────────────────────────────

echo "--- Step 3: Collect highlight queries ---"
bash "$SCRIPT_DIR/collect-queries.sh"
echo ""

# ── Step 4: Verify artifacts ──────────────────────────────────────────

echo "--- Step 4: Verify artifacts ---"

CROSS_DIR="$ROOT_DIR/native/target/cross"
WASM_DIR="$ROOT_DIR/wasm"
QUERIES_DIR="$ROOT_DIR/queries"

ERRORS=0

# Desktop platforms
for plat in linux-x86_64 linux-aarch64 macos-x86_64 macos-aarch64 windows-x86_64 windows-aarch64; do
  dir="$CROSS_DIR/$plat"
  if [ ! -d "$dir" ] || [ -z "$(ls "$dir" 2>/dev/null)" ]; then
    echo "  ✗ MISSING: $plat"
    ERRORS=$((ERRORS + 1))
  else
    count=$(ls "$dir" | wc -l | tr -d ' ')
    echo "  ✓ $plat: $count files"
  fi
done

# WASM
if [ ! -f "$WASM_DIR/web-tree-sitter.js" ]; then
  echo "  ✗ MISSING: web-tree-sitter.js"
  ERRORS=$((ERRORS + 1))
else
  wasm_count=$(ls "$WASM_DIR/grammars"/*.wasm 2>/dev/null | wc -l | tr -d ' ')
  echo "  ✓ WASM: $wasm_count grammar files"
  if [ "$wasm_count" -lt 30 ]; then
    echo "  ⚠ WARNING: Only $wasm_count WASM grammars (expected 80+)"
  fi
fi

# Queries
if [ ! -d "$QUERIES_DIR" ]; then
  echo "  ✗ MISSING: queries directory"
  ERRORS=$((ERRORS + 1))
else
  query_count=$(find "$QUERIES_DIR" -name 'highlights.scm' | wc -l | tr -d ' ')
  echo "  ✓ Queries: $query_count grammars with highlights.scm"
fi

if [ "$ERRORS" -gt 0 ]; then
  echo ""
  echo "ERROR: $ERRORS verification failures. Fix and retry."
  exit 1
fi

echo ""

# ── Step 5: Create release archives ──────────────────────────────────

echo "--- Step 5: Create release archives ---"

DIST_DIR="$ROOT_DIR/dist"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Desktop per-platform archives
for plat in linux-x86_64 linux-aarch64 macos-x86_64 macos-aarch64; do
  tar czf "$DIST_DIR/tree-sitter-${plat}.tar.gz" -C "$CROSS_DIR/${plat}" .
  echo "  ✓ tree-sitter-${plat}.tar.gz"
done
for plat in windows-x86_64 windows-aarch64; do
  (cd "$CROSS_DIR/${plat}" && zip -r "$DIST_DIR/tree-sitter-${plat}.zip" .)
  echo "  ✓ tree-sitter-${plat}.zip"
done

# WASM archive
tar czf "$DIST_DIR/tree-sitter-wasm.tar.gz" -C "$WASM_DIR" .
echo "  ✓ tree-sitter-wasm.tar.gz"

# Queries archive
tar czf "$DIST_DIR/tree-sitter-queries.tar.gz" -C "$QUERIES_DIR" .
echo "  ✓ tree-sitter-queries.tar.gz"

echo ""

# ── Step 6: Create tag and release ───────────────────────────────────

echo "--- Step 6: Create tag '$VERSION' ---"
git -C "$ROOT_DIR" tag "$VERSION"
git -C "$ROOT_DIR" push origin "$VERSION"
echo "  ✓ Tag pushed"

echo ""
echo "--- Step 7: Create GitHub release ---"
gh release create "$VERSION" \
  --repo kubuszok/tree-sitter-natives \
  --title "tree-sitter-natives $VERSION" \
  --generate-notes \
  "$DIST_DIR"/tree-sitter-*.tar.gz \
  "$DIST_DIR"/tree-sitter-*.zip

echo ""
echo "=== Release $VERSION complete ==="
echo "  https://github.com/kubuszok/tree-sitter-natives/releases/tag/$VERSION"
