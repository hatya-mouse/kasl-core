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
    ExprToken, ExprTokenKind, Range,
    global_decl_collection::expr_builder::ExpressionBuilder,
    resolution::expr_inference::SymbolTypeGetter,
    type_registry::{PrimitiveType, ResolvedType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypedToken {
    pub kind: TypedTokenKind,
    pub range: Range,
}

impl TypedToken {
    pub fn new(kind: TypedTokenKind, range: Range) -> Self {
        TypedToken { kind, range }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedTokenKind {
    Value {
        expr_token: ExprToken,
        value_type: ResolvedType,
    },
    PrefixOperator(String),
    InfixOperator(String),
    Parenthesized(Vec<TypedToken>),
}

impl ExpressionBuilder<'_> {
    /// Infer the type of each token in the expression and convert them to TypedTokens.
    pub fn get_typed_tokens(&mut self, expr: &[ExprToken]) -> Option<Vec<TypedToken>> {
        let expr_iter = expr.iter().peekable();
        let mut result: Vec<TypedToken> = Vec::new();

        for token in expr_iter {
            match &token.kind {
                ExprTokenKind::IntLiteral(_) => {
                    result.push(TypedToken {
                        kind: TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: ResolvedType::Primitive(PrimitiveType::Int),
                        },
                        range: token.range,
                    });
                }

                ExprTokenKind::FloatLiteral(_) => {
                    result.push(TypedToken {
                        kind: TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: ResolvedType::Primitive(PrimitiveType::Float),
                        },
                        range: token.range,
                    });
                }

                ExprTokenKind::BoolLiteral(_) => {
                    result.push(TypedToken {
                        kind: TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: ResolvedType::Primitive(PrimitiveType::Bool),
                        },
                        range: token.range,
                    });
                }

                ExprTokenKind::Access(var_name) => {
                    let Some(var_id) = self.scope_registry.lookup_var(self.current_scope, var_name)
                    else {
                        return None;
                    };
                    let Some(var) = self.scope_registry.get_var_by_id(var_id) else {
                        return None;
                    };
                    result.push(TypedToken {
                        kind: TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: var.value_type,
                        },
                        range: token.range,
                    });
                }

                ExprTokenKind::FuncCall { name, args } => {
                    let Some(func_id) = self.func_ctx.get_global_func_by_name(name) else {
                        return None;
                    };
                    let Some(func) = self.func_ctx.get_func(func_id) else {
                        return None;
                    };
                    let Some(return_type) = func.return_type.clone() else {
                        return None;
                    };
                    result.push(TypedToken {
                        kind: TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: return_type,
                        },
                        range: token.range,
                    });
                }

                ExprTokenKind::Operator(operator_symbol) => {
                    let last_token = result.last();
                    let operator_token = ExpressionBuilder::handle_operator_resolution(
                        operator_symbol,
                        token.range,
                        last_token,
                    );
                    result.push(operator_token);
                }

                ExprTokenKind::Parenthesized(inner) => {
                    let inner_tokens = self.get_typed_tokens(&inner)?;
                    result.push(TypedToken {
                        kind: TypedTokenKind::Parenthesized(inner_tokens),
                        range: token.range,
                    });
                }
            }
        }

        Some(result)
    }

    fn handle_operator_resolution(
        operator_symbol: &str,
        operator_range: Range,
        last_token: Option<&TypedToken>,
    ) -> TypedToken {
        // Whether the operator is infix or prefix can be determined by the last token
        let is_infix = match last_token {
            Some(unwrapped_token) => matches!(
                unwrapped_token.kind,
                TypedTokenKind::Value {
                    expr_token: _,
                    value_type: _,
                } | TypedTokenKind::Parenthesized(_)
            ),
            None => false,
        };

        if is_infix {
            TypedToken::new(
                TypedTokenKind::InfixOperator(operator_symbol.to_string()),
                operator_range,
            )
        } else {
            TypedToken::new(
                TypedTokenKind::PrefixOperator(operator_symbol.to_string()),
                operator_range,
            )
        }
    }
}
