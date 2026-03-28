//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    assert_func_ctx_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        builders::{func_decl, func_param, int_literal, struct_decl},
        collect_global_decls,
    },
};
use kasl::{error::EK, parser::parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_simple_func_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(false, "greet", &[], None, &[])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_multiple_func_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(false, "one", &[], None, &[]),
        func_decl(false, "two", &[], None, &[]),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_func_with_param() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "greet",
        &[func_param(None, "param", None, Some(&[int_literal(0)]))],
        None,
        &[],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

// --- ERROR CASES ---

#[test]
fn test_type_not_found_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "greet",
        &[],
        Some(ParserTypeName::SymbolPath(symbol_path!["Type".to_string()])),
        &[],
    )];
    let error = collect_global_decls(&mut test_ctx, &parsed).unwrap_err();
    assert_error(&error, EK::TypeNotFound);
}

#[test]
fn test_duplicate_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(false, "greet", &[], None, &[]),
        func_decl(false, "greet", &[], None, &[]),
    ];
    let error = collect_global_decls(&mut test_ctx, &parsed).unwrap_err();
    assert_error(&error, EK::DuplicateName);
}

#[test]
fn test_global_static_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(true, "greet", &[], None, &[])];
    let error = collect_global_decls(&mut test_ctx, &parsed).unwrap_err();
    assert_error(&error, EK::GlobalFuncCannotBeStatic);
}

#[test]
fn test_duplicate_param_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        struct_decl("Type", &[]),
        func_decl(
            false,
            "greet",
            &[
                func_param(
                    None,
                    "message",
                    Some(ParserTypeName::SymbolPath(symbol_path!["Type".to_string()])),
                    None,
                ),
                func_param(
                    None,
                    "message",
                    Some(ParserTypeName::SymbolPath(symbol_path!["Type".to_string()])),
                    None,
                ),
            ],
            None,
            &[],
        ),
    ];
    let error = collect_global_decls(&mut test_ctx, &parsed).unwrap_err();
    assert_error(&error, EK::DuplicateName);
}
