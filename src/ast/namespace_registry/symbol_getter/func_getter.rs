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
    Function, FunctionID, NameSpaceID, StructID,
    namespace_registry::{NameSpacePair, NameSpaceRegistry},
};

pub trait NameSpaceFuncGetter {
    fn get_member_func_id(
        &self,
        struct_id: &NameSpacePair<StructID>,
        method_name: &str,
    ) -> Option<NameSpacePair<FunctionID>>;

    fn get_func_id(
        &self,
        namespace_id: &NameSpaceID,
        func_name: &str,
    ) -> Option<NameSpacePair<FunctionID>>;

    fn get_func(&self, func_id: &NameSpacePair<FunctionID>) -> Option<&Function>;
}

impl NameSpaceFuncGetter for NameSpaceRegistry {
    // --- MEMBER FUNC ---

    fn get_member_func_id(
        &self,
        struct_id: &NameSpacePair<StructID>,
        method_name: &str,
    ) -> Option<NameSpacePair<FunctionID>> {
        let namespace = self.get_namespace_by_id(&struct_id.namespace_id)?;
        namespace
            .func_ctx
            .get_member_func_by_name(&struct_id.symbol_id, method_name)
            .map(|func_id| NameSpacePair {
                namespace_id: struct_id.namespace_id,
                symbol_id: func_id,
            })
    }

    // --- FUNC ---

    fn get_func_id(
        &self,
        namespace_id: &NameSpaceID,
        func_name: &str,
    ) -> Option<NameSpacePair<FunctionID>> {
        let namespace = self.get_namespace_by_id(namespace_id)?;
        namespace
            .func_ctx
            .get_global_func_by_name(func_name)
            .map(|func_id| NameSpacePair {
                namespace_id: *namespace_id,
                symbol_id: func_id,
            })
    }

    fn get_func(&self, func_id: &NameSpacePair<FunctionID>) -> Option<&Function> {
        let namespace = self.get_namespace_by_id(&func_id.namespace_id)?;
        namespace.func_ctx.get_func(&func_id.symbol_id)
    }
}
