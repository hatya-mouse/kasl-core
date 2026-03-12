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

use crate::common::collect_types;
use insta::{assert_yaml_snapshot, sorted_redaction};
use kasl::{
    CompilationState, NameSpace, ParserDeclStmt, ParserDeclStmtKind, Range, error::ErrorCollector,
};

#[test]
fn collect_single_type() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: "Type".to_string(),
            body: vec![],
        },
        range: Range::zero(),
    }];
    collect_types(&mut ec, &mut name_space, &mut comp_state, &parsed).unwrap();
    assert_yaml_snapshot!(comp_state.type_registry, {
        ".structs" => sorted_redaction(),
        ".path_to_id" => sorted_redaction()
    });
}

#[test]
fn collect_multiple_types() {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut comp_state = CompilationState::default();

    let parsed = vec![
        ParserDeclStmt {
            kind: ParserDeclStmtKind::StructDecl {
                name: "Animal".to_string(),
                body: vec![],
            },
            range: Range::zero(),
        },
        ParserDeclStmt {
            kind: ParserDeclStmtKind::StructDecl {
                name: "Fish".to_string(),
                body: vec![],
            },
            range: Range::zero(),
        },
    ];
    collect_types(&mut ec, &mut name_space, &mut comp_state, &parsed).unwrap();
    assert_yaml_snapshot!(comp_state.type_registry, {
        ".structs" => sorted_redaction(),
        ".path_to_id" => sorted_redaction()
    });
}
