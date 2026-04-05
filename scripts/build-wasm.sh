#!/bin/bash
# Build WASM artifacts: web-tree-sitter core + per-grammar .wasm files.
#
# Prerequisites:
#   npm install -g tree-sitter-cli
#   Emscripten SDK (emsdk) activated in PATH
#
# Output:
#   wasm/
#     tree-sitter.js          # web-tree-sitter runtime
#     tree-sitter.wasm        # web-tree-sitter core WASM
#     grammars/
#       tree-sitter-<lang>.wasm   # One .wasm per grammar

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WASM_DIR="$SCRIPT_DIR/../wasm"
GRAMMAR_DIR="$WASM_DIR/grammars"

# Grammar npm packages (must match the grammars in Cargo.toml)
GRAMMARS=(
  # Tier 1
  "tree-sitter-bash"
  "tree-sitter-c"
  "tree-sitter-cpp"
  "tree-sitter-c-sharp"
  "tree-sitter-css"
  "tree-sitter-go"
  "tree-sitter-html"
  "tree-sitter-java"
  "tree-sitter-javascript"
  "tree-sitter-json"
  "tree-sitter-markdown"
  "tree-sitter-python"
  "tree-sitter-regex"
  "tree-sitter-ruby"
  "tree-sitter-rust"
  "tree-sitter-scala"
  "tree-sitter-sql"
  "tree-sitter-toml"
  "tree-sitter-typescript"
  "tree-sitter-yaml"
  # Tier 2
  "tree-sitter-cmake"
  "tree-sitter-dockerfile"
  "tree-sitter-elixir"
  "tree-sitter-erlang"
  "tree-sitter-haskell"
  "tree-sitter-julia"
  "tree-sitter-kotlin"
  "tree-sitter-lua"
  "tree-sitter-make"
  "tree-sitter-ocaml"
  "tree-sitter-php"
  "tree-sitter-r"
  "tree-sitter-swift"
  "tree-sitter-vim"
  "tree-sitter-xml"
  "tree-sitter-zig"
)

rm -rf "$WASM_DIR"
mkdir -p "$GRAMMAR_DIR"

# 1. Get web-tree-sitter core from npm
echo "=== Fetching web-tree-sitter core ==="
TMPDIR=$(mktemp -d)
(
  cd "$TMPDIR"
  npm pack web-tree-sitter 2>/dev/null
  tar xzf web-tree-sitter-*.tgz
  cp package/tree-sitter.js "$WASM_DIR/"
  cp package/tree-sitter.wasm "$WASM_DIR/"
)
rm -rf "$TMPDIR"
echo "  OK: tree-sitter.js + tree-sitter.wasm"

# 2. Build each grammar as .wasm
echo ""
echo "=== Building grammar WASM files ==="
BUILT=0
FAILED=0

for grammar in "${GRAMMARS[@]}"; do
  echo -n "  $grammar... "
  TMPDIR=$(mktemp -d)
  if (
    cd "$TMPDIR"
    npm pack "$grammar" 2>/dev/null
    tar xzf ${grammar}-*.tgz 2>/dev/null
    cd package
    tree-sitter build --wasm 2>/dev/null
    # The output file name varies — find any .wasm file
    find . -maxdepth 1 -name "*.wasm" -exec cp {} "$GRAMMAR_DIR/" \;
  ); then
    echo "OK"
    BUILT=$((BUILT + 1))
  else
    echo "FAILED"
    FAILED=$((FAILED + 1))
  fi
  rm -rf "$TMPDIR"
done

echo ""
echo "=== WASM build complete: $BUILT succeeded, $FAILED failed ==="
echo "  Output: $WASM_DIR"
