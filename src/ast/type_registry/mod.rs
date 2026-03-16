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

mod primitive_type;
mod resolved_type;
mod struct_decl;
mod struct_field;
mod struct_graph;

pub use primitive_type::PrimitiveType;
pub use resolved_type::ResolvedType;
pub use struct_decl::StructDecl;
pub use struct_field::StructField;
pub use struct_graph::StructGraph;

use crate::{NameSpaceID, StructID};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, serde::Serialize)]
pub struct TypeRegistry {
    pub structs: HashMap<StructID, StructDecl>,
    pub name_to_id: HashMap<(NameSpaceID, String), StructID>,
    next_struct_id: usize,
}

impl TypeRegistry {
    pub fn generate_struct_id(&mut self) -> StructID {
        let id = StructID::new(self.next_struct_id);
        self.next_struct_id += 1;
        id
    }

    // --- TYPE RESOLUTION ---

    pub fn resolve_type(&self, namespace_id: NameSpaceID, type_name: &str) -> Option<ResolvedType> {
        match PrimitiveType::from_str(type_name) {
            Ok(primitive) => Some(ResolvedType::Primitive(primitive)),
            Err(_) => self
                .get_struct_id(namespace_id, type_name)
                .map(|id| ResolvedType::Struct(id)),
        }
    }

    // --- GETTER FUNCTIONS ---

    pub fn get_struct_id(&self, namespace_id: NameSpaceID, type_name: &str) -> Option<StructID> {
        self.name_to_id
            .get(&(namespace_id, type_name.to_string()))
            .copied()
    }

    pub fn get_struct(&self, id: &StructID) -> Option<&StructDecl> {
        self.structs.get(id)
    }

    pub fn get_all_structs(&self) -> Vec<StructID> {
        self.structs.keys().copied().collect()
    }

    // --- TYPE SIZE AND ALIGNMENT ---

    pub fn get_type_size(&self, type_id: &ResolvedType) -> usize {
        match type_id {
            ResolvedType::Primitive(ty) => ty.size(),
            ResolvedType::Struct(_) => size_of::<usize>(),
        }
    }

    pub fn get_type_alignment(&self, type_id: &ResolvedType) -> u8 {
        match type_id {
            ResolvedType::Primitive(ty) => ty.alignment(),
            ResolvedType::Struct(_) => 4,
        }
    }

    // --- REGISTRATION ---

    pub fn register_struct(
        &mut self,
        namespace_id: NameSpaceID,
        struct_decl: StructDecl,
        name: String,
        struct_id: StructID,
    ) {
        self.structs.insert(struct_id, struct_decl);
        self.name_to_id.insert((namespace_id, name), struct_id);
    }

    // --- FORMATTING ---

    pub fn format_type(&self, ty: &ResolvedType) -> String {
        match ty {
            ResolvedType::Primitive(ty) => ty.to_string(),
            ResolvedType::Struct(id) => self
                .get_struct(id)
                .map(|s| s.name.clone())
                .unwrap_or(format!("struct(ID: {})", id)),
        }
    }
}
