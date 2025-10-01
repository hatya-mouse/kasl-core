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

use crate::{
    ParserStatement, ParserStatementKind, Program, ResolverError, ResolverErrorType, TypeDef,
    collection::{
        collect_member_functions, collect_member_nests, collect_member_operators,
        collect_member_variables,
    },
};

/// Collects every single members in the structs and protocols.
pub fn collect_all_type_members(
    program: &mut Program,
    stmts: &[ParserStatement],
) -> Result<(), ResolverError> {
    for stmt in stmts {
        match &stmt.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits: _,
                body,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits: _,
                body,
            } => {
                let type_def = match program.find_type_def_mut(name) {
                    Some(ty) => ty,
                    None => {
                        return Err(ResolverError {
                            error_type: ResolverErrorType::TypeNotFound(name.clone()),
                            offset: stmt.start,
                        });
                    }
                };
                collect_type_members(body, type_def)?;
            }

            _ => (),
        }
    }

    Ok(())
}

pub fn collect_type_members(
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
) -> Result<(), ResolverError> {
    collect_member_variables(stmts, type_def)?;
    collect_member_functions(stmts, type_def)?;
    collect_member_operators(stmts, type_def)?;
    collect_member_nests(stmts, type_def)?;

    Ok(())
}
