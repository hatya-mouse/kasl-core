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
    ParserStatement, ParserStatementKind, ResolverError, ResolverErrorType, TypeDef,
    collection::collect_type_members,
};

pub fn collect_member_nests(
    stmts: &[ParserStatement],
    type_def: &mut TypeDef,
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
                let child_type_def = type_def.fine_type_def_mut(name);
                match child_type_def {
                    Some(child_type_def) => {
                        collect_type_members(body, child_type_def)?;
                    }
                    None => {
                        return Err(ResolverError {
                            error_type: ResolverErrorType::TypeNotFound(name.to_string()),
                            offset: 0,
                        });
                    }
                }
            }

            _ => (),
        }
    }

    Ok(())
}
