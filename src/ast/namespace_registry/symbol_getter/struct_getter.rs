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

use std::str::FromStr;

use crate::{
    NameSpaceID, StructID,
    namespace_registry::{NameSpacePair, NameSpaceRegistry},
    type_registry::{PrimitiveType, ResolvedType, StructDecl},
};

pub trait NameSpaceStructGetter {
    fn resolve_type_path(
        &self,
        namespace_id: &NameSpaceID,
        type_name: &str,
    ) -> Option<ResolvedType>;

    fn get_struct_id(
        &self,
        namespace_id: &NameSpaceID,
        name: &str,
    ) -> Option<NameSpacePair<StructID>>;

    fn get_struct(&self, id: &NameSpacePair<StructID>) -> Option<&StructDecl>;
}

impl NameSpaceStructGetter for NameSpaceRegistry {
    // --- TYPE RESOLUTION ---

    fn resolve_type_path(
        &self,
        namespace_id: &NameSpaceID,
        type_name: &str,
    ) -> Option<ResolvedType> {
        if let Ok(primitive_type) = PrimitiveType::from_str(type_name) {
            return Some(ResolvedType::Primitive(primitive_type));
        }
        let id = self.get_struct_id(namespace_id, type_name)?;
        Some(ResolvedType::Struct(id))
    }

    // --- STRUCT ---

    fn get_struct_id(
        &self,
        namespace_id: &NameSpaceID,
        name: &str,
    ) -> Option<NameSpacePair<StructID>> {
        let namespace = self.get_namespace_by_id(namespace_id)?;
        namespace
            .type_registry
            .get_struct_id_by_name(name)
            .map(|struct_id| NameSpacePair {
                namespace_id: *namespace_id,
                symbol_id: struct_id,
            })
    }

    fn get_struct(&self, id: &NameSpacePair<StructID>) -> Option<&StructDecl> {
        let namespace = self.get_namespace_by_id(&id.namespace_id)?;
        namespace.type_registry.get_struct(&id.symbol_id)
    }
}
