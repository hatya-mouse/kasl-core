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
    IfArm, ParserIfArm, ParserScopeStmt, Range, Statement,
    error::Ph,
    expr_engine::resolve_expr,
    statement_building::BlockStmtBuilder,
    type_registry::{PrimitiveType, ResolvedType},
};

impl BlockStmtBuilder<'_> {
    pub fn build_if_stmt(
        &mut self,
        main: &ParserIfArm,
        else_ifs: &[ParserIfArm],
        else_body: Option<&Vec<ParserScopeStmt>>,
        else_range: Option<Range>,
    ) -> Option<Statement> {
        // Build the arms
        let main_arm = self.build_if_arm(main)?;
        let else_ifs = else_ifs
            .iter()
            .map(|arm| self.build_if_arm(arm))
            .collect::<Option<Vec<_>>>()?;
        // Build the else block
        // None is allowed because the else block is optional
        let else_block = else_body
            .map(|arm| self.build_scope_block(arm, self.scope_id, else_range.unwrap_or_default()));

        // Return the constructed if statement
        Some(Statement::If {
            main: main_arm,
            else_ifs,
            else_block,
        })
    }

    fn build_if_arm(&mut self, arm: &ParserIfArm) -> Option<IfArm> {
        // Resolve the condition expression and verify it has a bool type
        let condition = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            &arm.condition,
        )?;
        if condition.value_type != ResolvedType::Primitive(PrimitiveType::Bool) {
            self.ec.non_bool_type_for_condition(
                arm.range,
                Ph::StatementCollection,
                self.prog_ctx
                    .type_registry
                    .format_type(&condition.value_type),
            );
            return None;
        }

        // Create a block for the arm's body
        let block = self.build_scope_block(&arm.body, self.scope_id, arm.range);
        Some(IfArm { condition, block })
    }
}
