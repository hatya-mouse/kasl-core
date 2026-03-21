use crate::{
    assert_func_ctx_snapshot,
    common::{
        TestContext, build_stmts,
        builders::{expression, func_call, func_decl},
        collect_global_decls,
    },
};

// -- SUCCESS CASES ---

#[test]
fn test_func_call_stmt() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(false, "do_something", &[], None, &[]),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[expression(&[func_call("do_something", &[])])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}
