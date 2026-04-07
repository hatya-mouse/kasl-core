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
    ast::{
        FunctionID, OperatorID, Range, ScopeID,
        flow_graph::{FlowGraph, FlowID},
    },
    ast_construction::flow_graph_analyzing::FlowGraphAnalyzer,
    error::Ph,
};
use std::collections::{HashMap, VecDeque};

impl FlowGraphAnalyzer<'_> {
    /// Ensures that the all value-returning functions have return statements in all possible execution paths.
    pub(super) fn analyze_func_graph(&mut self, func_id: &FunctionID) {
        // Get the flow graph corresponding to the function
        let Some(flow_graph) = self.func_flow_graphs.get(func_id) else {
            return;
        };

        let guaranteed = self.get_return_guarantee(flow_graph);

        // If the function requires return, and the entry node does not guarantee return,
        // throw an error
        if let Some(func) = self.prog_ctx.func_ctx.get_func(func_id) {
            if !func.return_type.is_void() && !guaranteed[&flow_graph.entry_node()] {
                self.emit_missing_return_error(&func.block.scope_id, &func.name);
            }
        }
    }

    /// Ensures that the all infix operators have return statements in all possible execution paths.
    pub(super) fn analyze_infix_op_graph(&mut self, op_id: &OperatorID) {
        // Get the flow graph corresponding to the function
        let Some(flow_graph) = self.op_flow_graphs.get(op_id) else {
            return;
        };

        let guaranteed = self.get_return_guarantee(flow_graph);

        // If the function requires return, and the entry node does not guarantee return,
        // throw an error
        if let Some(op) = self.prog_ctx.op_ctx.get_infix_func(op_id) {
            if !op.return_type.is_void() && !guaranteed[&flow_graph.entry_node()] {
                self.emit_missing_return_error(&op.block.scope_id, &op.symbol);
            }
        }
    }

    /// Ensures that the all prefix operators have return statements in all possible execution paths.
    pub(super) fn analyze_prefix_op_graph(&mut self, op_id: &OperatorID) {
        // Get the flow graph corresponding to the function
        let Some(flow_graph) = self.op_flow_graphs.get(op_id) else {
            return;
        };

        let guaranteed = self.get_return_guarantee(flow_graph);

        // If the function requires return, and the entry node does not guarantee return,
        // throw an error
        if let Some(op) = self.prog_ctx.op_ctx.get_prefix_func(op_id) {
            if !op.return_type.is_void() && !guaranteed[&flow_graph.entry_node()] {
                self.emit_missing_return_error(&op.block.scope_id, &op.symbol);
            }
        }
    }

    /// Ensures that the all postfix operators have return statements in all possible execution paths.
    pub(super) fn analyze_postfix_op_graph(&mut self, op_id: &OperatorID) {
        // Get the flow graph corresponding to the function
        let Some(flow_graph) = self.op_flow_graphs.get(op_id) else {
            return;
        };

        let guaranteed = self.get_return_guarantee(flow_graph);

        // If the function requires return, and the entry node does not guarantee return,
        // throw an error
        if let Some(op) = self.prog_ctx.op_ctx.get_postfix_func(op_id) {
            if !op.return_type.is_void() && !guaranteed[&flow_graph.entry_node()] {
                self.emit_missing_return_error(&op.block.scope_id, &op.symbol);
            }
        }
    }

    fn emit_missing_return_error(&mut self, scope_id: &ScopeID, func_name: &str) {
        // Get the range of the function's corresponding scope
        if let Some(func_scope) = self.prog_ctx.scope_registry.get_scope(scope_id) {
            self.ec
                .missing_return(func_scope.range, Ph::ScopeGraphAnalyzing);
        } else {
            // If the scope does not exist in the scope registry, throw an compiler bug error
            self.ec.comp_bug(Range::zero(), Ph::FlowGraphAnalyzing, &format!("The function {} does not have a return, but the function scope is not found in the scope registry.", func_name));
        }
    }

    /// Calculate whether the nodes guarantee return.
    fn get_return_guarantee(&mut self, flow_graph: &FlowGraph) -> HashMap<FlowID, bool> {
        // First sort the flow graph topologically
        let sorted_nodes = self.sort_graph(flow_graph);

        // Calculate whether return is guaranteed for every single nodes
        let mut guaranteed: HashMap<FlowID, bool> = HashMap::new();

        for &id in sorted_nodes.iter().rev() {
            let has_return = flow_graph.get_node(&id).unwrap().has_return;
            let succs = flow_graph.get_successors(&id);

            let result = if succs.is_empty() {
                // If the node is the end node, take has_return
                has_return
            } else {
                // If there are successors, get whether all the successors guarantee return
                has_return || succs.iter().all(|s| guaranteed[s])
            };

            guaranteed.insert(id, result);
        }

        guaranteed
    }

    fn sort_graph(&self, flow_graph: &FlowGraph) -> Vec<FlowID> {
        let mut result = Vec::new();
        // Collect the in degrees for each nodes
        let mut in_degrees: HashMap<FlowID, usize> = flow_graph
            .get_all_nodes()
            .iter()
            .map(|node| (*node, flow_graph.get_in_degree(node)))
            .collect();

        // Get the nodes with an in_degree of zero
        let mut zero_queue: VecDeque<FlowID> = in_degrees
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(id, _)| *id)
            .collect();

        // Loop through the nodes and sort
        while let Some(node) = zero_queue.pop_front() {
            result.push(node);

            let successors = flow_graph.get_successors(&node);
            // Decrement the in_degree for each successors
            for successor in &successors {
                let in_degree = in_degrees.get_mut(successor).unwrap();
                *in_degree -= 1;
                if *in_degree == 0 {
                    zero_queue.push_back(*successor);
                }
            }
        }

        result
    }
}
