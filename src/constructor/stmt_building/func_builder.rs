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

use crate::{ParserStatementKind, SymbolTable, stmt_building::stmt_builder::build_statements};

pub fn build_func_bodies(symbol_table: &SymbolTable) {
    // Build statements for every function bodies
    for func in symbol_table.funcs.values() {
        match &func.kind {
            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type,
                body,
            } => {
                if let Some(body) = body {
                    let parsed_body = build_statements(body);
                }
            }
            _ => (),
        }
    }
}
