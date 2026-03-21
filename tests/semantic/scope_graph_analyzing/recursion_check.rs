use crate::common::{
    TestContext, analyze_scopes,
    assert::assert_error,
    build_stmts,
    builders::{block, bool_literal, expression, func_call, func_decl, if_arm, if_stmt},
    collect_global_decls,
};
use kasl::error::EK;

// --- SUCCESS CASES ---

#[test]
fn test_not_cyclic_in_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "recursion1",
            &[],
            None,
            &[if_stmt(
                if_arm(
                    &[bool_literal(true)],
                    &[expression(&[func_call("recursion3", &[])])],
                ),
                &[],
                None,
            )],
        ),
        func_decl(
            false,
            "recursion2",
            &[],
            None,
            &[if_stmt(
                if_arm(
                    &[bool_literal(true)],
                    &[expression(&[func_call("recursion3", &[])])],
                ),
                &[],
                None,
            )],
        ),
        func_decl(false, "recursion3", &[], None, &[]),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    analyze_scopes(&mut test_ctx).unwrap();
}

// --- ERROR CASES ---

#[test]
fn test_simple_recursion() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        None,
        &[expression(&[func_call("do_something", &[])])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}

#[test]
fn test_mutual_recursion() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "recursion1",
            &[],
            None,
            &[expression(&[func_call("recursion2", &[])])],
        ),
        func_decl(
            false,
            "recursion2",
            &[],
            None,
            &[expression(&[func_call("recursion1", &[])])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}

#[test]
fn test_cyclic_recursion() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "recursion1",
            &[],
            None,
            &[expression(&[func_call("recursion2", &[])])],
        ),
        func_decl(
            false,
            "recursion2",
            &[],
            None,
            &[expression(&[func_call("recursion3", &[])])],
        ),
        func_decl(
            false,
            "recursion3",
            &[],
            None,
            &[expression(&[func_call("recursion1", &[])])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}

#[test]
fn test_recursion_in_block() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        None,
        &[block(&[expression(&[func_call("do_something", &[])])])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}

#[test]
fn test_recursion_in_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        None,
        &[if_stmt(
            if_arm(
                &[bool_literal(true)],
                &[expression(&[func_call("do_something", &[])])],
            ),
            &[],
            None,
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}

#[test]
fn test_cyclic_recursion_in_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "recursion1",
            &[],
            None,
            &[if_stmt(
                if_arm(
                    &[bool_literal(true)],
                    &[expression(&[func_call("recursion2", &[])])],
                ),
                &[],
                None,
            )],
        ),
        func_decl(
            false,
            "recursion2",
            &[],
            None,
            &[if_stmt(
                if_arm(
                    &[bool_literal(true)],
                    &[expression(&[func_call("recursion3", &[])])],
                ),
                &[],
                None,
            )],
        ),
        func_decl(
            false,
            "recursion3",
            &[],
            None,
            &[if_stmt(
                if_arm(
                    &[bool_literal(true)],
                    &[expression(&[func_call("recursion1", &[])])],
                ),
                &[],
                None,
            )],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    let error = analyze_scopes(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::RecursiveCall);
}
