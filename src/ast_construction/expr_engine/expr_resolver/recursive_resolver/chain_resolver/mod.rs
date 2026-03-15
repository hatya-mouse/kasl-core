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

mod builtin_resolver;
mod chain_lhs_resolver;
mod field_access_resolver;
mod member_access_resolver;
mod member_func_resolver;
mod static_func_resolver;

use std::{iter::Peekable, slice::Iter};

use crate::{
    Expr, ExprKind, NameSpaceID, Range,
    error::Ph,
    expr_engine::ExpressionResolver,
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

        // Resolve the namespace ID from the chain elements
        let namespace_id = if let Some(lhs) = lhs {
            self.namespace.namespace_id
        } else {
            self.resolve_namespace(&mut elements_iter)
        };

        // The namespace is resolved
        let namespace = self
            .namespace_registry
            .get_namespace_by_id(namespace_id)
            .unwrap();

        let mut expr = None;
        // Check if the next element is a type
        if let Some(UnresolvedChainElement::Identifier { name }) = elements_iter.peek() {
            if let Some(struct_id) = namespace.type_registry.get_struct_id_by_name(name) {
                // Consume the element
                elements_iter.next();

                // Assume that the next element is a static function
                let Some(next_element) = elements_iter.next() else {
                    self.ec.expr_ends_with_type(range, Ph::ExprEngine);
                    return None;
                };
                match next_element {
                    UnresolvedChainElement::Identifier { .. } => {
                        self.ec.static_var_access(range, Ph::ExprEngine);
                        return None;
                    }
                    UnresolvedChainElement::FuncCall {
                        name,
                        args: no_type_args,
                    } => {
                        // Get the function ID by name
                        let Some(func_id) = self
                            .namespace
                            .func_ctx
                            .get_member_func_by_name(&struct_id, &name)
                        else {
                            let struct_decl =
                                self.namespace.type_registry.get_struct(&struct_id)?;
                            self.ec.member_func_not_found(
                                range,
                                Ph::ExprEngine,
                                &struct_decl.name,
                                name,
                            );
                            return None;
                        };

                        // Get the function by ID
                        let func = self.namespace.func_ctx.get_func(&func_id)?;

                        // Throw an error if the function is static
                        if func.is_static {
                            self.ec
                                .static_func_call_on_instance(range, Ph::ExprEngine, &func.name);
                            return None;
                        }

                        // Resolve the arguments
                        let args =
                            self.resolve_func_call_args(&func.params, &no_type_args, range)?;

                        // Construct the expression
                        expr = Some(Expr::new(
                            ExprKind::StaticFuncCall { id: func_id, args },
                            func.return_type,
                            range,
                        ));
                    }
                }
            }
        }

        while let Some(element) = elements_iter.next() {
            match element {
                UnresolvedChainElement::Identifier { name } => {}
                UnresolvedChainElement::FuncCall { name, args } => {}
            }
        }

        None // DUMMY
    }

    pub fn resolve_namespace(
        &mut self,
        elements: &mut Peekable<Iter<UnresolvedChainElement>>,
    ) -> NameSpaceID {
        let mut current_id = self.namespace.namespace_id;
        // Loop over the elements in the chain and get the namespace ID from the first tokens
        while let Some(element) = elements.peek() {
            match element {
                UnresolvedChainElement::Identifier { name } => {
                    let current_namespace = self
                        .namespace_registry
                        .get_namespace_by_id(current_id)
                        .unwrap();
                    if let Some(namespace_id) = current_namespace.get_id_by_name(name) {
                        // Consume the identifier
                        elements.next();
                        current_id = namespace_id;
                    } else {
                        return current_id;
                    }
                }
                _ => return current_id,
            }
        }
        current_id
    }
}
