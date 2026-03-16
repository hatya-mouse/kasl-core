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

mod builtin_func_resolver;
mod field_access_resolver;
mod func_args_resolver;
mod func_call_resolver;
mod identifier_resolver;
mod instance_func_resolver;
mod static_func_resolver;

use std::{iter::Peekable, slice::Iter};

use crate::{
    Expr, Range, ScopeID,
    error::Ph,
    expr_engine::ExpressionResolver,
    namespace_registry::{NameSpacePair, NameSpaceStructGetter},
    symbol_table::{UnresolvedChainElement, UnresolvedExpr},
};

impl ExpressionResolver<'_> {
    pub fn resolve_chain(
        &mut self,
        lhs: Option<UnresolvedExpr>,
        elements: Vec<UnresolvedChainElement>,
        range: Range,
    ) -> Option<Expr> {
        // Resolve the LHS expression
        // let resolved_lhs = self.resolve_chain_lhs(lhs)?;

        let mut elements_iter = elements.iter().peekable();

        let mut expr = None;
        let mut target_scope = self.current_scope;

        if let Some(lhs) = lhs {
            // Resolve the lhs
            let resolved_lhs = self.resolve_recursively(lhs)?;
            expr = Some(resolved_lhs);
        } else {
            target_scope = self.resolve_namespace_scope(&mut elements_iter);

            if let Some(UnresolvedChainElement::Identifier { name }) = elements_iter.peek() {
                // Check if the next element is a type
                if let Some(struct_id) = self
                    .namespace_registry
                    .get_struct_id(&target_scope.namespace_id, name)
                {
                    // Consume the type name element
                    elements_iter.next();

                    // Assume that the next element is a static function
                    let Some(next_element) = elements_iter.next() else {
                        self.ec.expr_ends_with_type(range, Ph::ExprEngine);
                        return None;
                    };

                    // Resolve the static function call
                    expr = Some(self.resolve_static_func_call(struct_id, next_element, range)?);
                } else if name == "Builtin" {
                    // It's safe to unwrap because it has already been peeked
                    let element = elements_iter.next().unwrap();
                    // Resolve the builtin function call
                    expr = Some(self.resolve_builtin_func_call(element, range)?);
                }
            }
        }

        // Resolve member access and function calls
        while let Some(element) = elements_iter.next() {
            match element {
                UnresolvedChainElement::Identifier { name } => {
                    if let Some(last_expr) = expr {
                        expr = Some(self.resolve_field_access(last_expr, name, range)?)
                    } else {
                        expr = Some(self.resolve_identifier(target_scope, name, range)?)
                    }
                }
                UnresolvedChainElement::FuncCall {
                    name,
                    args: no_type_args,
                } => {
                    if let Some(last_expr) = expr {
                        expr = Some(self.resolve_instance_func_call(
                            last_expr,
                            name,
                            no_type_args,
                            range,
                        )?)
                    } else {
                        expr = Some(self.resolve_func_call(
                            target_scope.namespace_id,
                            name,
                            no_type_args,
                            range,
                        )?)
                    }
                }
            }
        }

        None // DUMMY
    }

    pub fn resolve_namespace_scope(
        &mut self,
        elements: &mut Peekable<Iter<UnresolvedChainElement>>,
    ) -> NameSpacePair<ScopeID> {
        let mut current_scope = self.current_scope;
        // Loop over the elements in the chain and get the namespace ID from the first tokens
        while let Some(element) = elements.peek() {
            match element {
                UnresolvedChainElement::Identifier { name } => {
                    let current_namespace = self
                        .namespace_registry
                        .get_namespace_by_id(&current_scope.namespace_id)
                        .unwrap();
                    if let Some(namespace_id) = current_namespace.get_id_by_name(name)
                        && let Some(namespace) =
                            self.namespace_registry.get_namespace_by_id(&namespace_id)
                    {
                        // Consume the identifier
                        elements.next();
                        current_scope.namespace_id = namespace_id;
                        // Get the global scope of the namespace
                        let global_scope = namespace.scope_registry.get_global_scope_id();
                        current_scope.symbol_id = global_scope;
                    } else {
                        return current_scope;
                    }
                }
                _ => return current_scope,
            }
        }
        current_scope
    }
}
