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

use crate::{
    ExprToken, ExprTokenKind, SymbolPath, SymbolTable,
    resolution::dependency_analysis::DependencyGraph,
};

pub fn build_var_graph(
    graph: &mut DependencyGraph,
    root_symbol_table: &SymbolTable,
    var_path: SymbolPath,
    def_val: &Vec<ExprToken>,
) {
    // If the default value has any identifiers, thus the variable depends on them
    for expr in def_val {
        match &expr.kind {
            ExprTokenKind::Identifier(path) => {
                let to_path = root_symbol_table.resolve_path(path);
                graph.add_edge(&var_path, &to_path);
            }
            _ => (),
        }
    }
}
