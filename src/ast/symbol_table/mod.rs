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

mod expression;
mod function;
mod operator;
mod statement;

pub use expression::{
    Expr, ExprKind, LValue, MemberAccess, UnresolvedChainElement, UnresolvedExpr,
    UnresolvedExprKind,
};
pub use function::{
    FuncBodyMap, FuncCallArg, FuncParam, Function, FunctionContext, NoTypeFuncCallArg,
};
pub use operator::{
    InfixOperator, InfixOperatorProperties, InfixQueryRef, OpBodyMap, OperatorAssociativity,
    OperatorContext, PostfixOperator, PostfixOperatorProperties, PostfixQueryRef, PrefixOperator,
    PrefixOperatorProperties, PrefixQueryRef,
};
pub use statement::{Block, IfArm, Statement};
