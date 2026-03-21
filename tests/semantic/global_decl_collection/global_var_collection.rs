use crate::{
    assert_scope_registry_snapshot,
    common::{
        TestContext,
        builders::{
            float_literal, global_const, input, input_attr, int_literal, output, state_var,
        },
        collect_global_decls,
    },
};

// --- SUCCESS CASES ---

#[test]
fn test_simple_input_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![input("in", None, &[int_literal(0)], &[])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_simple_output_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![output("output", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_simple_state_var_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![state_var("state_var", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_simple_let_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![global_const("const", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_multiple_variables_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        input("in", None, &[int_literal(0)], &[]),
        output("out", None, &[int_literal(0)]),
        state_var("delay", None, &[int_literal(0)]),
        global_const("const", None, &[int_literal(0)]),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_input_with_attribute() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![input(
        "in",
        None,
        &[float_literal(0.0)],
        &[input_attr(
            "slider",
            &[&[float_literal(0.0)], &[float_literal(1.0)]],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}
