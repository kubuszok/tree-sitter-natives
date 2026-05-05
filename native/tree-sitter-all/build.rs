// build.rs — Ensures all tree-sitter C API symbols (ts_*) are exported from the cdylib.
//
// By default, Rust cdylib only exports #[no_mangle] Rust symbols. The tree-sitter
// core C library functions (ts_parser_new, ts_tree_root_node, etc.) are linked
// in but not exported. This build script adds platform-specific linker flags to
// export them.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Core tree-sitter C API functions that must be exported.
/// Extracted from tree-sitter/include/tree_sitter/api.h (v0.25).
const TS_API_FUNCTIONS: &[&str] = &[
    // Parser
    "ts_parser_new",
    "ts_parser_delete",
    "ts_parser_language",
    "ts_parser_set_language",
    "ts_parser_included_ranges",
    "ts_parser_set_included_ranges",
    "ts_parser_parse",
    "ts_parser_parse_string",
    "ts_parser_parse_string_encoding",
    "ts_parser_reset",
    "ts_parser_set_timeout_micros",
    "ts_parser_timeout_micros",
    "ts_parser_set_cancellation_flag",
    "ts_parser_cancellation_flag",
    "ts_parser_set_logger",
    "ts_parser_logger",
    "ts_parser_print_dot_graphs",
    // Tree
    "ts_tree_copy",
    "ts_tree_delete",
    "ts_tree_root_node",
    "ts_tree_root_node_with_offset",
    "ts_tree_language",
    "ts_tree_included_ranges",
    "ts_tree_edit",
    "ts_tree_get_changed_ranges",
    "ts_tree_print_dot_graph",
    // Node
    "ts_node_type",
    "ts_node_symbol",
    "ts_node_language",
    "ts_node_grammar_type",
    "ts_node_grammar_symbol",
    "ts_node_start_byte",
    "ts_node_start_point",
    "ts_node_end_byte",
    "ts_node_end_point",
    "ts_node_string",
    "ts_node_is_null",
    "ts_node_is_named",
    "ts_node_is_missing",
    "ts_node_is_extra",
    "ts_node_has_changes",
    "ts_node_has_error",
    "ts_node_is_error",
    "ts_node_parse_state",
    "ts_node_next_parse_state",
    "ts_node_parent",
    "ts_node_child_with_descendant",
    "ts_node_child",
    "ts_node_field_name_for_child",
    "ts_node_field_name_for_named_child",
    "ts_node_child_count",
    "ts_node_named_child",
    "ts_node_named_child_count",
    "ts_node_child_by_field_name",
    "ts_node_child_by_field_id",
    "ts_node_next_sibling",
    "ts_node_prev_sibling",
    "ts_node_next_named_sibling",
    "ts_node_prev_named_sibling",
    "ts_node_first_child_for_byte",
    "ts_node_first_named_child_for_byte",
    "ts_node_descendant_count",
    "ts_node_descendant_for_byte_range",
    "ts_node_descendant_for_point_range",
    "ts_node_named_descendant_for_byte_range",
    "ts_node_named_descendant_for_point_range",
    "ts_node_edit",
    "ts_node_eq",
    // Tree cursor
    "ts_tree_cursor_new",
    "ts_tree_cursor_delete",
    "ts_tree_cursor_reset",
    "ts_tree_cursor_reset_to",
    "ts_tree_cursor_current_node",
    "ts_tree_cursor_current_field_name",
    "ts_tree_cursor_current_field_id",
    "ts_tree_cursor_goto_parent",
    "ts_tree_cursor_goto_next_sibling",
    "ts_tree_cursor_goto_previous_sibling",
    "ts_tree_cursor_goto_first_child",
    "ts_tree_cursor_goto_last_child",
    "ts_tree_cursor_goto_descendant",
    "ts_tree_cursor_current_descendant_index",
    "ts_tree_cursor_current_depth",
    "ts_tree_cursor_goto_first_child_for_byte",
    "ts_tree_cursor_goto_first_child_for_point",
    "ts_tree_cursor_copy",
    // Query
    "ts_query_new",
    "ts_query_delete",
    "ts_query_pattern_count",
    "ts_query_capture_count",
    "ts_query_string_count",
    "ts_query_start_byte_for_pattern",
    "ts_query_end_byte_for_pattern",
    "ts_query_predicates_for_pattern",
    "ts_query_is_pattern_rooted",
    "ts_query_is_pattern_non_local",
    "ts_query_is_pattern_guaranteed_at_step",
    "ts_query_capture_name_for_id",
    "ts_query_capture_quantifier_for_id",
    "ts_query_string_value_for_id",
    "ts_query_disable_capture",
    "ts_query_disable_pattern",
    // Query cursor
    "ts_query_cursor_new",
    "ts_query_cursor_delete",
    "ts_query_cursor_exec",
    "ts_query_cursor_did_exceed_match_limit",
    "ts_query_cursor_match_limit",
    "ts_query_cursor_set_match_limit",
    "ts_query_cursor_set_byte_range",
    "ts_query_cursor_set_point_range",
    "ts_query_cursor_next_match",
    "ts_query_cursor_remove_match",
    "ts_query_cursor_next_capture",
    "ts_query_cursor_set_max_start_depth",
    // Language
    "ts_language_copy",
    "ts_language_delete",
    "ts_language_symbol_count",
    "ts_language_state_count",
    "ts_language_symbol_name",
    "ts_language_symbol_for_name",
    "ts_language_field_count",
    "ts_language_field_name_for_id",
    "ts_language_field_id_for_name",
    "ts_language_symbol_type",
    "ts_language_version",
    "ts_language_next_state",
    "ts_language_name",
    "ts_language_metadata",
    // Lookahead iterator
    "ts_lookahead_iterator_new",
    "ts_lookahead_iterator_delete",
    "ts_lookahead_iterator_reset_state",
    "ts_lookahead_iterator_reset",
    "ts_lookahead_iterator_language",
    "ts_lookahead_iterator_next",
    "ts_lookahead_iterator_current_symbol",
    "ts_lookahead_iterator_current_symbol_name",
    // Utility
    "ts_set_allocator",
];

/// Grammar C functions compiled from parser.c in each grammar crate.
/// The linker must be forced to include these (via -u) because the Rust
/// wrappers in lib.rs call through LanguageFn function pointers, which
/// the linker can't trace into the C static archives.
const GRAMMAR_C_FUNCTIONS: &[&str] = &[
    "tree_sitter_agda",
    "tree_sitter_arduino",
    "tree_sitter_bash",
    "tree_sitter_bicep",
    "tree_sitter_c",
    "tree_sitter_c_sharp",
    "tree_sitter_cairo",
    "tree_sitter_cmake",
    "tree_sitter_commonlisp",
    "tree_sitter_cpon",
    "tree_sitter_cpp",
    "tree_sitter_css",
    "tree_sitter_cuda",
    "tree_sitter_diff",
    "tree_sitter_dockerfile",
    "tree_sitter_dtd",
    "tree_sitter_elixir",
    "tree_sitter_embedded_template",
    "tree_sitter_erlang",
    "tree_sitter_func",
    "tree_sitter_gitattributes",
    "tree_sitter_glsl",
    "tree_sitter_go",
    "tree_sitter_gosum",
    "tree_sitter_hare",
    "tree_sitter_haskell",
    "tree_sitter_hcl",
    "tree_sitter_hlsl",
    "tree_sitter_html",
    "tree_sitter_java",
    "tree_sitter_javascript",
    "tree_sitter_jsdoc",
    "tree_sitter_json",
    "tree_sitter_julia",
    "tree_sitter_kconfig",
    "tree_sitter_kdl",
    "tree_sitter_kotlin",
    "tree_sitter_lua",
    "tree_sitter_luadoc",
    "tree_sitter_luap",
    "tree_sitter_luau",
    "tree_sitter_make",
    "tree_sitter_markdown",
    "tree_sitter_objc",
    "tree_sitter_ocaml",
    "tree_sitter_ocaml_interface",
    "tree_sitter_ocaml_type",
    "tree_sitter_odin",
    "tree_sitter_php",
    "tree_sitter_php_only",
    "tree_sitter_po",
    "tree_sitter_pony",
    "tree_sitter_printf",
    "tree_sitter_properties",
    "tree_sitter_puppet",
    "tree_sitter_python",
    "tree_sitter_ql",
    "tree_sitter_qmldir",
    "tree_sitter_query",
    "tree_sitter_r",
    "tree_sitter_regex",
    "tree_sitter_requirements",
    "tree_sitter_ron",
    "tree_sitter_ruby",
    "tree_sitter_rust",
    "tree_sitter_scala",
    "tree_sitter_scss",
    "tree_sitter_sql",
    "tree_sitter_squirrel",
    "tree_sitter_starlark",
    "tree_sitter_svelte",
    "tree_sitter_swift",
    "tree_sitter_test",
    "tree_sitter_toml",
    "tree_sitter_tsx",
    "tree_sitter_typescript",
    "tree_sitter_ungrammar",
    "tree_sitter_verilog",
    "tree_sitter_vim",
    "tree_sitter_vue",
    "tree_sitter_wgsl_bevy",
    "tree_sitter_xml",
    "tree_sitter_yaml",
    "tree_sitter_yuck",
    "tree_sitter_zig",
];

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    match target_os.as_str() {
        "macos" | "ios" => {
            // Export tree-sitter C API symbols + grammar constructors.
            for func in TS_API_FUNCTIONS {
                println!("cargo:rustc-cdylib-link-arg=-Wl,-exported_symbol,_{func}");
            }
            for func in GRAMMAR_C_FUNCTIONS {
                println!("cargo:rustc-cdylib-link-arg=-Wl,-exported_symbol,_{func}");
            }
        }
        "linux" | "freebsd" | "openbsd" | "netbsd" => {
            // Use a version script to export ts_* symbols.
            let version_script = out_dir.join("ts_exports.map");
            let mut content = String::from("{\n  global:\n");
            for func in TS_API_FUNCTIONS {
                content.push_str(&format!("    {func};\n"));
            }
            content.push_str("    tree_sitter_*;\n");
            content.push_str("    ts_natives_*;\n");
            content.push_str("  local: *;\n};\n");
            fs::write(&version_script, content).unwrap();

            println!(
                "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
                version_script.display()
            );
        }
        "windows" => {
            // Windows: generate a .def file for exported symbols.
            let def_file = out_dir.join("ts_exports.def");
            let mut content = String::from("EXPORTS\n");
            for func in TS_API_FUNCTIONS {
                content.push_str(&format!("    {func}\n"));
            }
            for func in GRAMMAR_C_FUNCTIONS {
                content.push_str(&format!("    {func}\n"));
            }
            content.push_str("    ts_natives_grammar_count\n");
            content.push_str("    ts_natives_grammar_names\n");
            fs::write(&def_file, content).unwrap();

            println!(
                "cargo:rustc-cdylib-link-arg=/DEF:{}",
                def_file.display()
            );

            // Allow duplicate symbol definitions from multiple tree-sitter versions.
            // Old-API grammar crates (markdown, toml, sql, etc.) depend on tree-sitter
            // 0.19/0.20 while we also have 0.26. Unix linkers deduplicate, but lld-link
            // is stricter. /FORCE:MULTIPLE makes it behave like Unix linkers.
            println!("cargo:rustc-cdylib-link-arg=/FORCE:MULTIPLE");
        }
        _ => {}
    }
}
