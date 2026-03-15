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
    OperatorContext, ScopeRegistry,
    name_space::NameSpace,
    symbol_table::{FuncBodyMap, FunctionContext, OpBodyMap},
    type_registry::{StructGraph, TypeRegistry},
};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ProgramContext {
    pub func_ctx: FunctionContext,
    pub op_ctx: OperatorContext,
    pub scope_registry: ScopeRegistry,
    pub type_registry: TypeRegistry,
    pub name_spaces: Vec<NameSpace>,
}

#[derive(Debug, Default)]
pub struct CompilationState {
    pub func_body_map: FuncBodyMap,
    pub op_body_map: OpBodyMap,
    pub struct_graph: StructGraph,
}

#[derive(Debug, Default)]
pub struct CompilerConfig {
    pub search_paths: Vec<PathBuf>,
}
