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
    assert_func_ctx_snapshot, assert_scope_registry_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        build_stmts,
        builders::{
            expression, func_call, func_call_arg, func_decl, func_param, global_const, identifier,
            int_literal, local_var, return_stmt,
        },
        collect_global_decls,
    },
};
use kasl::{error::EK, parser::parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_local_var_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_var("local", None, &[int_literal(0)])],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_local_var_definition_with_annotation() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[local_var(
            "local",
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            &[int_literal(0)],
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

#[test]
fn test_local_var_definition_with_func_call() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[],
            Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
            &[return_stmt(Some(&[int_literal(0)]))],
        ),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_var("local", None, &[func_call("do_something", &[])])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_scope_registry_snapshot!(&test_ctx.prog_ctx.scope_registry);
}

// --- ERROR CASES ---

#[test]
fn test_access_to_var_before_definition() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        func_decl(
            false,
            "do_something",
            &[func_param(
                None,
                "number",
                Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
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
                local_var("local", None, &[int_literal(0)]),
            ],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::VarNotFound);
}

#[test]
fn test_local_var_shadowing() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        global_const("int_value", None, &[int_literal(0)]),
        func_decl(
            false,
            "main",
            &[],
            None,
            &[local_var("int_value", None, &[int_literal(0)])],
        ),
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::DuplicateName);
}
