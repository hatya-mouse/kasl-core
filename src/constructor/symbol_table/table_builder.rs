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

use crate::{ParserStatement, ParserStatementKind, SymbolTable};

pub fn build_symbol_table<'a>(
    symbol_table: &mut SymbolTable<'a>,
    statements: &'a [ParserStatement],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserStatementKind::FuncDecl {
                required_by: _,
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                symbol_table.insert_func(name.clone(), &stmt);
            }

            ParserStatementKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } => {
                symbol_table.insert_input(name.clone(), &stmt);
            }

            ParserStatementKind::Output {
                name,
                value_type: _,
            } => {
                symbol_table.insert_output(name.clone(), &stmt);
            }

            ParserStatementKind::State { vars } => {
                for var in vars {
                    symbol_table.insert_state(var.name.clone(), &stmt);
                }
            }

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
                let mut nested_table = SymbolTable::new();
                build_nest_symbol_table(&mut nested_table, body);
                symbol_table.insert_type_def(name.clone(), &stmt, nested_table);
            }

            _ => {}
        }
    }
}

pub fn build_nest_symbol_table<'a>(
    symbol_table: &mut SymbolTable<'a>,
    statements: &'a [ParserStatement],
) {
    for stmt in statements {
        match &stmt.kind {
            ParserStatementKind::Var {
                required_by: _,
                name,
                value_type: _,
                def_val: _,
            } => {
                symbol_table.insert_var(name.clone(), &stmt);
            }

            ParserStatementKind::FuncDecl {
                required_by: _,
                name,
                params: _,
                return_type: _,
                body: _,
            } => {
                symbol_table.insert_func(name.clone(), &stmt);
            }

            ParserStatementKind::Init {
                required_by: _,
                literal_bind: _,
                params: _,
                body: _,
            } => {
                symbol_table.insert_init(&stmt);
            }

            ParserStatementKind::Infix {
                symbol,
                params: _,
                return_type: _,
                attrs: _,
                body: _,
            }
            | ParserStatementKind::Prefix {
                symbol,
                params: _,
                return_type: _,
                body: _,
            }
            | ParserStatementKind::Postfix {
                symbol,
                params: _,
                return_type: _,
                body: _,
            } => {
                symbol_table.insert_operator(symbol.clone(), &stmt);
            }

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
                let mut nested_table = SymbolTable::new();
                build_nest_symbol_table(&mut nested_table, body);
                symbol_table.insert_type_def(name.clone(), &stmt, nested_table);
            }

            _ => (),
        }
    }
}
