//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ParserStatementKind, Program, SymbolPath, SymbolPathComponent, SymbolTable,
    error::ErrorCollector,
    member_collection::{collect_member_functions, collect_member_nests, collect_member_variables},
    symbol_path,
};

/// Loop through the top level and collect type members.
pub fn collect_all_type_members(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
) {
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
            } => {
                let scope_path = symbol_path![SymbolPathComponent::TypeDef(name.clone())];
                collect_type_members(ec, program, &stmt.1.1, scope_path);
            }

            _ => (),
        }
    }
}

// Collects members in a given struct or protocol.
pub fn collect_type_members(
    ec: &mut ErrorCollector,
    program: &mut Program,
    child_symbol_table: &SymbolTable,
    scope_path: SymbolPath,
) {
    collect_member_variables(ec, program, child_symbol_table, &scope_path);
    collect_member_functions(ec, program, child_symbol_table, &scope_path);
    collect_member_nests(ec, program, child_symbol_table, &scope_path);
}
