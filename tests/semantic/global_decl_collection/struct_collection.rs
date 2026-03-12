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
    builders::{
        bool_literal, float_literal, func_decl, int_literal, state_var, struct_decl, struct_field,
    },
    collect_global_decls,
};
use insta::{assert_debug_snapshot, assert_yaml_snapshot, sorted_redaction};
use kasl::symbol_path;

#[test]
fn test_single_field_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[struct_field("field", None, &[float_literal(5.3)])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.type_registry, {
        ".structs" => sorted_redaction(),
        ".path_to_id" => sorted_redaction(),
        ".**.indices" => sorted_redaction(),
    });
}

#[test]
fn test_single_member_func_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[func_decl(false, "new", &[], None, &[])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

#[test]
fn invalid_struct_decl_error() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[state_var("this_is_state", None, &[float_literal(0.5)])],
    )];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("This function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_complex_struct_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![struct_decl(
        "Type",
        &[
            struct_field(
                "float",
                Some(symbol_path!["Float".to_string()]),
                &[float_literal(5.3)],
            ),
            struct_field(
                "bool",
                Some(symbol_path!["Bool".to_string()]),
                &[bool_literal(false)],
            ),
            struct_field(
                "int",
                Some(symbol_path!["Int".to_string()]),
                &[int_literal(5)],
            ),
            func_decl(false, "new", &[], None, &[]),
        ],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.type_registry, {
        ".structs" => sorted_redaction(),
        ".path_to_id" => sorted_redaction(),
        ".**.indices" => sorted_redaction()
    });
}
