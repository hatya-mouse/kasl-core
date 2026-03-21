use crate::{
    assert_func_ctx_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        build_stmts,
        builders::{
            assign, func_decl, func_param, global_const, identifier, input, int_literal,
            local_const, local_var, output, state_var,
        },
        collect_global_decls,
    },
};
use kasl::{error::EK, parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_assign_to_output() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        output("out", None, &[int_literal(5)]),
        func_decl(
            false,
            "greet",
            &[func_param(
                None,
                "number",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
                None,
            )],
            None,
            &[assign(&[identifier("out")], &[identifier("number")])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_assign_to_state() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        state_var("state_var", None, &[int_literal(5)]),
        func_decl(
            false,
            "greet",
            &[],
            None,
            &[assign(&[identifier("state_var")], &[int_literal(0)])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_assign_to_local_var() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "greet",
        &[],
        None,
        &[
            local_var("local_var", None, &[int_literal(5)]),
            assign(&[identifier("local_var")], &[int_literal(1)]),
        ],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

// --- ERROR CASES ---

#[test]
fn test_assign_to_different_type() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        output("out", None, &[int_literal(5)]),
        func_decl(
            false,
            "greet",
            &[func_param(
                None,
                "this_is_float",
                Some(ParserTypeName::SymbolPath(symbol_path![
                    "Float".to_string()
                ])),
                None,
            )],
            None,
            &[assign(&[identifier("out")], &[identifier("this_is_float")])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::AssignTypeMismatch);
}

#[test]
fn test_assign_to_input() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        input(
            "this_is_immutable_because_input",
            None,
            &[int_literal(0)],
            &[],
        ),
        func_decl(
            false,
            "greet",
            &[func_param(
                None,
                "number",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
                None,
            )],
            None,
            &[assign(
                &[identifier("this_is_immutable_because_input")],
                &[identifier("number")],
            )],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ImmutableAssignment);
}

#[test]
fn test_assign_to_func_param() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "greet",
        &[func_param(
            None,
            "param",
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            None,
        )],
        None,
        &[assign(&[identifier("param")], &[int_literal(5)])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ImmutableAssignment);
}

#[test]
fn test_assign_to_global_const() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        global_const("const", None, &[int_literal(7)]),
        func_decl(
            false,
            "greet",
            &[func_param(
                None,
                "number",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
                None,
            )],
            None,
            &[assign(&[identifier("const")], &[identifier("number")])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ImmutableAssignment);
}

#[test]
fn test_assign_to_local_const() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "greet",
        &[func_param(
            None,
            "number",
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            None,
        )],
        None,
        &[
            local_const("local", None, &[int_literal(5)]),
            assign(&[identifier("local")], &[identifier("number")]),
        ],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ImmutableAssignment);
}
