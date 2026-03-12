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

use kasl::{
    CompilationState, NameSpace, ParserDeclStmt,
    error::{ErrorCollector, ErrorRecord},
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_collection::TypeCollector,
};

pub fn parse_expr(input: &str) -> Vec<ParserDeclStmt> {
    kasl_parser::parse(input).unwrap()
}

pub fn collect_types(
    ec: &mut ErrorCollector,
    name_space: &mut NameSpace,
    comp_state: &mut CompilationState,
    statements: &[ParserDeclStmt],
) -> Result<(), Vec<ErrorRecord>> {
    let mut type_collector = TypeCollector::new(ec, statements, name_space, comp_state);
    type_collector.process();
    ec.as_result()
}

pub fn collect_global_decls(
    ec: &mut ErrorCollector,
    name_space: &mut NameSpace,
    func_body_map: &mut FuncBodyMap,
    op_body_map: &mut OpBodyMap,
    comp_state: &mut CompilationState,
    statements: &[ParserDeclStmt],
) -> Result<(), Vec<ErrorRecord>> {
    let mut global_decl_collector = GlobalDeclCollector::new(
        ec,
        statements,
        name_space,
        func_body_map,
        op_body_map,
        comp_state,
    );
    global_decl_collector.process();
    ec.as_result()
}
