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

use crate::data::ParserStmtID;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DependencyGraph {
    pub nodes: HashMap<ParserStmtID, DependencyGraphNode>,
    pub edges: Vec<DependencyGraphEdge>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyGraphNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, from: ParserStmtID, target: ParserStmtID) {
        self.edges.push(DependencyGraphEdge::new(from, target));
    }

    pub fn node_paths(&self) -> Vec<ParserStmtID> {
        self.nodes.keys().cloned().collect()
    }

    pub fn edges(&self) -> Vec<&DependencyGraphEdge> {
        self.edges.iter().collect()
    }

    pub fn node(&self, id: &ParserStmtID) -> Option<&DependencyGraphNode> {
        self.nodes.get(id)
    }

    pub fn get_edges_from_node(&self, path: &ParserStmtID) -> Vec<&DependencyGraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.from == *path)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DependencyGraphNode {
    id: ParserStmtID,
}

impl DependencyGraphNode {
    pub fn new(id: ParserStmtID) -> Self {
        DependencyGraphNode { id }
    }
}

impl Clone for DependencyGraphNode {
    fn clone(&self) -> Self {
        DependencyGraphNode { id: self.id }
    }
}

/// Represents an edge in a dependency graph.
///
/// # Example
/// Edge `A -> B` means that "A depends on B", therefore B must be resolved before A.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DependencyGraphEdge {
    pub from: ParserStmtID,
    pub target: ParserStmtID,
}

impl DependencyGraphEdge {
    pub fn new(from: ParserStmtID, target: ParserStmtID) -> Self {
        DependencyGraphEdge { from, target }
    }
}
