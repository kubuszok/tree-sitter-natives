#!/bin/bash
# Collect highlight/injection/locals query files (.scm) from grammar sources.
#
# Strategy: try npm first, fall back to GitHub raw download for grammars
# whose npm packages don't bundle queries or are name-squatted.
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

# Format: "lang|npm-package|github-owner/repo|git-ref|queries-path-in-repo"
# npm-package can be "-" to skip npm and go straight to GitHub.
# queries-path-in-repo is relative to the repo root.
GRAMMARS=(
  # Tier 1
  "bash|tree-sitter-bash|-|-|-"
  "c|tree-sitter-c|-|-|-"
  "cpp|tree-sitter-cpp|-|-|-"
  "c-sharp|tree-sitter-c-sharp|-|-|-"
  "css|tree-sitter-css|-|-|-"
  "go|tree-sitter-go|-|-|-"
  "html|tree-sitter-html|-|-|-"
  "java|tree-sitter-java|-|-|-"
  "javascript|tree-sitter-javascript|-|-|-"
  "json|tree-sitter-json|-|-|-"
  "markdown|tree-sitter-markdown|tree-sitter-grammars/tree-sitter-markdown|split_parser|tree-sitter-markdown/queries"
  "python|tree-sitter-python|-|-|-"
  "regex|tree-sitter-regex|-|-|-"
  "ruby|tree-sitter-ruby|-|-|-"
  "rust|tree-sitter-rust|-|-|-"
  "scala|tree-sitter-scala|-|-|-"
  "sql|-|m-novikov/tree-sitter-sql|master|queries"
  "toml|tree-sitter-toml|-|-|-"
  "typescript|tree-sitter-typescript|-|-|-"
  "yaml|-|tree-sitter-grammars/tree-sitter-yaml|master|queries"
  # Tier 2
  "cmake|-|uyha/tree-sitter-cmake|master|queries"
  "dockerfile|-|camdencheek/tree-sitter-dockerfile|main|queries"
  "elixir|tree-sitter-elixir|-|-|-"
  "erlang|-|WhatsApp/tree-sitter-erlang|main|queries"
  "haskell|tree-sitter-haskell|-|-|-"
  "julia|tree-sitter-julia|-|-|-"
  "kotlin|tree-sitter-kotlin|-|-|-"
  "lua|-|tree-sitter-grammars/tree-sitter-lua|master|queries"
  "make|tree-sitter-make|-|-|-"
  "ocaml|tree-sitter-ocaml|-|-|-"
  "php|tree-sitter-php|-|-|-"
  "r|-|r-lib/tree-sitter-r|main|queries"
  "swift|tree-sitter-swift|-|-|-"
  "vim|-|tree-sitter-grammars/tree-sitter-vim|master|queries/vim"
  "xml|-|tree-sitter-grammars/tree-sitter-xml|master|queries/xml"
  "zig|-|tree-sitter-grammars/tree-sitter-zig|master|queries"
  # Tier 3: tree-sitter-grammars org
  "arduino|tree-sitter-arduino|-|-|-"
  "bicep|tree-sitter-bicep|-|-|-"
  "cairo|-|tree-sitter-grammars/tree-sitter-cairo|master|queries"
  "commonlisp|tree-sitter-commonlisp|-|-|-"
  "cpon|-|tree-sitter-grammars/tree-sitter-cpon|master|queries"
  "cuda|tree-sitter-cuda|-|-|-"
  "diff|tree-sitter-diff|-|-|-"
  "func|-|tree-sitter-grammars/tree-sitter-func|master|queries"
  "gitattributes|-|tree-sitter-grammars/tree-sitter-gitattributes|master|queries"
  "glsl|tree-sitter-glsl|-|-|-"
  "gosum|-|tree-sitter-grammars/tree-sitter-go-sum|master|queries"
  "hare|-|tree-sitter-grammars/tree-sitter-hare|master|queries"
  "hcl|tree-sitter-hcl|-|-|-"
  "hlsl|tree-sitter-hlsl|-|-|-"
  "kconfig|-|tree-sitter-grammars/tree-sitter-kconfig|master|queries"
  "kdl|tree-sitter-kdl|-|-|-"
  "luadoc|-|tree-sitter-grammars/tree-sitter-luadoc|master|queries"
  "luap|-|tree-sitter-grammars/tree-sitter-luap|master|queries"
  "luau|-|tree-sitter-grammars/tree-sitter-luau|master|queries"
  "objc|tree-sitter-objc|-|-|-"
  "odin|tree-sitter-odin|-|-|-"
  "po|-|tree-sitter-grammars/tree-sitter-po|master|queries"
  "pony|-|tree-sitter-grammars/tree-sitter-pony|master|queries"
  "printf|-|tree-sitter-grammars/tree-sitter-printf|master|queries"
  "properties|tree-sitter-properties|-|-|-"
  "puppet|tree-sitter-puppet|-|-|-"
  "ql|tree-sitter-ql|-|-|-"
  "qmldir|-|tree-sitter-grammars/tree-sitter-qmldir|master|queries"
  "query|-|tree-sitter-grammars/tree-sitter-query|master|queries"
  "requirements|tree-sitter-requirements|-|-|-"
  "ron|tree-sitter-ron|-|-|-"
  "scss|tree-sitter-scss|-|-|-"
  "squirrel|-|tree-sitter-grammars/tree-sitter-squirrel|master|queries"
  "starlark|tree-sitter-starlark|-|-|-"
  "svelte|tree-sitter-svelte|-|-|-"
  "test|-|tree-sitter-grammars/tree-sitter-test|master|queries"
  "ungrammar|-|tree-sitter-grammars/tree-sitter-ungrammar|master|queries"
  "vue|tree-sitter-vue|-|-|-"
  "wgsl_bevy|-|tree-sitter-grammars/tree-sitter-wgsl-bevy|master|queries"
  "yuck|-|tree-sitter-grammars/tree-sitter-yuck|master|queries"
  # Tier 4: tree-sitter core org
  "agda|tree-sitter-agda|-|-|-"
  "embedded_template|tree-sitter-embedded-template|-|-|-"
  "jsdoc|tree-sitter-jsdoc|-|-|-"
  "verilog|tree-sitter-verilog|-|-|-"
)

rm -rf "$QUERIES_DIR"
mkdir -p "$QUERIES_DIR"

echo "=== Collecting query files ==="
COLLECTED=0
FAILED=0

collect_from_npm() {
  local lang="$1" pkg="$2"
  local tmpdir
  tmpdir=$(mktemp -d)
  local found=1
  if (cd "$tmpdir" && npm pack "$pkg" 2>/dev/null && tar xzf ${pkg}-*.tgz 2>/dev/null); then
    for qpath in "$tmpdir/package/queries" "$tmpdir/package/src/queries"; do
      if [ -d "$qpath" ] && ls "$qpath"/*.scm >/dev/null 2>&1; then
        mkdir -p "$QUERIES_DIR/$lang"
        cp "$qpath"/*.scm "$QUERIES_DIR/$lang/"
        found=0
        break
      fi
    done
  fi
  rm -rf "$tmpdir"
  return $found
}

collect_from_github() {
  local lang="$1" repo="$2" ref="$3" qpath="$4"
  local base_url="https://raw.githubusercontent.com/${repo}/${ref}/${qpath}"
  mkdir -p "$QUERIES_DIR/$lang"
  local got=0
  for scm in highlights.scm locals.scm injections.scm tags.scm; do
    if curl -fsSL "${base_url}/${scm}" -o "$QUERIES_DIR/$lang/$scm" 2>/dev/null; then
      got=$((got + 1))
    else
      rm -f "$QUERIES_DIR/$lang/$scm"
    fi
  done
  [ "$got" -gt 0 ]
}

for entry in "${GRAMMARS[@]}"; do
  IFS='|' read -r lang npm_pkg gh_repo gh_ref gh_qpath <<< "$entry"
  echo -n "  $lang... "

  # Try npm first (unless skipped)
  if [ "$npm_pkg" != "-" ] && collect_from_npm "$lang" "$npm_pkg"; then
    count=$(ls "$QUERIES_DIR/$lang"/*.scm 2>/dev/null | wc -l | tr -d ' ')
    echo "OK via npm ($count files)"
    COLLECTED=$((COLLECTED + 1))
    continue
  fi

  # Fall back to GitHub
  if [ "$gh_repo" != "-" ] && collect_from_github "$lang" "$gh_repo" "$gh_ref" "$gh_qpath"; then
    count=$(ls "$QUERIES_DIR/$lang"/*.scm 2>/dev/null | wc -l | tr -d ' ')
    echo "OK via github ($count files)"
    COLLECTED=$((COLLECTED + 1))
    continue
  fi

  echo "FAILED"
  FAILED=$((FAILED + 1))
done

# ── Apply version-compatible overrides ───────────────────────────────
# Some grammars need hand-tuned queries because the npm/GitHub queries
# reference node types from a different grammar version than our Cargo
# pins, or because variant grammars (dtd, ocaml_interface) need their
# own query files separate from the parent grammar.
OVERRIDES_DIR="$SCRIPT_DIR/../query-overrides"
if [ -d "$OVERRIDES_DIR" ]; then
  echo ""
  echo "--- Applying query overrides ---"
  for override_lang in "$OVERRIDES_DIR"/*/; do
    lang=$(basename "$override_lang")
    mkdir -p "$QUERIES_DIR/$lang"
    cp "$override_lang"/*.scm "$QUERIES_DIR/$lang/"
    count=$(ls "$QUERIES_DIR/$lang"/*.scm 2>/dev/null | wc -l | tr -d ' ')
    echo "  $lang: $count files (override)"
    # Count as collected if it wasn't already
    if [ ! -d "$QUERIES_DIR/$lang" ] 2>/dev/null; then
      COLLECTED=$((COLLECTED + 1))
    fi
  done
fi

echo ""
echo "=== Collected queries for $COLLECTED / $((COLLECTED + FAILED)) grammars ==="
echo "  Output: $QUERIES_DIR"
if [ "$FAILED" -gt 0 ]; then
  echo "  ⚠ $FAILED grammars have no highlight queries (they won't support syntax highlighting)"
fi
