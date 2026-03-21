use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_simple_struct() {
    let code = "struct Type {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_with_field() {
    let code = "struct Type {
    var field: Int = 0
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_with_func() {
    let code = "struct Type {
    func process(label name: Type) -> Return {}
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_struct_field_and_func() {
    let code = "struct Type {
    var field: Int = 0
    func process(label name: Type) -> Return {}
}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}
