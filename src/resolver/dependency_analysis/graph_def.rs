//
// Copyright 2025 Shuntaro Kasatani
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
pub struct DependencyGraphEdge {
    pub from: DependencyGraphNode,
    pub to: DependencyGraphNode,
}

impl DependencyGraphEdge {
    pub fn new(from: DependencyGraphNode, to: DependencyGraphNode) -> Self {
        DependencyGraphEdge { from, to }
    }
}
