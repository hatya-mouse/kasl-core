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

use crate::{StructID, SymbolPath};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Default, serde::Serialize)]
pub struct TypeRegistry {
    pub structs: HashMap<StructID, StructDecl>,
    pub name_to_id: HashMap<String, StructID>,
    next_struct_id: usize,
}

impl TypeRegistry {
    pub fn generate_struct_id(&mut self) -> StructID {
        let id = StructID::new(self.next_struct_id);
        self.next_struct_id += 1;
        id
    }

    pub fn resolve_type_path(&self, type_name: &String) -> Option<ResolvedType> {
        if let Ok(primitive_type) = PrimitiveType::from_str(&type_name) {
            return Some(ResolvedType::Primitive(primitive_type));
        }
        let id = self.get_struct_id_by_name(type_name)?;
        Some(ResolvedType::Struct(id))
    }

    pub fn get_struct_id_by_name(&self, type_name: &String) -> Option<StructID> {
        self.name_to_id.get(type_name).copied()
    }

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

    pub fn register_struct(&mut self, struct_decl: StructDecl, name: String, id: StructID) {
        self.structs.insert(id, struct_decl);
        self.name_to_id.insert(name, id);
    }

    pub fn get_struct(&self, id: &StructID) -> Option<&StructDecl> {
        self.structs.get(id)
    }

    pub fn has_struct(&self, type_name: &String) -> bool {
        self.name_to_id.contains_key(type_name)
    }

    pub fn format_type(&self, ty: &ResolvedType) -> String {
        match ty {
            ResolvedType::Primitive(ty) => ty.to_string(),
            ResolvedType::Struct(id) => self
                .get_struct(id)
                .map(|s| s.name.clone())
                .unwrap_or(format!("struct(ID: {})", id)),
        }
    }

    pub fn get_all_structs(&self) -> HashSet<StructID> {
        self.structs.keys().copied().collect()
    }
}
