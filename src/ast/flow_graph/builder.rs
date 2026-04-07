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

use crate::ast::flow_graph::{FlowGraph, FlowID};

pub struct FlowGraphBuilder {
    building_graph: FlowGraph,
    current_node: FlowID,
}

impl FlowGraphBuilder {
    /// Initializes a FlowGraphBuilder with a single entry point node.
    pub fn with_entry_node() -> Self {
        let flow_graph = FlowGraph::with_entry_node();
        Self {
            current_node: flow_graph.entry_node(),
            building_graph: flow_graph,
        }
    }

    /// Moves out the built graph from the builder.
    pub fn take_graph(self) -> FlowGraph {
        self.building_graph
    }

    /// Adds a new node to the graph with has_return set to false.
    pub fn new_node(&mut self) -> FlowID {
        self.building_graph.add_flow_node(false)
    }

    /// Switches the current node to the given node.
    pub fn switch_node(&mut self, node: FlowID) {
        self.current_node = node;
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, from: FlowID, to: FlowID) {
        self.building_graph.add_flow_edge(from, to);
    }

    /// Returns the current node ID.
    pub fn current_node(&self) -> FlowID {
        self.current_node
    }

    /// Sets the has_return of the current node to true.
    pub fn current_has_return(&mut self) {
        self.building_graph.set_has_return(&self.current_node, true);
    }
}
