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

use crate::common::{TestContext, collect_global_decls};
use insta::{assert_debug_snapshot, assert_yaml_snapshot, sorted_redaction};
use kasl::{ParserDeclStmt, ParserDeclStmtKind, ParserFuncParam, Range, symbol_path};

#[test]
fn test_simple_func_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::FuncDecl {
            is_static: false,
            name: "greet".to_string(),
            params: vec![],
            return_type: None,
            body: vec![],
        },
        range: Range::zero(),
    }];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

#[test]
fn test_multiple_func_resolution() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "one".to_string(),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "two".to_string(),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            range: Range::zero(),
        },
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

#[test]
fn test_invalid_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::FuncDecl {
            is_static: false,
            name: "greet".to_string(),
            params: vec![],
            return_type: Some(symbol_path!["Type".to_string()]),
            body: vec![],
        },
        range: Range::zero(),
    }];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("The function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_duplicate_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            range: Range::zero(),
        },
    ];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("The function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_global_static_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::FuncDecl {
            is_static: true,
            name: "greet".to_string(),
            params: vec![],
            return_type: None,
            body: vec![],
        },
        range: Range::zero(),
    }];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("The function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_duplicate_param_func() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::StructDecl {
                name: "Type".to_string(),
                body: vec![],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![
                    ParserFuncParam {
                        label: None,
                        name: "message".to_string(),
                        value_type: Some(symbol_path!["Type".to_string()]),
                        def_val: None,
                        range: Range::zero(),
                    },
                    ParserFuncParam {
                        label: None,
                        name: "message".to_string(),
                        value_type: Some(symbol_path!["Type".to_string()]),
                        def_val: None,
                        range: Range::zero(),
                    },
                ],
                return_type: None,
                body: vec![],
            },
            range: Range::zero(),
        },
    ];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("The function should generate an error");
    assert_debug_snapshot!(error);
}
