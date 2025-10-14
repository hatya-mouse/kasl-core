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

use crate::{ParserStatementKind, SymbolTable, TypeDef};

pub fn collect_types(symbol_table: &SymbolTable) -> Vec<TypeDef> {
    let mut types = Vec::new();

    for (_, stmt) in &symbol_table.type_defs {
        match &stmt.0.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits: _,
                body: _,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits: _,
                body: _,
            } => {
                let nested_types = collect_types(&stmt.1);
                types.push(TypeDef {
                    name: name.clone(),
                    inherits: Vec::new(),
                    vars: Vec::new(),
                    inits: Vec::new(),
                    funcs: Vec::new(),
                    types: nested_types,
                    operators: Vec::new(),
                })
            }

            _ => {
                panic!(
                    "SymbolTable::type_defs must only include StructDecl(s) and ProtocolDecl(s)",
                );
            }
        }
    }

    types
}
