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
use kasl::{ExprToken, ExprTokenKind, ParserDeclStmt, ParserDeclStmtKind, Range, symbol_path};

#[test]
fn test_single_field_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: "Type".to_string(),
            body: vec![ParserDeclStmt {
                kind: ParserDeclStmtKind::StructField {
                    name: "field".to_string(),
                    value_type: None,
                    def_val: vec![ExprToken {
                        kind: ExprTokenKind::FloatLiteral(5.3),
                        range: Range::zero(),
                    }],
                },
                range: Range::zero(),
            }],
        },
        range: Range::zero(),
    }];
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

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: "Type".to_string(),
            body: vec![ParserDeclStmt {
                kind: ParserDeclStmtKind::FuncDecl {
                    is_static: false,
                    name: "new".to_string(),
                    params: vec![],
                    return_type: None,
                    body: vec![],
                },
                range: Range::zero(),
            }],
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
fn invalid_struct_decl_error() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: "Type".to_string(),
            body: vec![ParserDeclStmt {
                kind: ParserDeclStmtKind::StateVar {
                    name: "this_is_state".to_string(),
                    value_type: None,
                    def_val: vec![ExprToken {
                        kind: ExprTokenKind::FloatLiteral(0.5),
                        range: Range::zero(),
                    }],
                },
                range: Range::zero(),
            }],
        },
        range: Range::zero(),
    }];
    let error = collect_global_decls(&mut test_ctx, &parsed)
        .expect_err("This function should generate an error");
    assert_debug_snapshot!(error);
}

#[test]
fn test_complex_struct_collection() {
    let mut test_ctx = TestContext::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: "Type".to_string(),
            body: vec![
                ParserDeclStmt {
                    kind: ParserDeclStmtKind::StructField {
                        name: "float".to_string(),
                        value_type: Some(symbol_path!["Float".to_string()]),
                        def_val: vec![ExprToken {
                            kind: ExprTokenKind::FloatLiteral(5.3),
                            range: Range::zero(),
                        }],
                    },
                    range: Range::zero(),
                },
                ParserDeclStmt {
                    kind: ParserDeclStmtKind::StructField {
                        name: "bool".to_string(),
                        value_type: Some(symbol_path!["Bool".to_string()]),
                        def_val: vec![ExprToken {
                            kind: ExprTokenKind::BoolLiteral(false),
                            range: Range::zero(),
                        }],
                    },
                    range: Range::zero(),
                },
                ParserDeclStmt {
                    kind: ParserDeclStmtKind::StructField {
                        name: "int".to_string(),
                        value_type: Some(symbol_path!["Int".to_string()]),
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
                        name: "new".to_string(),
                        params: vec![],
                        return_type: None,
                        body: vec![],
                    },
                    range: Range::zero(),
                },
            ],
        },
        range: Range::zero(),
    }];
    collect_global_decls(&mut test_ctx, &parsed).unwrap();
    assert_yaml_snapshot!(test_ctx.comp_state.type_registry, {
        ".structs" => sorted_redaction(),
        ".path_to_id" => sorted_redaction(),
        ".**.indices" => sorted_redaction()
    });
}
