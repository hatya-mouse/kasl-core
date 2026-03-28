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

use crate::ast::{Expr, ExprKind, ScopeRegistry, scope_manager::VariableKind};

pub(crate) fn get_constant_int(scope_registry: &ScopeRegistry, expr: &Expr) -> Option<u32> {
    match &expr.kind {
        ExprKind::IntLiteral(value) => Some(*value),
        ExprKind::Identifier(id) => {
            // Check if the variable is a constant
            if let Some(scope_var) = scope_registry.get_var(id)
                && matches!(
                    scope_var.var_kind,
                    VariableKind::GlobalConst | VariableKind::LocalConst
                )
                && let Some(def_val) = &scope_var.def_val
            {
                get_constant_int(scope_registry, def_val)
            } else {
                None
            }
        }
        _ => None,
    }
}
