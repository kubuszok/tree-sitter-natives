// tree-sitter-all: Monolithic library containing tree-sitter core + all grammar parsers.
//
// Exports (C ABI):
//   - ts_*()                  — core tree-sitter API (ts_parser_new, ts_tree_root_node, etc.)
//   - tree_sitter_<lang>()    — per-grammar language constructors (from parser.c)
//   - ts_natives_*()          — introspection (grammar count, names)
//
// Each grammar crate compiles parser.c which defines tree_sitter_<lang>() in C.
// The build.rs ensures these C symbols are exported from the cdylib and not
// dead-stripped by the linker.

// ── Force each grammar crate to be linked ──────────────────────────────
// Referencing LANGUAGE (or language()) ensures the crate's build.rs runs and
// its static archive is included in the link. The actual C functions from
// parser.c are exported via linker flags in build.rs.

fn _force_link() {
    // Tier 1
    let _ = tree_sitter_bash::LANGUAGE;
    let _ = tree_sitter_c::LANGUAGE;
    let _ = tree_sitter_cpp::LANGUAGE;
    let _ = tree_sitter_c_sharp::LANGUAGE;
    let _ = tree_sitter_css::LANGUAGE;
    let _ = tree_sitter_go::LANGUAGE;
    let _ = tree_sitter_html::LANGUAGE;
    let _ = tree_sitter_java::LANGUAGE;
    let _ = tree_sitter_javascript::LANGUAGE;
    let _ = tree_sitter_json::LANGUAGE;
    let _ = tree_sitter_markdown::language();
    let _ = tree_sitter_python::LANGUAGE;
    let _ = tree_sitter_regex::LANGUAGE;
    let _ = tree_sitter_ruby::LANGUAGE;
    let _ = tree_sitter_rust::LANGUAGE;
    let _ = tree_sitter_scala::LANGUAGE;
    let _ = tree_sitter_sql::language();
    let _ = tree_sitter_toml::language();
    let _ = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    let _ = tree_sitter_typescript::LANGUAGE_TSX;
    let _ = tree_sitter_yaml::LANGUAGE;
    // Tier 2
    let _ = tree_sitter_cmake::LANGUAGE;
    let _ = tree_sitter_dockerfile::language();
    let _ = tree_sitter_elixir::LANGUAGE;
    let _ = tree_sitter_erlang::LANGUAGE;
    let _ = tree_sitter_haskell::LANGUAGE;
    let _ = tree_sitter_julia::LANGUAGE;
    let _ = tree_sitter_kotlin::language();
    let _ = tree_sitter_lua::LANGUAGE;
    let _ = tree_sitter_make::LANGUAGE;
    let _ = tree_sitter_ocaml::LANGUAGE_OCAML;
    let _ = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    let _ = tree_sitter_php::LANGUAGE_PHP;
    let _ = tree_sitter_php::LANGUAGE_PHP_ONLY;
    let _ = tree_sitter_r::LANGUAGE;
    let _ = tree_sitter_swift::LANGUAGE;
    let _ = tree_sitter_vim::language();
    let _ = tree_sitter_xml::LANGUAGE_XML;
    let _ = tree_sitter_xml::LANGUAGE_DTD;
    let _ = tree_sitter_zig::LANGUAGE;
}

// ── Introspection ───────────────────────────────────────────────────────

const GRAMMAR_COUNT: usize = 40;

/// Wrapper to make *const i8 usable in static context.
#[repr(transparent)]
struct SyncPtr(*const i8);
unsafe impl Sync for SyncPtr {}

/// Returns the number of bundled grammars.
#[no_mangle]
pub extern "C" fn ts_natives_grammar_count() -> i32 {
    GRAMMAR_COUNT as i32
}

/// Returns a null-terminated array of grammar name C strings (static lifetime).
/// Names match the C function suffix: tree_sitter_<name>().
#[no_mangle]
pub extern "C" fn ts_natives_grammar_names() -> *const *const i8 {
    static NAMES: [SyncPtr; GRAMMAR_COUNT + 1] = [
        // Tier 1
        SyncPtr(c"bash".as_ptr().cast()),
        SyncPtr(c"c".as_ptr().cast()),
        SyncPtr(c"cpp".as_ptr().cast()),
        SyncPtr(c"c_sharp".as_ptr().cast()),
        SyncPtr(c"css".as_ptr().cast()),
        SyncPtr(c"go".as_ptr().cast()),
        SyncPtr(c"html".as_ptr().cast()),
        SyncPtr(c"java".as_ptr().cast()),
        SyncPtr(c"javascript".as_ptr().cast()),
        SyncPtr(c"json".as_ptr().cast()),
        SyncPtr(c"markdown".as_ptr().cast()),
        SyncPtr(c"python".as_ptr().cast()),
        SyncPtr(c"regex".as_ptr().cast()),
        SyncPtr(c"ruby".as_ptr().cast()),
        SyncPtr(c"rust".as_ptr().cast()),
        SyncPtr(c"scala".as_ptr().cast()),
        SyncPtr(c"sql".as_ptr().cast()),
        SyncPtr(c"toml".as_ptr().cast()),
        SyncPtr(c"typescript".as_ptr().cast()),
        SyncPtr(c"tsx".as_ptr().cast()),
        SyncPtr(c"yaml".as_ptr().cast()),
        // Tier 2
        SyncPtr(c"cmake".as_ptr().cast()),
        SyncPtr(c"dockerfile".as_ptr().cast()),
        SyncPtr(c"dtd".as_ptr().cast()),
        SyncPtr(c"elixir".as_ptr().cast()),
        SyncPtr(c"erlang".as_ptr().cast()),
        SyncPtr(c"haskell".as_ptr().cast()),
        SyncPtr(c"julia".as_ptr().cast()),
        SyncPtr(c"kotlin".as_ptr().cast()),
        SyncPtr(c"lua".as_ptr().cast()),
        SyncPtr(c"make".as_ptr().cast()),
        SyncPtr(c"ocaml".as_ptr().cast()),
        SyncPtr(c"ocaml_interface".as_ptr().cast()),
        SyncPtr(c"php".as_ptr().cast()),
        SyncPtr(c"php_only".as_ptr().cast()),
        SyncPtr(c"r".as_ptr().cast()),
        SyncPtr(c"swift".as_ptr().cast()),
        SyncPtr(c"vim".as_ptr().cast()),
        SyncPtr(c"xml".as_ptr().cast()),
        SyncPtr(c"zig".as_ptr().cast()),
        // Null terminator
        SyncPtr(std::ptr::null()),
    ];
    NAMES.as_ptr() as *const *const i8
}
