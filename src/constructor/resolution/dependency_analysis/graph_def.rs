//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::SymbolPath;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DependencyGraph {
    pub nodes: HashMap<SymbolPath, DependencyGraphNode>,
    pub edges: Vec<DependencyGraphEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyGraphNode) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: &SymbolPath, target: &SymbolPath) {
        self.edges
            .push(DependencyGraphEdge::new(from.clone(), target.clone()));
    }

    pub fn node_paths(&self) -> Vec<&SymbolPath> {
        self.nodes.keys().collect()
    }

    pub fn edges(&self) -> Vec<&DependencyGraphEdge> {
        self.edges.iter().collect()
    }

    pub fn node(&self, name: &SymbolPath) -> Option<&DependencyGraphNode> {
        self.nodes.get(name)
    }

    pub fn edge(&self, from: &SymbolPath, target: &SymbolPath) -> Option<&DependencyGraphEdge> {
        self.edges
            .iter()
            .find(|edge| edge.from == *from && edge.target == *target)
    }

    pub fn get_edge_nodes(
        &self,
        edge: &DependencyGraphEdge,
    ) -> Option<(&DependencyGraphNode, &DependencyGraphNode)> {
        self.node(&edge.from)
            .and_then(|from| self.node(&edge.target).map(|target| (from, target)))
    }

    pub fn get_edges_from_node(&self, path: &SymbolPath) -> Vec<&DependencyGraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.from == *path)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DependencyGraphNode {
    name: SymbolPath,
}

impl DependencyGraphNode {
    pub fn new(name: SymbolPath) -> Self {
        DependencyGraphNode { name }
    }
}

impl Clone for DependencyGraphNode {
    fn clone(&self) -> Self {
        DependencyGraphNode {
            name: self.name.clone(),
        }
    }
}

/// Represents an edge in a dependency graph.
///
/// # Example
/// Edge `A -> B` means that "A depends on B", therefore B must be resolved before A.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DependencyGraphEdge {
    pub from: SymbolPath,
    pub target: SymbolPath,
}

impl DependencyGraphEdge {
    pub fn new(from: SymbolPath, target: SymbolPath) -> Self {
        DependencyGraphEdge { from, target }
    }
}
