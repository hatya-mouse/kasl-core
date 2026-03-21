use crate::{
    assert_func_ctx_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        build_stmts,
        builders::{func_decl, func_param, identifier, if_arm, if_stmt},
        collect_global_decls,
    },
};
use kasl::{error::EK, parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_basic_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "condition",
            Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
            None,
        )],
        None,
        &[if_stmt(if_arm(&[identifier("condition")], &[]), &[], None)],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

// --- ERROR CASES ---

#[test]
fn test_invalid_condition_type() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "condition",
            Some(ParserTypeName::SymbolPath(symbol_path![
                "Float".to_string()
            ])),
            None,
        )],
        None,
        &[if_stmt(if_arm(&[identifier("condition")], &[]), &[], None)],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::NonBoolTypeForCondition);
}
