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

use crate::{
    assert_func_ctx_snapshot,
    common::{
        TestContext,
        assert::assert_error,
        build_stmts,
        builders::{
            float_literal, func_decl, func_param, identifier, if_arm, if_stmt, int_literal,
            return_stmt,
        },
        collect_global_decls,
    },
};
use kasl::{error::EK, parser_ast::ParserTypeName, symbol_path};

// --- SUCCESS CASES ---

#[test]
fn test_return_on_no_return_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(false, "main", &[], None, &[return_stmt(None)])];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_return_int() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
        &[return_stmt(Some(&[int_literal(0)]))],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_return_in_both_if_and_else() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "param",
            Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
            None,
        )],
        Some(ParserTypeName::SymbolPath(symbol_path![
            "Float".to_string()
        ])),
        &[if_stmt(
            if_arm(
                &[identifier("param")],
                &[return_stmt(Some(&[float_literal(5.0)]))],
            ),
            &[],
            Some(&[return_stmt(Some(&[float_literal(3.0)]))]),
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_return_after_if() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[func_param(
            None,
            "param",
            Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
            None,
        )],
        Some(ParserTypeName::SymbolPath(symbol_path![
            "Float".to_string()
        ])),
        &[
            if_stmt(if_arm(&[identifier("param")], &[]), &[], None),
            return_stmt(Some(&[float_literal(5.0)])),
        ],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

#[test]
fn test_return_in_if_else_if_else() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[
            func_param(
                None,
                "if_param",
                Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
                None,
            ),
            func_param(
                None,
                "if_else_param",
                Some(ParserTypeName::SymbolPath(symbol_path!["Bool".to_string()])),
                None,
            ),
        ],
        Some(ParserTypeName::SymbolPath(symbol_path![
            "Float".to_string()
        ])),
        &[if_stmt(
            if_arm(
                &[identifier("if_param")],
                &[return_stmt(Some(&[float_literal(5.0)]))],
            ),
            &[if_arm(
                &[identifier("if_else_param")],
                &[return_stmt(Some(&[float_literal(3.0)]))],
            )],
            Some(&[return_stmt(Some(&[float_literal(1.0)]))]),
        )],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    build_stmts(&mut test_ctx).unwrap();
    assert_func_ctx_snapshot!(&test_ctx.prog_ctx.func_ctx);
}

// --- ERROR CASES ---

#[test]
fn test_return_int_on_no_return_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "main",
        &[],
        None,
        &[return_stmt(Some(&[int_literal(0)]))],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ReturnValueForNoReturnFunc);
}

#[test]
fn test_return_nothing_on_int_return_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        Some(ParserTypeName::SymbolPath(symbol_path!["Int".to_string()])),
        &[return_stmt(None)],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ReturnWithoutValueForReturnFunc);
}

#[test]
fn test_return_int_on_float_return_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![func_decl(
        false,
        "do_something",
        &[],
        Some(ParserTypeName::SymbolPath(symbol_path![
            "Float".to_string()
        ])),
        &[return_stmt(Some(&[int_literal(0)]))],
    )];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).unwrap_err();
    assert_error(&error, EK::ReturnTypeMismatch);
}
