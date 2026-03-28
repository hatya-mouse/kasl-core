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

mod builtin_func_resolver;
mod field_access_resolver;
mod func_args_resolver;
mod func_call_resolver;
mod identifier_resolver;
mod instance_func_resolver;
mod static_func_resolver;

use crate::{
    ast::{
        Expr, NameSpaceID, Range, ScopeID,
        symbol_table::{UnresolvedChainElement, UnresolvedExpr},
    },
    ast_construction::expr_engine::ExpressionResolver,
    error::Ph,
};
use std::{iter::Peekable, slice::Iter};

impl ExpressionResolver<'_> {
    pub fn resolve_chain(
        &mut self,
        lhs: Option<UnresolvedExpr>,
        elements: Vec<UnresolvedChainElement>,
        expr_range: Range,
    ) -> Option<Expr> {
        let mut elements_iter = elements.iter().peekable();

        let mut expr = None;
        let mut target_scope = self.current_scope;
        let mut target_namespace = self.current_namespace;

        if let Some(lhs) = lhs {
            // Resolve the lhs
            let resolved_lhs = self.resolve_recursively(lhs)?;
            expr = Some(resolved_lhs);
        } else {
            (target_scope, target_namespace) = self.resolve_namespace_scope(&mut elements_iter);

            if let Some(UnresolvedChainElement::Identifier { name, range }) = elements_iter.peek() {
                // Check if the next element is a type
                if let Some(struct_id) = self
                    .prog_ctx
                    .type_registry
                    .get_struct_id(target_namespace, name)
                {
                    // Consume the type name element
                    elements_iter.next();

                    // Assume that the next element is a static function
                    let Some(next_element) = elements_iter.next() else {
                        self.ec.expr_ends_with_type(*range, Ph::ExprEngine);
                        return None;
                    };

                    // Resolve the static function call
                    expr =
                        Some(self.resolve_static_func_call(struct_id, next_element, expr_range)?);
                } else if name == "Builtin" {
                    // Consume the "Builtin" element
                    elements_iter.next();
                    // Get the next element
                    let Some(next_element) = elements_iter.next() else {
                        self.ec.expr_ends_with_builtin(*range, Ph::ExprEngine);
                        return None;
                    };
                    // Resolve the builtin function call
                    expr = Some(self.resolve_builtin_func_call(next_element, expr_range)?);
                }
            }
        }

        // Resolve member access and function calls
        for element in &mut elements_iter {
            match element {
                UnresolvedChainElement::Identifier { name, range } => {
                    if let Some(last_expr) = expr {
                        expr = Some(self.resolve_field_access(last_expr, name, *range)?)
                    } else {
                        expr = Some(self.resolve_identifier(target_scope, name, *range)?)
                    }
                }
                UnresolvedChainElement::FuncCall {
                    name,
                    args: no_type_args,
                    range,
                } => {
                    if let Some(last_expr) = expr {
                        expr = Some(self.resolve_instance_func_call(
                            last_expr,
                            name,
                            no_type_args,
                            *range,
                        )?)
                    } else {
                        expr = Some(self.resolve_func_call(
                            target_namespace,
                            name,
                            no_type_args,
                            *range,
                        )?)
                    }
                }
            }
        }

        expr
    }

    fn resolve_namespace_scope(
        &mut self,
        elements: &mut Peekable<Iter<UnresolvedChainElement>>,
    ) -> (ScopeID, NameSpaceID) {
        let mut current_scope = self.current_scope;
        let mut current_namespace = self.current_namespace;
        // Loop over the elements in the chain and get the namespace ID from the first tokens
        while let Some(element) = elements.peek() {
            match element {
                UnresolvedChainElement::Identifier { name, .. } => {
                    let namespace_ref = self
                        .prog_ctx
                        .namespace_registry
                        .get_namespace_by_id(&current_namespace)
                        .unwrap();
                    if let Some(namespace_id) = namespace_ref.get_id_by_name(name) {
                        // Consume the identifier
                        elements.next();
                        current_namespace = namespace_id;
                        // Get the global scope of the namespace
                        let global_scope = self
                            .prog_ctx
                            .scope_registry
                            .get_global_scope_id(&current_namespace);
                        current_scope = global_scope;
                    } else {
                        return (current_scope, current_namespace);
                    }
                }
                _ => return (current_scope, current_namespace),
            }
        }
        (current_scope, current_namespace)
    }
}
