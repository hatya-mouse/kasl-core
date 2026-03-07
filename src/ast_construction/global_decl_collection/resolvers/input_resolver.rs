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
    ExprToken, ParserInputAttribute, Range, ScopeVar, SymbolPath,
    global_decl_collection::GlobalDeclCollector,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_input(
        &mut self,
        name: &str,
        value_type: &Option<SymbolPath>,
        def_val: &Vec<ExprToken>,
        attrs: &Vec<ParserInputAttribute>,
        decl_range: Range,
    ) {
        let global_scope = self.scope_registry.get_global_scope_mut();
        let var = ScopeVar {
            name: name.to_string(),
            value_type: (),
            def_val: (),
            range: (),
            var_kind: (),
        };
        global_scope.register_var(name, var, id);
    }
}
