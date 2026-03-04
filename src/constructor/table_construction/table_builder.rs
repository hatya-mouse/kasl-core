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
    ParserOperatorType, ParserTopLevelStmt, ParserTopLevelStmtKind, SymbolTable,
    error::{ErrorCollector, Ph},
};

pub fn build_symbol_table<'a>(
    ec: &mut ErrorCollector,
    symbol_table: &mut SymbolTable<'a>,
    statements: &'a [ParserTopLevelStmt],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserTopLevelStmtKind::FuncDecl {
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                symbol_table.insert_func(ec, name.clone(), stmt);
            }

            ParserTopLevelStmtKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } => {
                symbol_table.insert_input(ec, name.clone(), stmt);
            }

            ParserTopLevelStmtKind::Output {
                name,
                value_type: _,
                def_val: _,
            } => {
                symbol_table.insert_output(ec, name.clone(), stmt);
            }

            ParserTopLevelStmtKind::State { vars } => {
                for var in vars {
                    symbol_table.insert_state(ec, var.name.clone(), stmt);
                }
            }

            ParserTopLevelStmtKind::StructDecl { name, body } => {
                let mut nested_table = SymbolTable::new();
                build_nest_symbol_table(ec, &mut nested_table, body);
                symbol_table.insert_type_def(name.clone(), stmt, nested_table);
            }

            ParserTopLevelStmtKind::InfixDefine { symbol, .. } => {
                symbol_table.insert_infix_define(ec, symbol.clone(), stmt);
            }

            ParserTopLevelStmtKind::PrefixDefine { symbol } => {
                symbol_table.insert_prefix_define(ec, symbol.clone(), stmt);
            }

            ParserTopLevelStmtKind::OperatorFunc {
                op_type, symbol, ..
            } => match op_type {
                ParserOperatorType::Infix => symbol_table.insert_infix_func(symbol.clone(), stmt),
                ParserOperatorType::Prefix => symbol_table.insert_prefix_func(symbol.clone(), stmt),
            },

            _ => {
                ec.invalid_top_expr(stmt.range, Ph::StatementBuilding, &stmt.kind.to_string());
            }
        }
    }
}

pub fn build_nest_symbol_table<'a>(
    ec: &mut ErrorCollector,
    symbol_table: &mut SymbolTable<'a>,
    statements: &'a [ParserTopLevelStmt],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserTopLevelStmtKind::ScopeVar {
                name,
                value_type: _,
                def_val: _,
            } => {
                symbol_table.insert_var(ec, name.clone(), stmt);
            }

            ParserTopLevelStmtKind::FuncDecl {
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                symbol_table.insert_func(ec, name.clone(), stmt);
            }

            ParserTopLevelStmtKind::Init {
                literal_bind: _,
                params: _,
                body: _,
            } => {
                symbol_table.insert_init(stmt);
            }

            ParserTopLevelStmtKind::StructDecl { name, body } => {
                let mut nested_table = SymbolTable::new();
                build_nest_symbol_table(ec, &mut nested_table, body);
                symbol_table.insert_type_def(name.clone(), stmt, nested_table);
            }

            ParserTopLevelStmtKind::InfixDefine {
                symbol,
                infix_properties: _,
            } => {
                symbol_table.insert_infix_define(ec, symbol.clone(), stmt);
            }

            ParserTopLevelStmtKind::PrefixDefine { symbol } => {
                symbol_table.insert_prefix_define(ec, symbol.clone(), stmt);
            }

            ParserTopLevelStmtKind::OperatorFunc {
                op_type,
                symbol,
                params: _,
                return_type: _,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => symbol_table.insert_infix_func(symbol.clone(), stmt),
                ParserOperatorType::Prefix => symbol_table.insert_prefix_func(symbol.clone(), stmt),
            },

            _ => {
                ec.invalid_top_expr(stmt.range, Ph::StatementBuilding, &stmt.kind.to_string());
            }
        }
    }
}
