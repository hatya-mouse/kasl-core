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

mod builtin_func;
mod field_access;
mod func_args;
mod func_call;
mod identifier;
mod instance_func;
mod static_func;

use crate::{
    ast_nodes::{
        Expr, NameSpaceID, Range, ScopeID,
        symbol_table::{UnresolvedChainElement, UnresolvedExpr},
    },
    error::Ph,
    semantic_analysis::expr_engine::ExpressionResolver,
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

        let mut expr;
        let mut target_scope = self.current_scope;
        let mut target_namespace = self.current_namespace;

        if let Some(lhs) = lhs {
            // Resolve the lhs expression first and use it as the starting point for resolving the chain
            let resolved_lhs = self.resolve_recursively(lhs)?;
            expr = Some(resolved_lhs);
        } else {
            (target_scope, target_namespace) = self.resolve_namespace_scope(&mut elements_iter);
            expr = self.resolve_first_identifier(&mut elements_iter, target_namespace, expr_range);
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
            let UnresolvedChainElement::Identifier { name, .. } = element else {
                return (current_scope, current_namespace);
            };

            let namespace_ref = self
                .prog_ctx
                .namespace_registry
                .get_namespace_by_id(&current_namespace)
                .unwrap();

            let Some(namespace_id) = namespace_ref.get_id_by_name(name) else {
                return (current_scope, current_namespace);
            };

            // Consume the identifier
            elements.next();
            current_namespace = namespace_id;

            // Get the global scope of the namespace
            let global_scope = self
                .prog_ctx
                .scope_registry
                .get_global_scope_id(&current_namespace);
            current_scope = global_scope;
        }
        (current_scope, current_namespace)
    }

    fn resolve_first_identifier(
        &mut self,
        elements_iter: &mut Peekable<Iter<UnresolvedChainElement>>,
        target_namespace: NameSpaceID,
        expr_range: Range,
    ) -> Option<Expr> {
        let Some(UnresolvedChainElement::Identifier { name, range }) = elements_iter.peek() else {
            return None;
        };

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
            Some(self.resolve_static_func_call(struct_id, next_element, expr_range)?)
        } else if name == "Builtin" {
            // Consume the "Builtin" element
            elements_iter.next();
            // Get the next element
            let Some(next_element) = elements_iter.next() else {
                self.ec.expr_ends_with_builtin(*range, Ph::ExprEngine);
                return None;
            };
            // Resolve the builtin function call
            Some(self.resolve_builtin_func_call(next_element, expr_range)?)
        } else {
            None
        }
    }
}
