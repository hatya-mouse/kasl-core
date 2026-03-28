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

use crate::{
    ast::{
        Range, StructID,
        type_registry::{ResolvedType, StructDecl, StructField},
    },
    ast_construction::global_decl_collection::GlobalDeclCollector,
    parser::{ExprToken, parser_ast::ParserTypeName},
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_struct_field(
        &mut self,
        struct_id: StructID,
        struct_decl: &mut StructDecl,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        // Resolve the default value expression
        let Some(def_val) = self.resolve_def_val_global(value_type, def_val, decl_range) else {
            return;
        };

        // Add the field to the struct graph if the value has a struct type
        // The graph will be used in the scope graph analyzing phase to check if there aren't any struct cycles
        self.add_struct_dependency(struct_id, &def_val.value_type);

        // Create a struct field
        let struct_field = StructField {
            name: name.to_string(),
            value_type: def_val.value_type,
            def_val,
            range: decl_range,
        };

        // Register the struct field in the type registry
        struct_decl.register_field(struct_field);
    }

    fn add_struct_dependency(&mut self, struct_id: StructID, value_type: &ResolvedType) {
        match value_type {
            ResolvedType::Struct(field_struct_id) => {
                self.comp_data
                    .struct_graph
                    .add_edge(struct_id, *field_struct_id);
            }
            ResolvedType::Array(array_id) => {
                if let Some(array_decl) = self.prog_ctx.type_registry.get_array_decl(array_id) {
                    let item_type = *array_decl.item_type();
                    self.add_struct_dependency(struct_id, &item_type);
                }
            }
            _ => {}
        }
    }
}
