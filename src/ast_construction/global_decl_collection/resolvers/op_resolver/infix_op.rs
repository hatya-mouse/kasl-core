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
    FuncParam, InfixOperator, InfixOperatorProperties, ParserScopeStmt, Range, error::Ph,
    global_decl_collection::GlobalDeclCollector, symbol_table::Block, type_registry::ResolvedType,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_infix_define(&mut self, symbol: &str, props: &InfixOperatorProperties) {
        self.namespace
            .op_ctx
            .register_infix_properties(symbol.to_string(), props.clone());
    }

    pub fn register_infix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 2 {
            self.ec.wrong_param_count_for_infix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        // Construct infix operator
        let op = InfixOperator {
            symbol: symbol.to_string(),
            lhs: params[0].clone(),
            rhs: params[1].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let op_id = self.namespace.op_ctx.register_infix_func(op);

        // Register the function body to the function body map
        self.comp_data.op_body_map.register(op_id, body.to_vec());
    }
}
