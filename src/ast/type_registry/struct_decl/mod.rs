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

mod struct_field;

pub use struct_field::StructField;

use crate::{
    ast::{Range, type_registry::TypeRegistry},
    error::EK,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct StructDecl {
    /// The name of the struct.
    pub name: String,

    /// The fields of the struct.
    pub fields: Vec<StructField>,
    /// The map of field names to their indices in the `fields` vector.
    pub indices: HashMap<String, usize>,

    /// The map of field names to their offsets in bytes.
    pub field_offsets: Vec<u32>,
    /// The total size of the struct in bytes.
    pub total_size: u32,
    /// The alignment of the struct in bytes.
    pub alignment: u32,

    /// The range of the struct declaration in the source code.
    pub range: Range,
}

impl StructDecl {
    pub fn new(name: String, range: Range) -> Self {
        Self {
            name,
            fields: Vec::new(),
            indices: HashMap::new(),
            field_offsets: Vec::new(),
            total_size: 0,
            alignment: 1,
            range,
        }
    }

    pub fn get_field_by_index(&self, field_index: usize) -> Option<&StructField> {
        self.fields.get(field_index)
    }

    pub fn get_field_index(&self, field_name: &str) -> Option<usize> {
        self.indices.get(field_name).copied()
    }

    pub fn get_offset_by_index(&self, field_index: usize) -> Option<u32> {
        self.field_offsets.get(field_index).copied()
    }

    pub fn register_field(&mut self, field: StructField) {
        let field_index = self.fields.len();
        self.indices.insert(field.name.clone(), field_index);
        self.fields.push(field);
    }

    /// Calculates the memory layout of the struct and detects the struct cycle.
    pub fn compute_layout(&mut self, type_registry: &TypeRegistry) -> Result<(), EK> {
        let mut offset = 0u32;
        let mut max_alignment = 1u32;

        for field in &mut self.fields {
            // Get the size and alignment of the field's type
            let size = type_registry
                .get_type_actual_size(&field.value_type)
                .ok_or(EK::StructCycle)?;
            let alignment = type_registry.get_type_alignment(&field.value_type).unwrap();
            // If the alignment is greater than the max_alignment, update it
            if alignment > max_alignment {
                max_alignment = alignment;
            }
            // Align the offset to the field's alignment
            offset = (offset + (alignment - 1)) & !(alignment - 1);
            // Push the offset to the field_offsets vector
            self.field_offsets.push(offset);
            offset += size;
        }

        self.total_size = (offset).div_ceil(max_alignment) * max_alignment;
        self.alignment = max_alignment;

        Ok(())
    }
}
