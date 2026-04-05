# tree-sitter-natives

Cross-compiled [tree-sitter](https://tree-sitter.github.io/tree-sitter/) native
libraries for all major desktop platforms + WASM, with all officially supported
language grammars bundled into a single monolithic library.

## Supported Platforms

| Platform | Static (.a/.lib) | Shared (.so/.dylib/.dll) |
|----------|:-:|:-:|
| Linux x86_64 | ✓ | ✓ |
| Linux aarch64 | ✓ | ✓ |
| macOS x86_64 | ✓ | ✓ |
| macOS aarch64 | ✓ | ✓ |
| Windows x86_64 | ✓ | ✓ |
| Windows aarch64 | ✓ | ✓ |
| WASM (web-tree-sitter) | — | ✓ |

## Included Grammars

### Tier 1 (Most Popular)
Bash, C, C++, C#, CSS, Go, HTML, Java, JavaScript, JSON, Markdown,
Python, Regex, Ruby, Rust, Scala, SQL, TOML, TypeScript, YAML

### Tier 2 (Broadly Used)
CMake, Dockerfile, Elixir, Erlang, Haskell, Julia, Kotlin, Lua,
Make, OCaml, PHP, R, Swift, Vim, XML, Zig

## Release Artifacts

Each tagged release publishes:

| Archive | Contents |
|---------|----------|
| `tree-sitter-<platform>.tar.gz` | `libtree_sitter_all.{a,so,dylib}` or `.lib/.dll` |
| `tree-sitter-wasm.tar.gz` | `tree-sitter.js`, `tree-sitter.wasm`, `grammars/*.wasm` |
| `tree-sitter-queries.tar.gz` | Per-language `highlights.scm`, `locals.scm`, etc. |

## Usage

These archives are consumed by a downstream Scala provider module that embeds
them into fat JARs via [sbt-multi-arch-release](https://github.com/kubuszok/sbt-multi-arch-release),
enabling tree-sitter usage from:

- **JVM** — via Project Panama FFM (shared libraries)
- **Scala Native** — via `@extern` linking (static libraries)
- **Scala.js** — via web-tree-sitter facades (WASM)

## Building Locally

### Prerequisites

- Rust (stable)
- zig + cargo-zigbuild (Linux cross-compilation)
- cargo-xwin (Windows cross-compilation)
- Node.js + npm (WASM builds)
- Emscripten SDK (WASM grammar compilation)
- tree-sitter CLI (`npm install -g tree-sitter-cli`)

### Desktop (native macOS only)

```bash
cargo build --release --manifest-path native/Cargo.toml
```

### All 6 Desktop Targets (from macOS)

```bash
scripts/cross-all.sh
```

### WASM

```bash
scripts/build-wasm.sh
scripts/collect-queries.sh
```

## Adding a Grammar

1. Add the crate dependency to `native/tree-sitter-all/Cargo.toml`
2. Add the `export_grammar!` line to `native/tree-sitter-all/src/lib.rs`
3. Add the grammar name to the `GRAMMARS` array in `scripts/build-wasm.sh`
4. Add the grammar name to the `GRAMMARS` array in `scripts/collect-queries.sh`
5. Update the grammar count in `ts_natives_grammar_count()`
6. Update this README

## License

Apache-2.0
