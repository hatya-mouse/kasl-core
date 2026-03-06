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
    IfArm, ParserBodyStmt, ParserIfArm, Statement, data::SymbolID,
    resolution::expr_inference::ExprTreeBuilder, stmt_building::StmtBuildingCtx,
};

impl<'a> StmtBuildingCtx<'a> {
    pub fn build_if_stmt(
        &mut self,
        function_id: SymbolID,
        parser_main: &ParserIfArm,
        parser_else_ifs: &Vec<ParserIfArm>,
        parser_else_body: &[ParserBodyStmt],
    ) -> Option<Statement> {
        // Parse the main (if) arm
        let main = self.build_if_arm(function_id, parser_main)?;

        // Parse each arms in the parser_else_ifs
        let mut else_ifs = Vec::new();
        for parser_arm in parser_else_ifs {
            match self.build_if_arm(function_id, parser_arm) {
                Some(arm) => else_ifs.push(arm),
                None => continue,
            }
        }

        // Parse the else body
        let else_body = self.build_func_body_stmt(function_id, parser_else_body);

        Some(Statement::If {
            main,
            else_ifs,
            else_body,
        })
    }

    pub fn build_if_arm(
        &mut self,
        function_id: SymbolID,
        parser_arm: &ParserIfArm,
    ) -> Option<IfArm> {
        // Parse the main (if) condition
        let condition = match self.program.build_expr_tree_from_raw_tokens(
            self.ec,
            &parser_arm.condition,
            self.symbol_table,
        ) {
            Some(parsed_value) => parsed_value,
            None => {
                // Error should have been reported in the build_expr_tree_from_raw_tokens function so we don't need to report it here
                return None;
            }
        };

        // Collect the main body statements
        let body = self.build_func_body_stmt(function_id, &parser_arm.body);

        Some(IfArm::new(condition, body))
    }
}
