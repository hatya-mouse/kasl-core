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
    ast_nodes::{
        FuncParam, PostfixOperator, PostfixOperatorProperties, Range, symbol_table::Block,
        type_registry::ResolvedType,
    },
    error::{EK, Ph},
    parser::ParserScopeStmt,
    semantic_analysis::decl_collection::GlobalDeclCollector,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_postfix_define(&mut self, symbol: &str, props: &PostfixOperatorProperties) {
        if let Err(error_kind) = self
            .prog_ctx
            .op_ctx
            .register_postfix_properties(symbol.to_string(), props.clone())
            && error_kind == EK::DuplicatePostfixDefine
        {
            self.ec
                .duplicate_postfix_define(props.range, Ph::GlobalDeclCollection, symbol);
        }
    }

    pub fn register_postfix_func(
        &mut self,
        symbol: &str,
        params: Vec<FuncParam>,
        return_type: ResolvedType,
        body: &[ParserScopeStmt],
        block: Block,
        decl_range: Range,
    ) {
        if params.len() != 1 {
            self.ec.wrong_param_count_for_postfix(
                decl_range,
                Ph::GlobalDeclCollection,
                symbol,
                params.len(),
            );
            return;
        }

        for param in params.iter() {
            // Throw a warning if label is set for parameters in operator function
            if param.label.is_some() {
                self.ec
                    .label_for_op_param(decl_range, Ph::GlobalDeclCollection);
            }

            // Throw a warning if there's a default for the parameters in operator function
            if param.def_val.is_some() {
                self.ec
                    .def_val_for_op_param(decl_range, Ph::GlobalDeclCollection);
            }
        }

        // Construct postfix operator
        let op = PostfixOperator {
            symbol: symbol.to_string(),
            namespace_id: self.current_namespace,
            operand: params[0].clone(),
            return_type,
            block,
            range: decl_range,
        };

        // Register the operator
        let Ok(op_id) = self.prog_ctx.op_ctx.register_postfix_func(op) else {
            self.ec.duplicate_postfix_func(
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
