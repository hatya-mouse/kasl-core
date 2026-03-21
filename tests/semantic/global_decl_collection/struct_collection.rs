use crate::{
    assert_type_registry_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        builders::{
            bool_literal, float_literal, func_decl, int_literal, state_var, struct_decl,
            struct_field,
        },
        collect_global_decls,
    },
};
use kasl::{error::EK, parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_single_field_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[struct_field("field", None, &[float_literal(5.3)])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_type_registry_snapshot!(&test_ctx.prog_ctx.type_registry);
}

#[test]
fn test_single_member_func_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[func_decl(false, "new", &[], None, &[])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_type_registry_snapshot!(&test_ctx.prog_ctx.type_registry);
}

#[test]
fn test_complex_struct_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[
            struct_field(
                "float",
                Some(ParserTypeName::SymbolPath(symbol_path![
                    "Float".to_string()
                ])),
                &[float_literal(5.3)],
            ),
            struct_field(
                "bool",
                Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
                &[bool_literal(false)],
            ),
            struct_field(
                "int",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
                &[int_literal(5)],
            ),
            func_decl(false, "new", &[], None, &[]),
        ],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_type_registry_snapshot!(&test_ctx.prog_ctx.type_registry);
}

// --- ERROR CASES ---

#[test]
fn invalid_struct_decl_error() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[state_var("this_is_state", None, &[float_literal(0.5)])],
    )];
    let error = collect_global_decls(&mut test_ctx, &parsed).unwrap_err();
    assert_error(&error, EK::InvalidStructStmt);
}
