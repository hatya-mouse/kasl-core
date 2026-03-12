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
use insta::{assert_yaml_snapshot, sorted_redaction};
use kasl::{
    CompilationState, ExprToken, ExprTokenKind, NameSpace, ParserDeclStmt, ParserDeclStmtKind,
    ParserInputAttribute, Range,
    error::ErrorCollector,
    symbol_table::{FuncBodyMap, OpBodyMap},
};

#[test]
pub fn test_simple_input_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::Input {
            name: "in".to_string(),
            value_type: None,
            def_val: vec![ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            }],
            attrs: vec![],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
pub fn test_simple_output_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::Output {
            name: "output".to_string(),
            value_type: None,
            def_val: vec![ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            }],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
pub fn test_simple_state_var_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StateVar {
            name: "state".to_string(),
            value_type: None,
            def_val: vec![ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            }],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
pub fn test_simple_let_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::GlobalConst {
            name: "const".to_string(),
            value_type: None,
            def_val: vec![ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            }],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
pub fn test_multiple_variables_resolution() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::Input {
                name: "in".to_string(),
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
            kind: ParserDeclStmtKind::Output {
                name: "out".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(0),
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::StateVar {
                name: "delay".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(0),
                    range: Range::zero(),
                }],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::GlobalConst {
                name: "const".to_string(),
                value_type: None,
                def_val: vec![ExprToken {
                    kind: ExprTokenKind::IntLiteral(0),
                    range: Range::zero(),
                }],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}

#[test]
pub fn test_input_with_attribute() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut op_body_map = OpBodyMap::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::Input {
            name: "in".to_string(),
            value_type: None,
            def_val: vec![ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            }],
            attrs: vec![ParserInputAttribute {
                name: "slider".to_string(),
                args: vec![
                    vec![ExprToken {
                        kind: ExprTokenKind::FloatLiteral(0.0),
                        range: Range::zero(),
                    }],
                    vec![ExprToken {
                        kind: ExprTokenKind::FloatLiteral(1.0),
                        range: Range::zero(),
                    }],
                ],
                range: Range::zero(),
            }],
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
    assert_yaml_snapshot!(comp_state.scope_registry, {
        ".scopes" => sorted_redaction(),
        ".variables" => sorted_redaction(),
        ".**.name_to_id" => sorted_redaction()
    });
}
