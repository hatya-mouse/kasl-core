use crate::common::parse_expr;
use insta::assert_debug_snapshot;

#[test]
fn parse_simple_func() {
    let code = "func main() {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed)
}

#[test]
fn parse_func_with_param() {
    let code = "func main(label name: Type) {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_return() {
    let code = "func main() -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_param_and_return() {
    let code = "func main(label name: Type) -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}

#[test]
fn parse_func_with_many_params() {
    let code = "func main(label1 name1: Type, label2 name2: Type, label3 name3: Type) -> Return {}";
    let parsed = parse_expr(code);
    assert_debug_snapshot!(parsed);
}
