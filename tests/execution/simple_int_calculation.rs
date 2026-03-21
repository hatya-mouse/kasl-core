use crate::common::{
    TestContext, analyze_scopes, analyze_structs, build_blueprint, build_stmts,
    collect_global_decls, execute_program, parse_expr,
};

#[test]
fn simple_int_calculation() {
    let mut test_ctx = TestContext::default();

    let code = r#"operator infix * {
    precedence: 10,
    associativity: left
}

func infix *(lhs: Int, rhs: Int) -> Int {
    return Builtin.imul(lhs, rhs)
}

input in_val = 0
output out_val = 0

func main() {
    out_val = in_val * 102
}
"#;
    let parsed = parse_expr(code);
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    analyze_structs(&mut test_ctx).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    analyze_scopes(&mut test_ctx).unwrap();
    let blueprint = build_blueprint(&mut test_ctx);

    // Compile the program
    let mut in_val = 42i32;
    let mut out_val = 0i32;

    let in_ptr = &mut in_val as *mut i32 as *mut ();
    let out_ptr = &mut out_val as *mut i32 as *mut ();

    execute_program(&mut test_ctx, &blueprint, &[in_ptr], &[out_ptr], &[]);
    assert_eq!(out_val, 4284);
}
