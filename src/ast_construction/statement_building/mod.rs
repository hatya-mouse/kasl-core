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
    ParserDeclStmt,
    symbol_table::{FuncBodyMap, FunctionContext},
};

pub struct StatementBuilder<'a> {
    func_ctx: &'a mut FunctionContext,
    func_body_map: &'a FuncBodyMap<'a>,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(func_ctx: &'a mut FunctionContext, func_body_map: &'a FuncBodyMap<'a>) -> Self {
        Self {
            func_ctx,
            func_body_map,
        }
    }

    pub fn collect(&mut self, stmts: &mut Vec<ParserDeclStmt>) {
        for func in self.func_ctx.funcs_mut() {}
    }
}
