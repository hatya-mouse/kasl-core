//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ExprToken, Expression, ParserSymbolPath, Range, SymbolPath,
    error::Phase,
    resolution::{TypeResolveCtx, expr_inference::ExprTreeBuilder},
};

impl<'a> TypeResolveCtx<'a> {
    /// Infer the type of the variable and convert the default value to Expression.
    ///
    /// # Arguments
    /// - `symbol_path`: SymbolPath of the variable to infer type for.
    /// - `decl_range`: Range of the declaration statement.
    /// - `value_type`: Type annotation of the variable in ParserSymbolPath.
    /// - `def_val`: Default value of the variable in ExprToken.
    ///
    /// # Return
    /// Returns the inferred type and the default value expression of the variable.
    pub fn resolve_var_type(
        &mut self,
        decl_range: Range,
        value_type: Option<&ParserSymbolPath>,
        def_val: &[ExprToken],
    ) -> Option<(SymbolPath, Expression)> {
        let parsed_expr =
            self.program
                .build_expr_tree_from_raw_tokens(self.ec, def_val, self.symbol_table)?;
        let def_val_type = parsed_expr.get_type(self.ec, self.program, decl_range)?;

        if let Some(value_type) = value_type {
            if let Some(annotation_type) = self.program.resolve_type_def_parser_path(value_type) {
                // Check if the type annotation matches the inferred type
                if annotation_type == def_val_type {
                    Some((annotation_type, parsed_expr))
                } else {
                    // If the type annotation doesn't match the inferred type, throw an error
                    self.ec.type_mismatch(
                        decl_range,
                        Phase::TypeResolution,
                        &annotation_type.to_string(),
                        &def_val_type.to_string(),
                    );
                    None
                }
            } else {
                // If the type annotation is not found, throw an error
                self.ec
                    .type_not_found(decl_range, Phase::TypeResolution, &value_type.to_string());
                None
            }
        } else {
            // If the symbol doesn't have a type annotation, use the inferred one
            Some((def_val_type, parsed_expr))
        }
    }
}
