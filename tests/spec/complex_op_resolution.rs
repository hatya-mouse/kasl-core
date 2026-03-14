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

use crate::common::{TestContext, build_stmts, collect_global_decls, parse_expr};
use insta::{assert_yaml_snapshot, sorted_redaction};

#[test]
fn complex_op_resolution() {
    let mut test_ctx = TestContext::default();

    let code = r#"operator infix || {
    precedence: 1,
    associativity: left
}

func infix *(lhs: Int, rhs: Int) -> Int {
    return Builtin.imul(lhs, rhs)
}

operator infix && {
    precedence: 2,
    associativity: left
}

operator infix == {
    precedence: 3,
    associativity: left
}

operator infix > {
    precedence: 4,
    associativity: left
}

operator infix + {
    precedence: 5,
    associativity: left
}

operator infix * {
    precedence: 6,
    associativity: left
}

operator prefix ! {
    precedence: 7
}"#;
    let parsed = parse_expr(code);
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}
