use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_int() {
    let code = "let int = 3";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_negative_int() {
    let code = "let int = -5";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_float() {
    let code = "let float = 5.2";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_negative_float() {
    let code = "let float = -2.6";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_bool_true() {
    let code = "let bool = true";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_bool_false() {
    let code = "let bool = false";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
