// tree-sitter-all: Monolithic library containing tree-sitter core + all grammar parsers.
//
// Exports (C ABI):
//   - ts_*()                  — core tree-sitter API (ts_parser_new, ts_tree_root_node, etc.)
//   - tree_sitter_<lang>()    — per-grammar language constructors
//   - ts_natives_*()          — introspection (grammar count, names)
//
// Each grammar crate compiles parser.c which defines tree_sitter_<lang>() in C.
// However, cdylib only exports Rust #[no_mangle] symbols, so we create thin
// wrappers that call through to the grammar crate's Rust API.

// ── Export macros ───────────────────────────────────────────────────────

/// Export a grammar that uses the new LanguageFn API (LANGUAGE constant).
/// LanguageFn::into_raw() returns `unsafe extern "C" fn() -> *const ()`.
macro_rules! export_grammar {
    // Standard: crate::LANGUAGE
    ($crate_name:ident, $export_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $export_name() -> *const () {
            let f = $crate_name::LANGUAGE.into_raw();
            unsafe { f() }
        }
    };
    // Named variant: crate::LANGUAGE_VARIANT (for multi-grammar crates)
    ($crate_name:ident { $lang_const:ident }, $export_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $export_name() -> *const () {
            let f = $crate_name::$lang_const.into_raw();
            unsafe { f() }
        }
    };
}

/// Export a grammar that uses the old language() function API.
/// The old Language type is #[repr(transparent)] wrapping *const TSLanguage,
/// so transmuting to *const () is safe.
macro_rules! export_grammar_old {
    ($crate_path:path, $export_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $export_name() -> *const () {
            use $crate_path as lang;
            let l = lang::language();
            unsafe { std::mem::transmute(l) }
        }
    };
}

// ── Tier 1: Most popular languages ──────────────────────────────────────

export_grammar!(tree_sitter_bash, tree_sitter_bash);
export_grammar!(tree_sitter_c, tree_sitter_c);
export_grammar!(tree_sitter_cpp, tree_sitter_cpp);
export_grammar!(tree_sitter_c_sharp, tree_sitter_c_sharp);
export_grammar!(tree_sitter_css, tree_sitter_css);
export_grammar!(tree_sitter_go, tree_sitter_go);
export_grammar!(tree_sitter_html, tree_sitter_html);
export_grammar!(tree_sitter_java, tree_sitter_java);
export_grammar!(tree_sitter_javascript, tree_sitter_javascript);
export_grammar!(tree_sitter_json, tree_sitter_json);
export_grammar_old!(tree_sitter_markdown, tree_sitter_markdown);
export_grammar!(tree_sitter_python, tree_sitter_python);
export_grammar!(tree_sitter_regex, tree_sitter_regex);
export_grammar!(tree_sitter_ruby, tree_sitter_ruby);
export_grammar!(tree_sitter_rust, tree_sitter_rust);
export_grammar!(tree_sitter_scala, tree_sitter_scala);
export_grammar_old!(tree_sitter_sql, tree_sitter_sql);
export_grammar_old!(tree_sitter_toml, tree_sitter_toml);
export_grammar!(tree_sitter_yaml, tree_sitter_yaml);

// TypeScript: two variants
export_grammar!(tree_sitter_typescript { LANGUAGE_TYPESCRIPT }, tree_sitter_typescript);
export_grammar!(tree_sitter_typescript { LANGUAGE_TSX }, tree_sitter_tsx);

// ── Tier 2: Broadly used languages ──────────────────────────────────────

export_grammar!(tree_sitter_cmake, tree_sitter_cmake);
export_grammar_old!(tree_sitter_dockerfile, tree_sitter_dockerfile);
export_grammar!(tree_sitter_elixir, tree_sitter_elixir);
export_grammar!(tree_sitter_erlang, tree_sitter_erlang);
export_grammar!(tree_sitter_haskell, tree_sitter_haskell);
export_grammar!(tree_sitter_julia, tree_sitter_julia);
export_grammar_old!(tree_sitter_kotlin, tree_sitter_kotlin);
export_grammar!(tree_sitter_lua, tree_sitter_lua);
export_grammar!(tree_sitter_make, tree_sitter_make);
export_grammar!(tree_sitter_r, tree_sitter_r);
export_grammar!(tree_sitter_swift, tree_sitter_swift);
export_grammar_old!(tree_sitter_vim, tree_sitter_vim);
export_grammar!(tree_sitter_zig, tree_sitter_zig);

// OCaml: two variants
export_grammar!(tree_sitter_ocaml { LANGUAGE_OCAML }, tree_sitter_ocaml);
export_grammar!(tree_sitter_ocaml { LANGUAGE_OCAML_INTERFACE }, tree_sitter_ocaml_interface);

// PHP: two variants
export_grammar!(tree_sitter_php { LANGUAGE_PHP }, tree_sitter_php);
export_grammar!(tree_sitter_php { LANGUAGE_PHP_ONLY }, tree_sitter_php_only);

// XML: two variants
export_grammar!(tree_sitter_xml { LANGUAGE_XML }, tree_sitter_xml);
export_grammar!(tree_sitter_xml { LANGUAGE_DTD }, tree_sitter_dtd);

// ── Introspection ───────────────────────────────────────────────────────

// Total count of exported tree_sitter_<lang>() functions.
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
