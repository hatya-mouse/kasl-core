//
// Copyright 2025 Shuntaro Kasatani
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

use crate::{ConstructorError, ExprToken, ExprTokenKind, Expression, Program, TypedTokenKind};

pub fn build_expr_tree(
    program: &Program,
    rpn_tokens: Vec<ExprToken>,
) -> Result<Expression, ConstructorError> {
    let stack = Vec::new();

    for current_token in rpn_tokens.into_iter() {
        match current_token.kind {
            TypedTokenKind::Value {
                expr_token,
                value_type,
            } => match expr_token.kind {
                ExprTokenKind::
            },

            // AI Suggestion:
            // Good — here's a short, simple recipe.
            //
            // Use a stack of nodes (`Node`).
            // For each token in the `RPN` sequence:
            //   - If token is a `Value` (literal/identifier): push a leaf `Node(Value)`.
            //   - If token is a `PrefixOperator`: pop 1 node (operand), create `Node(Operator, operand)`, push it.
            //   - If token is an `InfixOperator`: pop right then left (order matters), create `Node(Operator, left, right)`, push it.
            // After all tokens, the stack must contain exactly one node — that is the expression tree root. If not, return an error (`ArityMismatch` or `TrailingValues`).
            // Error cases to check: insufficient operands when popping, multiple items left at end, unknown operator arity.
            //
            // That’s it — stack-based rebuild from RPN produces the tree.
        }
    }

    Ok(())
}
