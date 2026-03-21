use crate::{
    assert_func_ctx_snapshot, assert_scope_registry_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        build_stmts,
        builders::{
            expression, func_call, func_call_arg, func_decl, func_param, global_const, identifier,
            int_literal, local_const, return_stmt,
        },
        collect_global_decls,
    },
};
use kasl::{error::EK, parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_local_let_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_const("local", None, &[int_literal(0)])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_local_let_definition_with_annotation() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_const(
            "local",
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            &[int_literal(0)],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_local_const_definition_with_func_call() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[],
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            &[return_stmt(Some(&[int_literal(0)]))],
        ),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_const(
                "local",
                None,
                &[func_call("do_something", &[])],
            )],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

// --- ERROR CASES ---

#[test]
fn test_access_to_let_before_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[func_param(
                None,
                "number",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
                None,
            )],
            None,
            &[],
        ),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[
                expression(&[func_call(
                    "do_something",
                    &[func_call_arg(None, &[identifier("local")])],
                )]),
                local_const("local", None, &[int_literal(0)]),
            ],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::VarNotFound);
}

#[test]
fn test_local_let_shadowing() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        global_const("this_is_const", None, &[int_literal(0)]),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_const("this_is_const", None, &[int_literal(0)])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::DuplicateName);
}
