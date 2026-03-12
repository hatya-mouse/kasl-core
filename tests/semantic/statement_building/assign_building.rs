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

use crate::common::{TestContext, build_stmts, builders::identifier, collect_global_decls};
use insta::{assert_debug_snapshot, assert_yaml_snapshot, sorted_redaction};
use kasl::{
    ExprToken, ExprTokenKind, ParserDeclStmt, ParserDeclStmtKind, ParserFuncParam, ParserScopeStmt,
    ParserScopeStmtKind, Range, symbol_path,
};

#[test]
fn test_simple_assignment() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::Output {
                name: "out".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(5),
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![ParserFuncParam {
                    label: None,
                    name: "number".to_string(),
                    value_type: Some(symbol_path!["Int".to_string()]),
                    def_val: None,
                    range: Range::zero(),
                }],
                return_type: None,
                body: vec![ParserScopeStmt {
                    kind: ParserScopeStmtKind::Assign {
                        target: vec![identifier("out")],
                        value: vec![identifier("number")],
                    },
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
    ];
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
fn test_assign_to_different_type() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::Output {
                name: "out".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(5),
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![ParserFuncParam {
                    label: None,
                    name: "this_is_float".to_string(),
                    value_type: Some(symbol_path!["Float".to_string()]),
                    def_val: None,
                    range: Range::zero(),
                }],
                return_type: None,
                body: vec![ParserScopeStmt {
                    kind: ParserScopeStmtKind::Assign {
                        target: vec![identifier("out")],
                        value: vec![identifier("this_is_float")],
                    },
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).expect_err("This function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_assign_to_immutable_var() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::Input {
                name: "this_is_immutable_because_input".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(0),
                    range: Range::zero(),
                }],
                attrs: vec![],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::FuncDecl {
                is_static: false,
                name: "greet".to_string(),
                params: vec![ParserFuncParam {
                    label: None,
                    name: "number".to_string(),
                    value_type: Some(symbol_path!["Int".to_string()]),
                    def_val: None,
                    range: Range::zero(),
                }],
                return_type: None,
                body: vec![ParserScopeStmt {
                    kind: ParserScopeStmtKind::Assign {
                        target: vec![identifier("this_is_immutable_because_input")],
                        value: vec![identifier("number")],
                    },
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
    ];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    let error = build_stmts(&mut test_ctx).expect_err("This function should generate an error");
    assert_debug_snapshot!(error);
}
