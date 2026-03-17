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
fn simple_float_calculation() {
    let mut test_ctx = TestContext::default();

    let code = r#"operator infix * {
    precedence: 10,
    associativity: left
}

func infix *(lhs: Float, rhs: Int) -> Float {
    return Builtin.fmul(lhs, Builtin.itof(rhs))
}

input in_val = 0.0
output out_val = 0.0

func main() {
    out_val = in_val * 123
}
"#;
    let parsed = parse_expr(code);
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    analyze_structs(&mut test_ctx).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    analyze_scopes(&mut test_ctx).unwrap();
    let blueprint = build_blueprint(&mut test_ctx);

    // Compile the program
    let mut in_val = 3.215f32;
    let mut out_val = 0f32;

    let in_ptr = &mut in_val as *mut f32 as *mut ();
    let out_ptr = &mut out_val as *mut f32 as *mut ();

    execute_program(&mut test_ctx, &blueprint, &[in_ptr], &[out_ptr], &[]);
    assert_eq!(out_val, 3.215f32 * 123f32);
}
