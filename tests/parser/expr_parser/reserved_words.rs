use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
#[should_panic]
fn parse_expr_with_input_keyword() {
    let code = "let value = 6 + input";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_output_keyword() {
    let code = "let output = 2 + 6";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_var_keyword() {
    let code = "let var = 2 + 6";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_let_keyword() {
    let code = "let value = 2 + let";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_state_keyword() {
    let code = "let value = state + 3";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_static_keyword() {
    let code = "let static = in + 3";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_func_keyword() {
    let code = "let value = func + 5";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_return_keyword() {
    let code = "let value = in + return";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_if_keyword() {
    let code = "let if = 5 - 2";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_else_keyword() {
    let code = "let value = else - 2";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_struct_keyword() {
    let code = "let value = 2 + struct";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_operator_keyword() {
    let code = "let value = 8 * operator";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_infix_keyword() {
    let code = "let infix = 3 / 6";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_prefix_keyword() {
    let code = "let op = 3 / prefix";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
#[should_panic]
fn parse_expr_with_postfix_keyword() {
    let code = "let value = 5.2 * postfix";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
