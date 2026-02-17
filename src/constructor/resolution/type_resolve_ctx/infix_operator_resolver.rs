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
    InfixOperator, InfixOperatorProperties, ParserFuncParam, ParserSymbolPath, Range, error::Phase,
    resolution::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_infix_func(
        &mut self,
        symbol: &str,
        params: &[ParserFuncParam],
        return_type: &ParserSymbolPath,
        decl_range: Range,
    ) {
        // Get the return type
        let return_type_path = match self.program.resolve_type_def_parser_path(return_type) {
            Some(path) => path,
            None => {
                self.ec
                    .type_not_found(decl_range, Phase::TypeResolution, &return_type.to_string());
                return;
            }
        };

        let lhs = match self.resolve_param(&params[0]) {
            Some(operand) => operand,
            None => return,
        };
        let rhs = match self.resolve_param(&params[1]) {
            Some(operand) => operand,
            None => return,
        };

        // Once we've got the types, we can get the exact operator
        let infix = InfixOperator {
            symbol: symbol.to_string(),
            lhs,
            rhs,
            return_type: return_type_path,
            body: Vec::new(),
        };
        self.program.register_infix_func(infix);
    }

    pub fn register_infix_define(&mut self, symbol: &str, properties: InfixOperatorProperties) {
        self.program
            .register_infix_operator(symbol.to_string(), properties);
    }
}
