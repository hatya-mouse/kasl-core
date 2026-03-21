use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_single_identifier() {
    let code = "let value = num";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_identifier_access() {
    let code = "let value = type.num";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_identifier_access_to_func() {
    let code = "let value = new().first";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_access() {
    let code = "let value = new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_access_to_identifier() {
    let code = "let value = num.new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_chain() {
    let code = "let value = chain().chain().new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_chain_on_expr() {
    let code = "let value = (1 + 7).chain().chain().new()";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
