use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_basic_op() {
    let code = "let value = 3 * 2";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_many_ops() {
    let code = "let value = 3 * 2 + 5 * 2";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_parenthesis() {
    let code = "let value = 3 * (2 + 5)";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_many_parentheses() {
    let code = "let value = (3 + 7) * (2 + 5)";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
