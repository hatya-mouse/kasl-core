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

mod infix_op;
mod postfix_op;
mod prefix_op;

use crate::{
    ParserFuncParam, ParserOperatorType, ParserScopeStmt, Range, SymbolPath, error::Ph,
    global_decl_collection::GlobalDeclCollector, symbol_table::Block,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_operator_func(
        &mut self,
        op_type: &ParserOperatorType,
        symbol: &str,
        params: &[ParserFuncParam],
        return_type: &SymbolPath,
        body: &[ParserScopeStmt],
        decl_range: Range,
    ) {
        // Create a new scope and a block for the function
        let global_scope_id = self.namespace.scope_registry.get_global_scope_id();
        let op_scope_id = self
            .namespace
            .scope_registry
            .create_scope(Some(global_scope_id), decl_range);
        let op_block = Block::new(op_scope_id);

        // Resolve the function parameters
        let Some(params) = self.resolve_func_params(params, op_scope_id) else {
            return;
        };

        // Resolve the return type
        let Some(return_type) = self.namespace.type_registry.resolve_type_path(return_type) else {
            self.ec.type_not_found(
                decl_range,
                Ph::GlobalDeclCollection,
                return_type.to_string(),
            );
            return;
        };

        match op_type {
            ParserOperatorType::Infix => {
                self.register_infix_func(symbol, params, return_type, body, op_block, decl_range)
            }
            ParserOperatorType::Prefix => {
                self.register_prefix_func(symbol, params, return_type, body, op_block, decl_range)
            }
            ParserOperatorType::Postfix => {
                self.register_postfix_func(symbol, params, return_type, body, op_block, decl_range)
            }
        }
    }
}
