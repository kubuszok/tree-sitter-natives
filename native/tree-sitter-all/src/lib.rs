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
// For new-API grammars: call into_raw() then invoke the function pointer.
// This forces the linker to retain the C parser code on all platforms.
// For old-API grammars: language() directly calls the C function.

#[inline(never)]
fn _force_link() {
    unsafe {
    let _ = (tree_sitter_agda::LANGUAGE.into_raw())();
    let _ = (tree_sitter_arduino::LANGUAGE.into_raw())();
    let _ = (tree_sitter_bash::LANGUAGE.into_raw())();
    let _ = (tree_sitter_bicep::LANGUAGE.into_raw())();
    let _ = (tree_sitter_c::LANGUAGE.into_raw())();
    let _ = (tree_sitter_c_sharp::LANGUAGE.into_raw())();
    let _ = tree_sitter_cairo::language();
    let _ = (tree_sitter_cmake::LANGUAGE.into_raw())();
    let _ = (tree_sitter_commonlisp::LANGUAGE_COMMONLISP.into_raw())();
    let _ = tree_sitter_cpon::language();
    let _ = (tree_sitter_cpp::LANGUAGE.into_raw())();
    let _ = (tree_sitter_css::LANGUAGE.into_raw())();
    let _ = (tree_sitter_cuda::LANGUAGE.into_raw())();
    let _ = (tree_sitter_diff::LANGUAGE.into_raw())();
    let _ = tree_sitter_dockerfile::language();
    let _ = (tree_sitter_elixir::LANGUAGE.into_raw())();
    let _ = (tree_sitter_embedded_template::LANGUAGE.into_raw())();
    let _ = (tree_sitter_erlang::LANGUAGE.into_raw())();
    let _ = tree_sitter_func::language();
    let _ = tree_sitter_gitattributes::language();
    let _ = (tree_sitter_glsl::LANGUAGE_GLSL.into_raw())();
    let _ = (tree_sitter_go::LANGUAGE.into_raw())();
    let _ = tree_sitter_go_sum::language();
    let _ = tree_sitter_hare::language();
    let _ = (tree_sitter_haskell::LANGUAGE.into_raw())();
    let _ = (tree_sitter_hcl::LANGUAGE.into_raw())();
    let _ = (tree_sitter_hlsl::LANGUAGE_HLSL.into_raw())();
    let _ = (tree_sitter_html::LANGUAGE.into_raw())();
    let _ = (tree_sitter_java::LANGUAGE.into_raw())();
    let _ = (tree_sitter_javascript::LANGUAGE.into_raw())();
    let _ = (tree_sitter_jsdoc::LANGUAGE.into_raw())();
    let _ = (tree_sitter_json::LANGUAGE.into_raw())();
    let _ = (tree_sitter_julia::LANGUAGE.into_raw())();
    let _ = (tree_sitter_kconfig::LANGUAGE.into_raw())();
    let _ = tree_sitter_kdl::language();
    let _ = tree_sitter_kotlin::language();
    let _ = (tree_sitter_lua::LANGUAGE.into_raw())();
    let _ = tree_sitter_luadoc::language();
    let _ = tree_sitter_luap::language();
    let _ = (tree_sitter_luau::LANGUAGE.into_raw())();
    let _ = (tree_sitter_make::LANGUAGE.into_raw())();
    let _ = tree_sitter_markdown::language();
    let _ = (tree_sitter_objc::LANGUAGE.into_raw())();
    let _ = (tree_sitter_ocaml::LANGUAGE_OCAML.into_raw())();
    let _ = (tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into_raw())();
    let _ = (tree_sitter_ocaml::LANGUAGE_OCAML_TYPE.into_raw())();
    let _ = (tree_sitter_odin::LANGUAGE.into_raw())();
    let _ = (tree_sitter_php::LANGUAGE_PHP.into_raw())();
    let _ = (tree_sitter_php::LANGUAGE_PHP_ONLY.into_raw())();
    let _ = tree_sitter_po::language();
    let _ = tree_sitter_pony::language();
    let _ = (tree_sitter_printf::LANGUAGE.into_raw())();
    let _ = (tree_sitter_properties::LANGUAGE.into_raw())();
    let _ = (tree_sitter_puppet::LANGUAGE.into_raw())();
    let _ = (tree_sitter_python::LANGUAGE.into_raw())();
    let _ = (tree_sitter_ql::LANGUAGE.into_raw())();
    let _ = tree_sitter_qmldir::language();
    let _ = tree_sitter_query::language();
    let _ = (tree_sitter_r::LANGUAGE.into_raw())();
    let _ = (tree_sitter_regex::LANGUAGE.into_raw())();
    let _ = (tree_sitter_requirements::LANGUAGE.into_raw())();
    let _ = tree_sitter_ron::language();
    let _ = (tree_sitter_ruby::LANGUAGE.into_raw())();
    let _ = (tree_sitter_rust::LANGUAGE.into_raw())();
    let _ = (tree_sitter_scala::LANGUAGE.into_raw())();
    let _ = tree_sitter_scss::language();
    let _ = tree_sitter_sql::language();
    let _ = tree_sitter_squirrel::language();
    let _ = (tree_sitter_starlark::LANGUAGE.into_raw())();
    let _ = tree_sitter_svelte::language();
    let _ = (tree_sitter_swift::LANGUAGE.into_raw())();
    let _ = (tree_sitter_test::LANGUAGE.into_raw())();
    let _ = tree_sitter_toml::language();
    let _ = (tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into_raw())();
    let _ = (tree_sitter_typescript::LANGUAGE_TSX.into_raw())();
    let _ = tree_sitter_ungrammar::language();
    let _ = (tree_sitter_verilog::LANGUAGE.into_raw())();
    let _ = tree_sitter_vim::language();
    let _ = tree_sitter_vue::language();
    let _ = (tree_sitter_wgsl_bevy::LANGUAGE.into_raw())();
    let _ = (tree_sitter_xml::LANGUAGE_DTD.into_raw())();
    let _ = (tree_sitter_xml::LANGUAGE_XML.into_raw())();
    let _ = (tree_sitter_yaml::LANGUAGE.into_raw())();
    let _ = tree_sitter_yuck::language();
    let _ = (tree_sitter_zig::LANGUAGE.into_raw())();
    }
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
    _force_link();
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
