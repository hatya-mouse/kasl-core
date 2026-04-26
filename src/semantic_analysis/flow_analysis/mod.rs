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

mod graph_analyzer;

use crate::{
    ast_nodes::{FunctionID, OperatorID, compilation_data::ProgramContext, flow_graph::FlowGraph},
    error::ErrorCollector,
};
use std::collections::HashMap;

pub struct FlowGraphAnalyzer<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a ProgramContext,
    func_flow_graphs: &'a HashMap<FunctionID, FlowGraph>,
    op_flow_graphs: &'a HashMap<OperatorID, FlowGraph>,
}

impl<'a> FlowGraphAnalyzer<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a ProgramContext,
        func_flow_graphs: &'a HashMap<FunctionID, FlowGraph>,
        op_flow_graphs: &'a HashMap<OperatorID, FlowGraph>,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            func_flow_graphs,
            op_flow_graphs,
        }
    }

    pub fn analyze_all(&mut self) {
        // Analyze the graph for each functions
        for func in &self.prog_ctx.func_ctx.get_all_func_ids() {
            self.analyze_func_graph(func);
        }

        // Analyze the graph for each operators
        for op in &self.prog_ctx.op_ctx.all_infix_ids() {
            self.analyze_infix_op_graph(op);
        }

        for op in &self.prog_ctx.op_ctx.all_prefix_ids() {
            self.analyze_prefix_op_graph(op);
        }

        for op in &self.prog_ctx.op_ctx.all_postfix_ids() {
            self.analyze_postfix_op_graph(op);
        }
    }
}
