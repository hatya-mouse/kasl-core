//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast_nodes::{Range, Statement, scope_manager::VariableKind},
    parser::{ExprToken, parser_ast::ParserTypeName},
    semantic_analysis::stmt_builder::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_local_var(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        stmt_range: Range,
    ) -> Option<Statement> {
        // Build and register the scope variable
        let var_id = self.build_and_register_scope_var(
            name,
            value_type,
            def_val,
            stmt_range,
            VariableKind::LocalVar,
        )?;

        // Return the local var statement
        let local_var_stmt = Statement::LocalVar { var_id };
        Some(local_var_stmt)
    }

    pub fn build_local_const(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        stmt_range: Range,
    ) -> Option<Statement> {
        // Build and register the scope variable
        let var_id = self.build_and_register_scope_var(
            name,
            value_type,
            def_val,
            stmt_range,
            VariableKind::LocalConst,
        )?;

        // Return the local const statement
        let local_const_stmt = Statement::LocalConst { var_id };
        Some(local_const_stmt)
    }
}
