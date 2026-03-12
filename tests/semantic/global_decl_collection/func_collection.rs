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

use crate::common::collect_global_decls;
use insta::{assert_debug_snapshot, assert_yaml_snapshot, sorted_redaction};
use kasl::{
    CompilationState, NameSpace, ParserDeclStmt, ParserDeclStmtKind, Range,
    error::ErrorCollector,
    symbol_path,
    symbol_table::{FuncBodyMap, OpBodyMap},
};

#[test]
pub fn test_simple_func_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

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
    collect_global_decls(
        &mut ec,
        &mut name_space,
        &mut func_body_map,
        &mut op_body_map,
        &mut comp_state,
        &parsed,
    )
    .unwrap();
    assert_yaml_snapshot!(comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

#[test]
pub fn test_multiple_func_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

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
    collect_global_decls(
        &mut ec,
        &mut name_space,
        &mut func_body_map,
        &mut op_body_map,
        &mut comp_state,
        &parsed,
    )
    .unwrap();
    assert_yaml_snapshot!(comp_state.func_ctx, {
        ".funcs" => sorted_redaction(),
        ".member_functions" => sorted_redaction(),
        ".static_functions" => sorted_redaction(),
        ".global_functions" => sorted_redaction()
    });
}

#[test]
pub fn test_invalid_func() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

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
    let error = collect_global_decls(
        &mut ec,
        &mut name_space,
        &mut func_body_map,
        &mut op_body_map,
        &mut comp_state,
        &parsed,
    )
    .expect_err("The function should generate an error");
    assert_debug_snapshot!(error);
}
