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

mod assign;
mod if_stmt;
mod local_decl;
mod loop_stmt;
mod return_stmt;

use crate::{
    ast_nodes::{Statement, symbol_table},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    /// Translates the given block AST.
    pub fn translate_block(
        &mut self,
        block: &symbol_table::Block,
        exit_block: kasl_ir::Block,
    ) -> bool {
        // Loop over the statements in the block and translate them
        for stmt in &block.body {
            if self.translate_stmt(stmt, exit_block) {
                return true;
            }
        }
        false
    }

    /// Translates a single statement into IR.
    fn translate_stmt(&mut self, stmt: &Statement, exit_block: kasl_ir::Block) -> bool {
        match stmt {
            Statement::Block { block } => {
                return self.translate_block(block, exit_block);
            }
            Statement::LocalVar { var_id } => self.translate_local_var(var_id),
            Statement::LocalConst { var_id } => self.translate_local_const(var_id),
            Statement::Assign { target, value } => self.translate_assign(target, value),
            Statement::Expression { expr } => {
                self.translate_expr(expr);
            }
            Statement::If {
                main,
                else_ifs,
                else_block,
            } => self.translate_if(main, else_ifs, else_block.as_ref(), exit_block),
            Statement::Return { value } => {
                self.translate_return(value, exit_block);
                return true;
            }
            Statement::Loop { count, body } => {
                self.translate_loop(*count, body, exit_block);
            }
        }
        false
    }
}
