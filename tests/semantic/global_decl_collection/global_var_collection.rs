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
    TestContext,
    builders::{float_literal, global_const, input, input_attr, int_literal, output, state_var},
    collect_global_decls,
};
use insta::{assert_yaml_snapshot, sorted_redaction};

#[test]
fn test_simple_input_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![input("in", None, &[int_literal(0)], &[])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_simple_output_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![output("output", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_simple_state_var_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![state_var("state_var", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_simple_let_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![global_const("const", None, &[int_literal(0)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_multiple_variables_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        input("in", None, &[int_literal(0)], &[]),
        output("out", None, &[int_literal(0)]),
        state_var("delay", None, &[int_literal(0)]),
        global_const("const", None, &[int_literal(0)]),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_input_with_attribute() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![input(
        "in",
        None,
        &[float_literal(0.0)],
        &[input_attr(
            "slider",
            &[&[float_literal(0.0)], &[float_literal(1.0)]],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}
