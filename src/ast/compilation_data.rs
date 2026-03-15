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
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_registry::StructGraph,
};
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, Default)]
pub struct CompilationData {
    pub func_body_map: FuncBodyMap,
    pub op_body_map: OpBodyMap,
    pub struct_graph: StructGraph,
}

#[derive(Debug, Default)]
pub struct ConstructorState {
    pub imported_paths: HashSet<PathBuf>,
}

#[derive(Debug, Default, Clone)]
pub struct CompilerConfig {
    pub search_paths: Vec<PathBuf>,
}
