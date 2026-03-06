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
    ParserFuncParam, PrefixOperator, Range, SymbolPath, error::Ph,
    resolution::type_resolve_ctx::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_prefix_operator(
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

        // Check if the function has one parameter
        if params.len() != 1 {
            self.ec
                .invalid_param_numbers_for_prefix(decl_range, Ph::TypeResolution, params.len());
            return;
        }

        // Resolve the operand
        let operand = match self.resolve_param(&params[0]) {
            Some(operand) => operand,
            None => return,
        };

        // Construct the prefix operator
        let prefix = PrefixOperator {
            symbol: symbol.to_string(),
            operand,
            return_type: resolved_return_type,
            body: Vec::new(),
        };
        self.program.register_prefix_func(prefix);
    }
}
