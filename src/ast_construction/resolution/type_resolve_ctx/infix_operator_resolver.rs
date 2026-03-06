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
    InfixOperator, InfixOperatorProperties, ParserFuncParam, Range, SymbolPath, error::Ph,
    resolution::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_infix_func(
        &mut self,
        symbol: &str,
        params: &[ParserFuncParam],
        return_type: &SymbolPath,
        decl_range: Range,
    ) {
        // Get the return type id
        let resolved_return_type = match self
            .program
            .get_id_by_path(return_type)
            .and_then(|ids| ids.first().cloned())
        {
            Some(resolved_path) => resolved_path,
            None => {
                self.ec
                    .type_not_found(decl_range, Ph::TypeResolution, &return_type.to_string());
                return;
            }
        };

        // Check if the function has two parameters
        if params.len() != 2 {
            self.ec
                .invalid_param_numbers_for_infix(decl_range, Ph::TypeResolution, params.len());
            return;
        }

        // Resolve the parameters
        let lhs = match self.resolve_param(&params[0]) {
            Some(operand) => operand,
            None => return,
        };
        let rhs = match self.resolve_param(&params[1]) {
            Some(operand) => operand,
            None => return,
        };

        // Ensure that the parameters don't have any default value
        if lhs.def_val.is_some() {
            self.ec
                .op_def_val(decl_range, Ph::TypeResolution, &lhs.name);
        }
        if rhs.def_val.is_some() {
            self.ec
                .op_def_val(decl_range, Ph::TypeResolution, &rhs.name);
        }

        // Once we've got the types, we can get the exact operator
        let infix = InfixOperator {
            symbol: symbol.to_string(),
            lhs,
            rhs,
            return_type: resolved_return_type,
            body: Vec::new(),
        };
        self.program.register_infix_func(infix);
    }

    pub fn register_infix_define(&mut self, symbol: &str, properties: InfixOperatorProperties) {
        self.program.register_infix_operator(symbol, properties);
    }
}
