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
    let _ = tree_sitter_agda::LANGUAGE;
    let _ = tree_sitter_arduino::LANGUAGE;
    let _ = tree_sitter_bash::LANGUAGE;
    let _ = tree_sitter_bicep::LANGUAGE;
    let _ = tree_sitter_c::LANGUAGE;
    let _ = tree_sitter_c_sharp::LANGUAGE;
    let _ = tree_sitter_cairo::language();
    let _ = tree_sitter_cmake::LANGUAGE;
    let _ = tree_sitter_commonlisp::LANGUAGE_COMMONLISP;
    let _ = tree_sitter_cpon::language();
    let _ = tree_sitter_cpp::LANGUAGE;
    let _ = tree_sitter_css::LANGUAGE;
    let _ = tree_sitter_cuda::LANGUAGE;
    let _ = tree_sitter_diff::LANGUAGE;
    let _ = tree_sitter_dockerfile::language();
    let _ = tree_sitter_elixir::LANGUAGE;
    let _ = tree_sitter_embedded_template::LANGUAGE;
    let _ = tree_sitter_erlang::LANGUAGE;
    let _ = tree_sitter_func::language();
    let _ = tree_sitter_gitattributes::language();
    let _ = tree_sitter_glsl::LANGUAGE_GLSL;
    let _ = tree_sitter_go::LANGUAGE;
    let _ = tree_sitter_go_sum::language();
    let _ = tree_sitter_hare::language();
    let _ = tree_sitter_haskell::LANGUAGE;
    let _ = tree_sitter_hcl::LANGUAGE;
    let _ = tree_sitter_hlsl::LANGUAGE_HLSL;
    let _ = tree_sitter_html::LANGUAGE;
    let _ = tree_sitter_java::LANGUAGE;
    let _ = tree_sitter_javascript::LANGUAGE;
    let _ = tree_sitter_jsdoc::LANGUAGE;
    let _ = tree_sitter_json::LANGUAGE;
    let _ = tree_sitter_julia::LANGUAGE;
    let _ = tree_sitter_kconfig::LANGUAGE;
    let _ = tree_sitter_kdl::language();
    let _ = tree_sitter_kotlin::language();
    let _ = tree_sitter_lua::LANGUAGE;
    let _ = tree_sitter_luadoc::language();
    let _ = tree_sitter_luap::language();
    let _ = tree_sitter_luau::LANGUAGE;
    let _ = tree_sitter_make::LANGUAGE;
    let _ = tree_sitter_markdown::language();
    let _ = tree_sitter_objc::LANGUAGE;
    let _ = tree_sitter_ocaml::LANGUAGE_OCAML;
    let _ = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    let _ = tree_sitter_ocaml::LANGUAGE_OCAML_TYPE;
    let _ = tree_sitter_odin::LANGUAGE;
    let _ = tree_sitter_php::LANGUAGE_PHP;
    let _ = tree_sitter_php::LANGUAGE_PHP_ONLY;
    let _ = tree_sitter_po::language();
    let _ = tree_sitter_pony::language();
    let _ = tree_sitter_printf::LANGUAGE;
    let _ = tree_sitter_properties::LANGUAGE;
    let _ = tree_sitter_puppet::LANGUAGE;
    let _ = tree_sitter_python::LANGUAGE;
    let _ = tree_sitter_ql::LANGUAGE;
    let _ = tree_sitter_qmldir::language();
    let _ = tree_sitter_query::language();
    let _ = tree_sitter_r::LANGUAGE;
    let _ = tree_sitter_regex::LANGUAGE;
    let _ = tree_sitter_requirements::LANGUAGE;
    let _ = tree_sitter_ron::language();
    let _ = tree_sitter_ruby::LANGUAGE;
    let _ = tree_sitter_rust::LANGUAGE;
    let _ = tree_sitter_scala::LANGUAGE;
    let _ = tree_sitter_scss::language();
    let _ = tree_sitter_sql::language();
    let _ = tree_sitter_squirrel::language();
    let _ = tree_sitter_starlark::LANGUAGE;
    let _ = tree_sitter_svelte::language();
    let _ = tree_sitter_swift::LANGUAGE;
    let _ = tree_sitter_test::LANGUAGE;
    let _ = tree_sitter_toml::language();
    let _ = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    let _ = tree_sitter_typescript::LANGUAGE_TSX;
    let _ = tree_sitter_ungrammar::language();
    let _ = tree_sitter_verilog::LANGUAGE;
    let _ = tree_sitter_vim::language();
    let _ = tree_sitter_vue::language();
    let _ = tree_sitter_wgsl_bevy::LANGUAGE;
    let _ = tree_sitter_xml::LANGUAGE_DTD;
    let _ = tree_sitter_xml::LANGUAGE_XML;
    let _ = tree_sitter_yaml::LANGUAGE;
    let _ = tree_sitter_yuck::language();
    let _ = tree_sitter_zig::LANGUAGE;
}

// ── Introspection ───────────────────────────────────────────────────────

const GRAMMAR_COUNT: usize = 85;

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
        SyncPtr(c"agda".as_ptr().cast()),
        SyncPtr(c"arduino".as_ptr().cast()),
        SyncPtr(c"bash".as_ptr().cast()),
        SyncPtr(c"bicep".as_ptr().cast()),
        SyncPtr(c"c".as_ptr().cast()),
        SyncPtr(c"c_sharp".as_ptr().cast()),
        SyncPtr(c"cairo".as_ptr().cast()),
        SyncPtr(c"cmake".as_ptr().cast()),
        SyncPtr(c"commonlisp".as_ptr().cast()),
        SyncPtr(c"cpon".as_ptr().cast()),
        SyncPtr(c"cpp".as_ptr().cast()),
        SyncPtr(c"css".as_ptr().cast()),
        SyncPtr(c"cuda".as_ptr().cast()),
        SyncPtr(c"diff".as_ptr().cast()),
        SyncPtr(c"dockerfile".as_ptr().cast()),
        SyncPtr(c"dtd".as_ptr().cast()),
        SyncPtr(c"elixir".as_ptr().cast()),
        SyncPtr(c"embedded_template".as_ptr().cast()),
        SyncPtr(c"erlang".as_ptr().cast()),
        SyncPtr(c"func".as_ptr().cast()),
        SyncPtr(c"gitattributes".as_ptr().cast()),
        SyncPtr(c"glsl".as_ptr().cast()),
        SyncPtr(c"go".as_ptr().cast()),
        SyncPtr(c"gosum".as_ptr().cast()),
        SyncPtr(c"hare".as_ptr().cast()),
        SyncPtr(c"haskell".as_ptr().cast()),
        SyncPtr(c"hcl".as_ptr().cast()),
        SyncPtr(c"hlsl".as_ptr().cast()),
        SyncPtr(c"html".as_ptr().cast()),
        SyncPtr(c"java".as_ptr().cast()),
        SyncPtr(c"javascript".as_ptr().cast()),
        SyncPtr(c"jsdoc".as_ptr().cast()),
        SyncPtr(c"json".as_ptr().cast()),
        SyncPtr(c"julia".as_ptr().cast()),
        SyncPtr(c"kconfig".as_ptr().cast()),
        SyncPtr(c"kdl".as_ptr().cast()),
        SyncPtr(c"kotlin".as_ptr().cast()),
        SyncPtr(c"lua".as_ptr().cast()),
        SyncPtr(c"luadoc".as_ptr().cast()),
        SyncPtr(c"luap".as_ptr().cast()),
        SyncPtr(c"luau".as_ptr().cast()),
        SyncPtr(c"make".as_ptr().cast()),
        SyncPtr(c"markdown".as_ptr().cast()),
        SyncPtr(c"objc".as_ptr().cast()),
        SyncPtr(c"ocaml".as_ptr().cast()),
        SyncPtr(c"ocaml_interface".as_ptr().cast()),
        SyncPtr(c"ocaml_type".as_ptr().cast()),
        SyncPtr(c"odin".as_ptr().cast()),
        SyncPtr(c"php".as_ptr().cast()),
        SyncPtr(c"php_only".as_ptr().cast()),
        SyncPtr(c"po".as_ptr().cast()),
        SyncPtr(c"pony".as_ptr().cast()),
        SyncPtr(c"printf".as_ptr().cast()),
        SyncPtr(c"properties".as_ptr().cast()),
        SyncPtr(c"puppet".as_ptr().cast()),
        SyncPtr(c"python".as_ptr().cast()),
        SyncPtr(c"ql".as_ptr().cast()),
        SyncPtr(c"qmldir".as_ptr().cast()),
        SyncPtr(c"query".as_ptr().cast()),
        SyncPtr(c"r".as_ptr().cast()),
        SyncPtr(c"regex".as_ptr().cast()),
        SyncPtr(c"requirements".as_ptr().cast()),
        SyncPtr(c"ron".as_ptr().cast()),
        SyncPtr(c"ruby".as_ptr().cast()),
        SyncPtr(c"rust".as_ptr().cast()),
        SyncPtr(c"scala".as_ptr().cast()),
        SyncPtr(c"scss".as_ptr().cast()),
        SyncPtr(c"sql".as_ptr().cast()),
        SyncPtr(c"squirrel".as_ptr().cast()),
        SyncPtr(c"starlark".as_ptr().cast()),
        SyncPtr(c"svelte".as_ptr().cast()),
        SyncPtr(c"swift".as_ptr().cast()),
        SyncPtr(c"test".as_ptr().cast()),
        SyncPtr(c"toml".as_ptr().cast()),
        SyncPtr(c"tsx".as_ptr().cast()),
        SyncPtr(c"typescript".as_ptr().cast()),
        SyncPtr(c"ungrammar".as_ptr().cast()),
        SyncPtr(c"verilog".as_ptr().cast()),
        SyncPtr(c"vim".as_ptr().cast()),
        SyncPtr(c"vue".as_ptr().cast()),
        SyncPtr(c"wgsl_bevy".as_ptr().cast()),
        SyncPtr(c"xml".as_ptr().cast()),
        SyncPtr(c"yaml".as_ptr().cast()),
        SyncPtr(c"yuck".as_ptr().cast()),
        SyncPtr(c"zig".as_ptr().cast()),
        // Null terminator
        SyncPtr(std::ptr::null()),
    ];
    NAMES.as_ptr() as *const *const i8
}
