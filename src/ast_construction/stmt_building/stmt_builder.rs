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
    Program, SymbolTable,
    data::SymbolID,
    error::{ErrorCollector, Ph},
    stmt_building::{StmtBuildingCtx, function_graph::FunctionGraph},
};
use std::collections::{HashMap, VecDeque};

pub fn build_statements(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
) {
    // Create a building context'
    let mut ctx = StmtBuildingCtx::new(ec, program, symbol_table);

    // Build the function bodies
    ctx.build_func_bodies();

    // Check if there's any cycles
    let cyclic_nodes = detect_cycles(&ctx.function_graph);
    if !cyclic_nodes.is_empty() {
        for func_id in cyclic_nodes {
            if let Some(func_path) = program.get_path_by_id(&func_id)
                && let Some(func_stmt) = symbol_table.get_id_by_path(func_path).and_then(|ids| {
                    ids.first()
                        .copied()
                        .and_then(|id| symbol_table.get_statement_by_id(&id))
                })
            {
                ec.recursive_func(
                    func_stmt.range,
                    Ph::StatementBuilding,
                    &func_path.to_string(),
                );
            }
        }
    }
}

// Gets the cyclic nodes using Kahn's algorithm.
fn detect_cycles(graph: &FunctionGraph) -> Vec<SymbolID> {
    let mut in_degrees: HashMap<SymbolID, u32> = HashMap::new();

    // Initialize in-degrees for all nodes
    for node in &graph.nodes {
        in_degrees.entry(*node).or_insert(0);
    }

    // Calculate in-degrees for each node
    for edges in graph.edges.values() {
        for edge in edges {
            *in_degrees.entry(*edge).or_default() += 1;
        }
    }

    // Push the nodes with in-degree 0 into the queue
    let mut queue: VecDeque<SymbolID> = VecDeque::new();
    for (node, degree) in in_degrees.iter() {
        if *degree == 0 {
            queue.push_back(*node);
        }
    }

    // Process the queue
    while let Some(node) = queue.pop_front() {
        // Get outgoing edges from the current node
        if let Some(edges) = graph.edges.get(&node) {
            for edge in edges {
                *in_degrees.get_mut(edge).unwrap() -= 1;
                if in_degrees[edge] == 0 {
                    queue.push_back(*edge);
                }
            }
        }
    }

    // Get the cyclic nodes
    let cyclic_nodes: Vec<SymbolID> = in_degrees
        .iter()
        .filter(|(_, degree)| **degree > 0)
        .map(|(node, _)| *node)
        .collect();

    cyclic_nodes
}
