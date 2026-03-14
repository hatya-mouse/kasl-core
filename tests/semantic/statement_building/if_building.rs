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
    TestContext, assert_error, build_stmts,
    builders::{func_decl, func_param, identifier, if_arm, if_stmt},
    collect_global_decls,
};
use insta::{assert_yaml_snapshot, sorted_redaction};
use kasl::{error::EK, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_basic_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "condition",
            Some(symbol_path!["Bool".to_string()]),
            None,
        )],
        None,
        &[if_stmt(if_arm(&[identifier("condition")], &[]), &[], None)],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

// --- ERROR CASES ---

#[test]
fn test_invalid_condition_type() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "condition",
            Some(symbol_path!["Float".to_string()]),
            None,
        )],
        None,
        &[if_stmt(if_arm(&[identifier("condition")], &[]), &[], None)],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::NonBoolTypeForCondition);
}
