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

use std::collections::{HashMap, VecDeque};

use crate::{
    Range, SymbolTable,
    data::ParserStmtID,
    error::{ErrorCollector, Phase},
    resolution::dependency_analysis::DependencyGraph,
};

pub fn sort_graph(
    ec: &mut ErrorCollector,
    symbol_table: &SymbolTable,
    graph: &DependencyGraph,
) -> Option<Vec<ParserStmtID>> {
    // Calculate the in degree of each node
    let mut in_degrees = HashMap::new();

    for node in graph.node_paths() {
        in_degrees.insert(node, 0);
    }

    for edge in graph.edges() {
        *in_degrees.get_mut(&edge.target).unwrap() += 1;
    }

    // Initialize the queue with nodes that have no incoming edges
    let mut queue = VecDeque::new();
    for node in graph.node_paths() {
        if in_degrees[&node] == 0 {
            queue.push_back(node);
        }
    }

    // Perform topological sorting
    let mut sorted_nodes = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted_nodes.push(node);
        for edge in graph.get_edges_from_node(&node) {
            *in_degrees.get_mut(&edge.target).unwrap() -= 1;
            if in_degrees[&edge.target] == 0 {
                in_degrees.remove(&edge.target);
                queue.push_back(edge.target);
            }
        }
    }

    // Drain the remaining nodes that have no incoming edges
    while let Some(node) = queue.pop_front() {
        sorted_nodes.insert(0, node);
    }

    // Check if the graph is acyclic
    if sorted_nodes.len() != graph.node_paths().len() {
        let cyclic_nodes = in_degrees.keys().cloned().collect();
        generate_cyclic_errors(ec, symbol_table, cyclic_nodes);
        None
    } else {
        Some(sorted_nodes.to_vec())
    }
}

pub fn generate_cyclic_errors(
    ec: &mut ErrorCollector,
    symbol_table: &SymbolTable,
    cyclic_nodes: Vec<ParserStmtID>,
) {
    for symbol_id in cyclic_nodes {
        if let Some(stmt) = symbol_table.get_statement_by_id(&symbol_id) {
            // And get the range in which the statement is declared
            let symbol_path = match symbol_table.get_path_by_id(&symbol_id) {
                Some(path) => path.to_string(),
                None => {
                    ec.comp_bug(
                        stmt.range,
                        Phase::GraphConstruction,
                        "generate_cyclic_errors received a SymbolID that doesn't exist in the SymbolTable",
                    );
                    return;
                }
            };

            ec.dep_cycle(stmt.range, Phase::GraphConstruction, &symbol_path);
        } else {
            let symbol_path = match symbol_table.get_path_by_id(&symbol_id) {
                Some(path) => path.to_string(),
                None => {
                    ec.comp_bug(
                        Range::zero(),
                        Phase::GraphConstruction,
                        "generate_cyclic_errors received a SymbolID that doesn't exist in the SymbolTable",
                    );
                    return;
                }
            };

            ec.dep_cycle(
                Range::zero(),
                Phase::GraphConstruction,
                &symbol_path.to_string(),
            );
        }
    }
}
