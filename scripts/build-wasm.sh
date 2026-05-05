#!/bin/bash
# Build WASM artifacts: web-tree-sitter core + per-grammar .wasm files.
#
# Strategy: try npm first, fall back to GitHub clone for grammars whose npm
# packages don't exist or are name-squatted.
#
# Prerequisites:
#   npm install -g tree-sitter-cli
#   Emscripten SDK (emsdk) activated in PATH
#
# Output:
#   wasm/
#     web-tree-sitter.js
#     web-tree-sitter.wasm
#     grammars/
#       tree-sitter-<lang>.wasm

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WASM_DIR="$SCRIPT_DIR/../wasm"
GRAMMAR_DIR="$WASM_DIR/grammars"

# Format: "output-name|npm-package|github-owner/repo|subdirectory"
# npm-package can be "-" to skip npm.
# subdirectory is for multi-grammar repos (e.g. typescript has two).
GRAMMARS=(
  # Tier 1
  "bash|tree-sitter-bash|-|-"
  "c|tree-sitter-c|-|-"
  "cpp|tree-sitter-cpp|-|-"
  "c_sharp|tree-sitter-c-sharp|-|-"
  "css|tree-sitter-css|-|-"
  "go|tree-sitter-go|-|-"
  "html|tree-sitter-html|-|-"
  "java|tree-sitter-java|-|-"
  "javascript|tree-sitter-javascript|-|-"
  "json|tree-sitter-json|-|-"
  "markdown|-|tree-sitter-grammars/tree-sitter-markdown|tree-sitter-markdown"
  "python|tree-sitter-python|-|-"
  "regex|tree-sitter-regex|-|-"
  "ruby|tree-sitter-ruby|-|-"
  "rust|tree-sitter-rust|-|-"
  "scala|tree-sitter-scala|-|-"
  "SQL|-|m-novikov/tree-sitter-sql|-"
  "toml|tree-sitter-toml|-|-"
  "typescript|tree-sitter-typescript|tree-sitter/tree-sitter-typescript|typescript"
  "tsx|tree-sitter-typescript|tree-sitter/tree-sitter-typescript|tsx"
  "yaml|-|tree-sitter-grammars/tree-sitter-yaml|-"
  # Tier 2
  "cmake|-|uyha/tree-sitter-cmake|-"
  "dockerfile|-|camdencheek/tree-sitter-dockerfile|-"
  "elixir|tree-sitter-elixir|-|-"
  "erlang|-|WhatsApp/tree-sitter-erlang|-"
  "haskell|tree-sitter-haskell|-|-"
  "julia|tree-sitter-julia|-|-"
  "kotlin|tree-sitter-kotlin|-|-"
  "lua|-|tree-sitter-grammars/tree-sitter-lua|-"
  "make|tree-sitter-make|-|-"
  "ocaml|tree-sitter-ocaml|tree-sitter/tree-sitter-ocaml|grammars/ocaml"
  "ocaml_interface|tree-sitter-ocaml|tree-sitter/tree-sitter-ocaml|grammars/interface"
  "php|tree-sitter-php|tree-sitter/tree-sitter-php|php"
  "php_only|tree-sitter-php|tree-sitter/tree-sitter-php|php_only"
  "r|-|r-lib/tree-sitter-r|-"
  "swift|tree-sitter-swift|-|-"
  "vim|-|tree-sitter-grammars/tree-sitter-vim|-"
  "dtd|-|tree-sitter-grammars/tree-sitter-xml|dtd"
  "xml|-|tree-sitter-grammars/tree-sitter-xml|xml"
  "zig|-|tree-sitter-grammars/tree-sitter-zig|-"
  # Tier 3: tree-sitter-grammars org
  "arduino|tree-sitter-arduino|-|-"
  "bicep|tree-sitter-bicep|-|-"
  "cairo|-|tree-sitter-grammars/tree-sitter-cairo|-"
  "commonlisp|tree-sitter-commonlisp|-|-"
  "cpon|-|tree-sitter-grammars/tree-sitter-cpon|-"
  "cuda|tree-sitter-cuda|-|-"
  "diff|tree-sitter-diff|tree-sitter-grammars/tree-sitter-diff|-"
  "func|-|tree-sitter-grammars/tree-sitter-func|-"
  "gitattributes|-|tree-sitter-grammars/tree-sitter-gitattributes|-"
  "glsl|tree-sitter-glsl|-|-"
  "gosum|-|tree-sitter-grammars/tree-sitter-go-sum|-"
  "hare|-|tree-sitter-grammars/tree-sitter-hare|-"
  "hcl|tree-sitter-hcl|tree-sitter-grammars/tree-sitter-hcl|-"
  "hlsl|tree-sitter-hlsl|-|-"
  "kconfig|-|tree-sitter-grammars/tree-sitter-kconfig|-"
  "kdl|tree-sitter-kdl|-|-"
  "luadoc|-|tree-sitter-grammars/tree-sitter-luadoc|-"
  "luap|-|tree-sitter-grammars/tree-sitter-luap|-"
  "luau|-|tree-sitter-grammars/tree-sitter-luau|-"
  "objc|tree-sitter-objc|-|-"
  "odin|tree-sitter-odin|-|-"
  "po|-|tree-sitter-grammars/tree-sitter-po|-"
  "pony|-|tree-sitter-grammars/tree-sitter-pony|-"
  "printf|-|tree-sitter-grammars/tree-sitter-printf|-"
  "properties|tree-sitter-properties|-|-"
  "puppet|tree-sitter-puppet|-|-"
  "ql|tree-sitter-ql|-|-"
  "qmldir|-|tree-sitter-grammars/tree-sitter-qmldir|-"
  "query|-|tree-sitter-grammars/tree-sitter-query|-"
  "requirements|tree-sitter-requirements|-|-"
  "ron|tree-sitter-ron|-|-"
  "scss|tree-sitter-scss|-|-"
  "squirrel|-|tree-sitter-grammars/tree-sitter-squirrel|-"
  "starlark|tree-sitter-starlark|-|-"
  "svelte|tree-sitter-svelte|-|-"
  "test|-|tree-sitter-grammars/tree-sitter-test|-"
  "ungrammar|-|tree-sitter-grammars/tree-sitter-ungrammar|-"
  "vue|tree-sitter-vue|-|-"
  "wgsl_bevy|-|tree-sitter-grammars/tree-sitter-wgsl-bevy|-"
  "yuck|-|tree-sitter-grammars/tree-sitter-yuck|-"
  # Tier 4: tree-sitter core org
  "agda|tree-sitter-agda|-|-"
  "embedded_template|tree-sitter-embedded-template|-|-"
  "jsdoc|tree-sitter-jsdoc|-|-"
  "verilog|tree-sitter-verilog|-|-"
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
  cp package/web-tree-sitter.js "$WASM_DIR/"
  cp package/web-tree-sitter.wasm "$WASM_DIR/"
)
rm -rf "$TMPDIR"
echo "  OK: web-tree-sitter.js + web-tree-sitter.wasm"

# 2. Build each grammar as .wasm (parallel, max concurrent)
echo ""
echo "=== Building grammar WASM files ==="

MAX_JOBS=3
export CFLAGS="-Oz"

build_one_grammar() {
  local entry="$1"
  IFS='|' read -r output_name npm_pkg gh_repo subdir <<< "$entry"
  local result_file="$GRAMMAR_DIR/tree-sitter-${output_name}.wasm"

  # Try npm first
  if [ "$npm_pkg" != "-" ]; then
    local tmpdir
    tmpdir=$(mktemp -d)
    if (
      cd "$tmpdir"
      npm pack "$npm_pkg" 2>/dev/null
      tar xzf ${npm_pkg}-*.tgz 2>/dev/null
      cd package
      tree-sitter build --wasm 2>/dev/null
      find . -name "*.wasm" -exec cp {} "$result_file" \;
    ) 2>/dev/null && [ -f "$result_file" ]; then
      rm -rf "$tmpdir"
      echo "  $output_name... OK (npm)"
      return 0
    fi
    rm -rf "$tmpdir"
  fi

  # Fall back to GitHub
  if [ "$gh_repo" != "-" ]; then
    local tmpdir
    tmpdir=$(mktemp -d)
    if (
      cd "$tmpdir"
      git clone --depth=1 "https://github.com/${gh_repo}.git" repo 2>/dev/null
      build_dir="repo"
      [ "$subdir" != "-" ] && build_dir="repo/${subdir}"
      cd "$build_dir"
      tree-sitter build --wasm 2>/dev/null
      find . -name "*.wasm" -exec cp {} "$result_file" \;
    ) 2>/dev/null && [ -f "$result_file" ]; then
      rm -rf "$tmpdir"
      echo "  $output_name... OK (github)"
      return 0
    fi
    rm -rf "$tmpdir"
  fi

  echo "  $output_name... FAILED"
  return 1
}

export -f build_one_grammar
export GRAMMAR_DIR

# Run builds in parallel
RESULTS_DIR=$(mktemp -d)
JOB_COUNT=0
PIDS=()

for entry in "${GRAMMARS[@]}"; do
  build_one_grammar "$entry" &
  PIDS+=($!)
  JOB_COUNT=$((JOB_COUNT + 1))
  # Limit concurrency
  if [ "$JOB_COUNT" -ge "$MAX_JOBS" ]; then
    wait "${PIDS[0]}" || true
    PIDS=("${PIDS[@]:1}")
    JOB_COUNT=$((JOB_COUNT - 1))
  fi
done

# Wait for remaining jobs (ignore individual failures)
for pid in "${PIDS[@]}"; do
  wait "$pid" || true
done

# Count results
BUILT=$(ls "$GRAMMAR_DIR"/tree-sitter-*.wasm 2>/dev/null | wc -l | tr -d ' ')
TOTAL=${#GRAMMARS[@]}
FAILED=$((TOTAL - BUILT))

echo ""
echo "=== WASM build: $BUILT / $TOTAL grammars ==="
echo "  Output: $WASM_DIR"
rm -rf "$RESULTS_DIR"
