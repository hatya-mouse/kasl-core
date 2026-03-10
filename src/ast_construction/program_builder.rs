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
    CompilationState, NameSpace, ParserDeclStmt,
    error::{ErrorCollector, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    symbol_table::FuncBodyMap,
    type_collection::TypeCollector,
};

pub fn construct_program(statements: Vec<ParserDeclStmt>) -> Result<(), Vec<ErrorRecord>> {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::default();
    let mut func_body_map = FuncBodyMap::default();
    let mut compilation_state = CompilationState::default();

    // 1. Collect types
    let mut type_collector = TypeCollector::new(
        &mut ec,
        &statements,
        &mut name_space,
        &mut compilation_state,
    );
    type_collector.process();

    // 2. Collect global declarations, such as inputs, outputs, states, struct fields and functions
    let mut global_decl_collector = GlobalDeclCollector::new(
        &mut ec,
        &statements,
        &mut name_space,
        &mut func_body_map,
        &mut compilation_state,
    );
    global_decl_collector.process();

    ec.as_result()
}
