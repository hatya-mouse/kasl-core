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

use crate::{
    ConstructorError, ParserStatementKind, SymbolTable, TypeDef,
    member_collection::collect_type_members,
};

/// Loop through the scope and collect type members.
pub fn collect_member_nests(
    symbol_table: &SymbolTable,
    type_def: &mut TypeDef,
) -> Result<(), ConstructorError> {
    for stmt in &symbol_table.type_defs {
        match &stmt.1.0.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits: _,
                body: _,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits: _,
                body: _,
            } => match type_def.get_type_def_mut(&name) {
                Some(parent_type_def) => {
                    collect_type_members(&stmt.1.1, parent_type_def)?;
                }
                None => {
                    panic!("TypeDef {} not found while it's defined", name);
                }
            },

            _ => (),
        }
    }

    Ok(())
}
