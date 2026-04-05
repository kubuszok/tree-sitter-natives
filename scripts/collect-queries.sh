#!/bin/bash
# Collect highlight/injection/locals query files (.scm) from grammar npm packages.
#
# Output:
#   queries/
#     bash/highlights.scm
#     python/highlights.scm
#     python/locals.scm
#     ...

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
QUERIES_DIR="$SCRIPT_DIR/../queries"

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

rm -rf "$QUERIES_DIR"
mkdir -p "$QUERIES_DIR"

echo "=== Collecting query files ==="
COLLECTED=0

for grammar in "${GRAMMARS[@]}"; do
  # Extract language name from package name (tree-sitter-foo → foo)
  lang="${grammar#tree-sitter-}"
  echo -n "  $lang... "

  TMPDIR=$(mktemp -d)
  if (
    cd "$TMPDIR"
    npm pack "$grammar" 2>/dev/null
    tar xzf ${grammar}-*.tgz 2>/dev/null
  ); then
    # Look for queries in common locations
    FOUND=0
    for queries_path in "$TMPDIR/package/queries" "$TMPDIR/package/src/queries"; do
      if [ -d "$queries_path" ]; then
        dest="$QUERIES_DIR/$lang"
        mkdir -p "$dest"
        cp "$queries_path"/*.scm "$dest/" 2>/dev/null && FOUND=1 || true
      fi
    done

    if [ "$FOUND" -eq 1 ]; then
      file_count=$(ls "$QUERIES_DIR/$lang"/*.scm 2>/dev/null | wc -l | tr -d ' ')
      echo "OK ($file_count files)"
      COLLECTED=$((COLLECTED + 1))
    else
      echo "no queries"
    fi
  else
    echo "FAILED to fetch"
  fi
  rm -rf "$TMPDIR"
done

echo ""
echo "=== Collected queries for $COLLECTED grammars ==="
echo "  Output: $QUERIES_DIR"
