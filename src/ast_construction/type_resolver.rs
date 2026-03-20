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
    compilation_data::ProgramContext, error::ErrorCollector, parser_ast::ParserTypeName,
    type_registry::ResolvedType,
};

pub fn resolve_type(
    ec: &mut ErrorCollector,
    prog_ctx: &mut ProgramContext,
    parser_type: &ParserTypeName,
) -> Option<ResolvedType> {
    match parser_type {
        ParserTypeName::SymbolPath(path) => {
            let (namespace_id, type_name) = prog_ctx
                .namespace_registry
                .resolve_namespace_from_path(path.clone());

            // Resolved the type name
            match prog_ctx
                .type_registry
                .resolve_type_name(namespace_id, &type_name.to_string())
            {
                Some(ty) => Some(ty),
                None => None,
            }
        }
        ParserTypeName::Array(item_type, count) => {
            let resolved_item_type = resolve_type(ec, prog_ctx, item_type)?;

            // Register or get the array ID
            let array_id = prog_ctx
                .type_registry
                .register_or_get_array(resolved_item_type, *count);
            Some(ResolvedType::Array(array_id))
        }
    }
}
