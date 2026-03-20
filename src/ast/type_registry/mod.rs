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

mod array_decl;
mod primitive_type;
mod resolved_type;
mod struct_decl;
mod struct_graph;

pub use array_decl::ArrayDecl;
pub use primitive_type::PrimitiveType;
pub use resolved_type::ResolvedType;
pub use struct_decl::{StructDecl, StructField};
pub use struct_graph::StructGraph;

use crate::{NameSpaceID, StructID, namespace_registry::ArrayID};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, serde::Serialize)]
pub struct TypeRegistry {
    // Struct Registration
    structs: HashMap<StructID, StructDecl>,
    name_to_id: HashMap<(NameSpaceID, String), StructID>,

    // Array Registration
    array_id_to_decl: HashMap<ArrayID, ArrayDecl>,
    decl_to_array_id: HashMap<ArrayDecl, ArrayID>,

    // ID Generation
    next_struct_id: usize,
    next_array_id: usize,
}

impl TypeRegistry {
    pub fn generate_struct_id(&mut self) -> StructID {
        let id = StructID(self.next_struct_id);
        self.next_struct_id += 1;
        id
    }

    pub fn generate_array_id(&mut self) -> ArrayID {
        let id = ArrayID(self.next_array_id);
        self.next_array_id += 1;
        id
    }

    // --- TYPE RESOLUTION ---

    pub fn resolve_type_name(
        &self,
        namespace_id: NameSpaceID,
        type_name: &str,
    ) -> Option<ResolvedType> {
        match PrimitiveType::from_str(type_name) {
            Ok(primitive) => Some(ResolvedType::Primitive(primitive)),
            Err(_) => self
                .get_struct_id(namespace_id, type_name)
                .map(ResolvedType::Struct),
        }
    }

    pub fn register_or_get_array(&mut self, elem_type: ResolvedType, count: usize) -> ArrayID {
        let array_decl = ArrayDecl::new(elem_type, count);

        // Add or get the array id
        if self.decl_to_array_id.contains_key(&array_decl) {
            *self.decl_to_array_id.get(&array_decl).unwrap()
        } else {
            let id = self.generate_array_id();
            self.decl_to_array_id.insert(array_decl.clone(), id);
            self.array_id_to_decl.insert(id, array_decl);
            id
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

    pub fn get_array_info(&self, id: &ArrayID) -> Option<&ArrayDecl> {
        self.array_id_to_decl.get(id)
    }

    // --- TYPE SIZE AND ALIGNMENT ---

    pub fn get_type_actual_size(&self, type_id: &ResolvedType) -> Option<usize> {
        match type_id {
            ResolvedType::Primitive(ty) => Some(ty.size()),
            ResolvedType::Struct(struct_id) => {
                self.get_struct(struct_id).map(|s| s.total_size as usize)
            }
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
            ResolvedType::Primitive(prim_type) => prim_type.to_string(),
            ResolvedType::Struct(id) => self
                .get_struct(id)
                .map(|s| s.name.clone())
                .unwrap_or(format!("struct(ID: {})", id.0)),
        }
    }
}
