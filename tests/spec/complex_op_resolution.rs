use crate::common::{
    TestContext, analyze_scopes, analyze_structs, build_stmts, collect_global_decls, parse_expr,
};
use insta::{assert_yaml_snapshot, sorted_redaction};

#[test]
fn complex_op_resolution() {
    let mut test_ctx = TestContext::default();

    let code = r#"operator infix || {
    precedence: 1,
    associativity: left
}

func infix ||(lhs: Bool, rhs: Bool) -> Bool {
    return Builtin.or(lhs, rhs)
}

operator infix && {
    precedence: 2,
    associativity: left
}

func infix &&(lhs: Bool, rhs: Bool) -> Bool {
    return Builtin.and(lhs, rhs)
}

operator infix == {
    precedence: 3,
    associativity: left
}

func infix ==(lhs: Int, rhs: Int) -> Bool {
    return Builtin.ieq(lhs, rhs)
}

operator infix > {
    precedence: 4,
    associativity: left
}

func infix >(lhs: Int, rhs: Int) -> Bool {
    return Builtin.igt(lhs, rhs)
}

operator infix + {
    precedence: 5,
    associativity: left
}

func infix +(lhs: Int, rhs: Int) -> Int {
    return Builtin.iadd(lhs, rhs)
}

operator infix * {
    precedence: 6,
    associativity: left
}

func infix *(lhs: Int, rhs: Int) -> Int {
    return Builtin.imul(lhs, rhs)
}

operator prefix ! {
    precedence: 7
}

func prefix !(operand: Bool) -> Bool {
    return Builtin.not(operand)
}

let test_expr = 1 + 2 * 3 == 7 && !true || 5 > 2
"#;
    let parsed = parse_expr(code);
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    analyze_structs(&mut test_ctx).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    analyze_scopes(&mut test_ctx).unwrap();

    assert_yaml_snapshot!(test_ctx.prog_ctx.op_ctx, {
        ".infix_operator_properties" => sorted_redaction(),
        ".infix_operators" => sorted_redaction(),
        ".infix_ids" => sorted_redaction(),
        ".prefix_operator_properties" => sorted_redaction(),
        ".prefix_operators" => sorted_redaction(),
        ".prefix_ids" => sorted_redaction(),
        ".postfix_operator_properties" => sorted_redaction(),
        ".postfix_operators" => sorted_redaction(),
        ".postfix_ids" => sorted_redaction(),
    });

    assert_yaml_snapshot!(test_ctx.prog_ctx.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}
