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

use crate::{Statement, backend::func_translator::FuncTranslator, symbol_table::Block};

impl FuncTranslator<'_> {
    /// Translates the given block. This method does not create any new blocks.
    pub fn translate_block(&mut self, block: &Block) {
        // Loop over the statements in the function and translate them
        for stmt in &block.body {
            self.translate_stmt(stmt);
        }
    }

    fn translate_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Block { block } => self.translate_block(block),
            Statement::LocalVar { var_id } => self.translate_local_var(var_id),
            Statement::LocalConst { var_id } => self.translate_local_const(var_id),
            Statement::Assign { target, value } => self.translate_assign(target, value),
            Statement::Expression { expr } => {}
            Statement::If {
                main,
                else_ifs,
                else_block,
            } => {}
            Statement::Return { value } => {}
        }
    }
}
