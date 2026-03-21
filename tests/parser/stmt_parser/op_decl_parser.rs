use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_infix_def_left() {
    let code = "operator infix * { precedence: 50, associativity: left }";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_infix_def_right() {
    let code = "operator infix % { associativity: right, precedence: 60 }";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_infix_def_none() {
    let code = "operator infix + {
    associativity: none,
    precedence: 80
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_prefix_def() {
    let code = "operator prefix - { precedence: 100 }";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_postfix_def() {
    let code = "operator postfix ! { precedence: 20 }";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_infix_func() {
    let code = "func infix *(_ lhs: Int, _ rhs: Int) -> Int {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_prefix_func() {
    let code = "func prefix -(_ operand: Int) -> Int {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_postfix_func() {
    let code = "func postfix !(_ operand: Int) -> Int {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
