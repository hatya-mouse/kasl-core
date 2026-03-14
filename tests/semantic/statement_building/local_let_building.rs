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
    builders::{
        expression, func_call, func_call_arg, func_decl, func_param, global_const, identifier,
        int_literal, local_const, return_stmt,
    },
    collect_global_decls,
};
use insta::{assert_yaml_snapshot, sorted_redaction};
use kasl::{error::EK, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_local_let_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_const("local", None, &[int_literal(0)])],
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

#[test]
fn test_local_let_definition_with_annotation() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_const(
            "local",
            Some(symbol_path!["Int".to_string()]),
            &[int_literal(0)],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
fn test_local_const_definition_with_func_call() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[],
            Some(symbol_path!["Int".to_string()]),
            &[return_stmt(Some(&[int_literal(0)]))],
        ),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_const(
                "local",
                None,
                &[func_call("do_something", &[])],
            )],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

// --- ERROR CASES ---

#[test]
fn test_access_to_let_before_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[func_param(
                None,
                "number",
                Some(symbol_path!["Int".to_string()]),
                None,
            )],
            None,
            &[],
        ),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[
                expression(&[func_call(
                    "do_something",
                    &[func_call_arg(None, &[identifier("local")])],
                )]),
                local_const("local", None, &[int_literal(0)]),
            ],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::VarNotFound);
}

#[test]
fn test_local_let_shadowing() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        global_const("this_is_const", None, &[int_literal(0)]),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_const("this_is_const", None, &[int_literal(0)])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::DuplicateVarName);
}
