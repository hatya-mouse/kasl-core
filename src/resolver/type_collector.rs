//
// Copyright 2025 Shuntaro Kasatani
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

use crate::{ParserStatement, ParserStatementKind, TypeDef};

pub fn collect_types(stmts: &Vec<ParserStatement>) -> Vec<TypeDef> {
    let mut types = Vec::new();

    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits: _,
                body,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits: _,
                body,
            } => {
                let mut type_def = TypeDef::new(name.clone());
                let child_types = collect_types(&body);
                type_def.types = child_types;
                types.push(type_def);
            }
            _ => (),
        }
    }

    types
}
