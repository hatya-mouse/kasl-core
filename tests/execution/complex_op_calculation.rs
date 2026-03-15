//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::common::{
    TestContext, analyze_scopes, analyze_structs, build_blueprint, build_stmts,
    collect_global_decls, execute_program, parse_expr,
};

#[test]
fn complex_op_calculation() {
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

input in_val: Int = 0
output out_val: Int = 0

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

    let in_ptr = &mut in_val as *mut i32;
    let out_ptr = &mut out_val as *mut i32;

    execute_program(&mut test_ctx, &blueprint, &[in_ptr], &[out_ptr], &[]);
    assert_eq!(out_val, 4284)
}
