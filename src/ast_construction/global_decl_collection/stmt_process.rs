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
    ParserDeclStmt, ParserDeclStmtKind,
    error::Ph,
    global_decl_collection::{FuncDeclInfo, GlobalDeclCollector},
};

impl<'a> GlobalDeclCollector<'a> {
    pub fn process_stmt(&mut self, stmt: &'a ParserDeclStmt) {
        match &stmt.kind {
            ParserDeclStmtKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => self.resolve_input(name, value_type, def_val, attrs, stmt.range),
            ParserDeclStmtKind::Output {
                name,
                value_type,
                def_val,
            } => self.resolve_output(name, value_type, def_val, stmt.range),
            ParserDeclStmtKind::StateVar {
                name,
                value_type,
                def_val,
            } => self.resolve_state_var(name, value_type, def_val, stmt.range),
            ParserDeclStmtKind::GlobalConst {
                name,
                value_type,
                def_val,
            } => self.resolve_global_const(name, value_type, def_val, stmt.range),

            ParserDeclStmtKind::StructDecl { name, body } => {
                self.resolve_struct_decl(name, body, stmt.range)
            }

            ParserDeclStmtKind::FuncDecl {
                is_static,
                name,
                params,
                return_type,
                body,
            } => {
                let info = FuncDeclInfo {
                    is_static: *is_static,
                    name,
                    params,
                    return_type,
                    body,
                };
                self.resolve_global_func_decl(stmt.range, info)
            }

            ParserDeclStmtKind::InfixDefine { symbol, props } => {
                self.resolve_infix_define(symbol, props)
            }
            ParserDeclStmtKind::PrefixDefine { symbol, props } => {
                self.resolve_prefix_define(symbol, props)
            }
            ParserDeclStmtKind::PostfixDefine { symbol, props } => {
                self.resolve_postfix_define(symbol, props)
            }

            ParserDeclStmtKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body: _,
            } => self.resolve_operator_func(op_type, symbol, params, return_type, stmt.range),

            ParserDeclStmtKind::StructField { name, .. } => {
                self.ec
                    .top_level_struct_field(stmt.range, Ph::GlobalDeclCollection, name)
            }
        }
    }
}
