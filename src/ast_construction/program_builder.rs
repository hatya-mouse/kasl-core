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
    NameSpace, OperatorContext, ParserDeclStmt, ScopeRegistry,
    error::{ErrorCollector, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    symbol_table::FunctionContext,
    type_collection::TypeCollector,
    type_registry::TypeRegistry,
};

pub fn construct_program(statements: Vec<ParserDeclStmt>) -> Result<(), Vec<ErrorRecord>> {
    let mut ec = ErrorCollector::new();
    let mut name_space = NameSpace::new();
    let mut func_ctx = FunctionContext::new();
    let mut op_ctx = OperatorContext::new();
    let mut scope_registry = ScopeRegistry::new();
    let mut type_registry = TypeRegistry::new();

    // 1. Collect types
    let mut type_collector = TypeCollector {
        decl_stmts: &statements,
        name_space: &mut name_space,
        type_registry: &mut type_registry,
    };
    type_collector.process();

    // 2. Collect global declarations, such as inputs, outputs, states, struct fields and functions
    let mut global_decl_collector = GlobalDeclCollector {
        ec: &mut ec,
        decl_stmts: &statements,
        name_space: &mut name_space,
        type_registry: &mut type_registry,
        func_ctx: &mut func_ctx,
        op_ctx: &mut op_ctx,
        scope_registry: &mut scope_registry,
    };
    global_decl_collector.process();

    ec.as_result()
}
