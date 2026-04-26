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
    ast_nodes::{
        IfArm, Range, Statement,
        type_registry::{PrimitiveType, ResolvedType},
    },
    error::Ph,
    parser::{ParserIfArm, ParserScopeStmt},
    semantic_analysis::{expr_engine::resolve_expr, stmt_builder::BlockStmtBuilder},
};

impl BlockStmtBuilder<'_> {
    pub fn build_if_stmt(
        &mut self,
        parser_main: &ParserIfArm,
        parser_else_ifs: &[ParserIfArm],
        parser_else_body: Option<&Vec<ParserScopeStmt>>,
        else_range: Option<Range>,
    ) -> Option<Statement> {
        // Create a new flow node for the operations after the if statement
        let before_node = self.flow_graph_builder.current_node();
        let after_node = self.flow_graph_builder.new_node();
        // Create a vector of flow nodes for if, if-else and else nodes
        let mut branch_in_nodes = Vec::new();
        let mut branch_out_nodes = Vec::new();

        // Create a new flow node for the main if arm
        let main_flow_node = self.flow_graph_builder.new_node();
        self.flow_graph_builder.switch_node(main_flow_node);
        branch_in_nodes.push(main_flow_node);
        // Build the main if arm
        let main_arm = self.build_if_arm(parser_main)?;
        // Add the current branch to the branch out nodes
        branch_out_nodes.push(self.flow_graph_builder.current_node());

        let mut else_ifs = Vec::new();
        for parser_arm in parser_else_ifs {
            // Create a new flow node for the else-if arms
            let else_if_node = self.flow_graph_builder.new_node();
            self.flow_graph_builder.switch_node(else_if_node);
            branch_in_nodes.push(else_if_node);
            // Build the else-if arm
            else_ifs.push(self.build_if_arm(parser_arm)?);
            // Add the current branch to the branch out nodes
            branch_out_nodes.push(self.flow_graph_builder.current_node());
        }

        let else_block = if let Some(parser_else_body) = parser_else_body {
            // Create a new flow node for the else arm
            let else_node = self.flow_graph_builder.new_node();
            self.flow_graph_builder.switch_node(else_node);
            branch_in_nodes.push(else_node);

            // Build the else block
            let built_block = self.build_scope_block(
                parser_else_body,
                self.scope_id,
                else_range.unwrap_or_default(),
            );

            // Add the current branch to the branch out nodes
            branch_out_nodes.push(self.flow_graph_builder.current_node());

            Some(built_block)
        } else {
            // If the if statement does not have else arm, add an edge from the before_node to the after_node
            self.flow_graph_builder.add_edge(before_node, after_node);
            // None is allowed because the else block is optional
            None
        };

        // Add the edges from the before node to in-nodes, and the edges from the out-nodes to after node
        for (in_node, out_node) in branch_in_nodes.iter().zip(branch_out_nodes) {
            self.flow_graph_builder.add_edge(before_node, *in_node);
            self.flow_graph_builder.add_edge(out_node, after_node);
        }
        // Switch to the after node
        self.flow_graph_builder.switch_node(after_node);

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
                Ph::StatementBuilding,
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
