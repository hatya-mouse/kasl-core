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

use crate::{
    ParserTopLevelStmt, ParserTopLevelStmtKind, SymbolPath, SymbolPathComponent, SymbolTable,
    error::{ErrorCollector, Ph},
    symbol_path,
};

pub fn build_symbol_table<'a>(
    ec: &mut ErrorCollector,
    symbol_table: &mut SymbolTable<'a>,
    statements: &'a [ParserTopLevelStmt],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserTopLevelStmtKind::FuncDecl { name, .. }
            | ParserTopLevelStmtKind::Input { name, .. }
            | ParserTopLevelStmtKind::Output { name, .. }
            | ParserTopLevelStmtKind::StateVar { name, .. } => {
                symbol_table.insert_statement(symbol_path![name.clone()], stmt);
            }

            ParserTopLevelStmtKind::InfixDefine { symbol, .. }
            | ParserTopLevelStmtKind::OperatorFunc { symbol, .. } => {
                symbol_table.insert_statement(symbol_path![symbol.clone()], stmt);
            }

            ParserTopLevelStmtKind::StructDecl { name, body } => {
                let struct_path = symbol_path![name.clone()];
                build_nest_symbol_table(ec, symbol_table, &struct_path, body);
                symbol_table.insert_statement(struct_path, stmt);
            }

            _ => {
                ec.invalid_top_expr(stmt.range, Ph::StatementBuilding, &stmt.kind.to_string());
            }
        }
    }
}

pub fn build_nest_symbol_table<'a>(
    ec: &mut ErrorCollector,
    symbol_table: &mut SymbolTable<'a>,
    type_path: &SymbolPath,
    statements: &'a [ParserTopLevelStmt],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserTopLevelStmtKind::ScopeVar { name, .. }
            | ParserTopLevelStmtKind::FuncDecl { name, .. } => {
                symbol_table.insert_statement(
                    type_path.extended(SymbolPathComponent {
                        symbol: name.clone(),
                    }),
                    stmt,
                );
            }

            ParserTopLevelStmtKind::StructDecl { name, body } => {
                let struct_path = type_path.extended(SymbolPathComponent {
                    symbol: name.clone(),
                });
                build_nest_symbol_table(ec, symbol_table, &struct_path, body);
                symbol_table.insert_statement(struct_path, stmt);
            }

            _ => {
                ec.invalid_top_expr(stmt.range, Ph::StatementBuilding, &stmt.kind.to_string());
            }
        }
    }
}
