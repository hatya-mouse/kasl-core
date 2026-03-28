//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast::{
        FuncParam, PrefixOperator, PrefixOperatorProperties, Range, symbol_table::Block,
        type_registry::ResolvedType,
    },
    ast_construction::global_decl_collection::GlobalDeclCollector,
    error::{EK, Ph},
    parser::ParserScopeStmt,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_prefix_define(&mut self, symbol: &str, props: &PrefixOperatorProperties) {
        if let Err(error_kind) = self
            .prog_ctx
            .op_ctx
            .register_prefix_properties(symbol.to_string(), props.clone())
            && error_kind == EK::DuplicatePrefixDefine
        {
            self.ec
                .duplicate_prefix_define(props.range, Ph::GlobalDeclCollection, symbol);
        }
    }

    pub fn register_prefix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 1 {
            self.ec.wrong_param_count_for_prefix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        // Construct prefix operator
        let op = PrefixOperator {
            symbol: symbol.to_string(),
            namespace_id: self.current_namespace,
            operand: params[0].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let Ok(op_id) = self.prog_ctx.op_ctx.register_prefix_func(op) else {
            self.ec.duplicate_prefix_func(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                self.prog_ctx
                    .type_registry
                    .format_type(&params[0].value_type),
            );
            return;
        };

        // Register the function body to the function body map
        self.comp_data.op_body_map.register(op_id, body.to_vec());
    }
}
