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

use crate::ast_nodes::{NameSpaceID, StructID, namespace_registry::ArrayID};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct TypeRegistry {
    // Struct Registration
    /// Map of struct IDs to struct declarations. Can be None while collecting the struct body.
    structs: HashMap<StructID, StructDecl>,
    name_to_id: HashMap<(NameSpaceID, String), StructID>,

    // Array Registration
    array_id_to_decl: HashMap<ArrayID, ArrayDecl>,
    decl_to_array_id: HashMap<ArrayDecl, ArrayID>,

    // Typealiases
    alias_to_type: HashMap<(NameSpaceID, String), ResolvedType>,

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
                .get_alias_type(namespace_id, type_name)
                .copied()
                .or_else(|| {
                    self.get_struct_id(namespace_id, type_name)
                        .map(ResolvedType::Struct)
                }),
        }
    }

    pub fn register_or_get_array(&mut self, elem_type: ResolvedType, count: u32) -> ArrayID {
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

    pub fn get_array_decl(&self, id: &ArrayID) -> Option<&ArrayDecl> {
        self.array_id_to_decl.get(id)
    }

    pub fn get_alias_type(&self, namespace_id: NameSpaceID, alias: &str) -> Option<&ResolvedType> {
        self.alias_to_type.get(&(namespace_id, alias.to_string()))
    }

    // --- TYPE SIZE AND ALIGNMENT ---

    pub fn get_type_actual_size(&self, type_id: &ResolvedType) -> Option<u32> {
        match type_id {
            ResolvedType::Primitive(ty) => Some(ty.size()),
            ResolvedType::Array(array_id) => self.get_array_decl(array_id).and_then(|a| {
                self.get_type_actual_size(a.item_type())
                    .map(|s| s * *a.count())
            }),
            ResolvedType::Struct(struct_id) => self.get_struct(struct_id).map(|s| s.total_size),
        }
    }

    pub fn get_type_alignment(&self, type_id: &ResolvedType) -> Option<u32> {
        match type_id {
            ResolvedType::Primitive(ty) => Some(ty.alignment()),
            ResolvedType::Array(array_id) => self
                .get_array_decl(array_id)
                .and_then(|a| self.get_type_alignment(a.item_type())),
            ResolvedType::Struct(struct_id) => self.get_struct(struct_id).map(|s| s.alignment),
        }
    }

    // --- REGISTRATION ---

    pub fn register_struct(&mut self, namespace_id: NameSpaceID, name: String) -> StructID {
        let id = self.generate_struct_id();
        self.name_to_id.insert((namespace_id, name), id);
        id
    }

    pub fn set_struct_decl(&mut self, struct_id: StructID, struct_decl: StructDecl) {
        self.structs.insert(struct_id, struct_decl);
    }

    pub fn register_typealias(
        &mut self,
        namespace_id: NameSpaceID,
        alias: String,
        target: ResolvedType,
    ) {
        self.alias_to_type.insert((namespace_id, alias), target);
    }

    // --- FORMATTING ---

    pub fn format_type(&self, ty: &ResolvedType) -> String {
        match ty {
            ResolvedType::Primitive(prim_type) => prim_type.to_string(),
            ResolvedType::Array(id) => self
                .get_array_decl(id)
                .map(|a| format!("[{}; {}]", self.format_type(a.item_type()), a.count()))
                .unwrap_or(format!("array(ID: {})", id.0)),
            ResolvedType::Struct(id) => self
                .get_struct(id)
                .map(|s| s.name.clone())
                .unwrap_or(format!("struct(ID: {})", id.0)),
        }
    }
}
