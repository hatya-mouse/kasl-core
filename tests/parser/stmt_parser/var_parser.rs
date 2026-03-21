use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_input_no_type() {
    let code = "input in = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_input_with_type() {
    let code = "input in: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_output_no_type() {
    let code = "output out = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_output_with_type() {
    let code = "output out: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_state_no_type() {
    let code = "state delay = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_state_with_type() {
    let code = "state delay: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_const_no_type() {
    let code = "let factor = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_const_with_type() {
    let code = "let factor: Int = 0";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}
