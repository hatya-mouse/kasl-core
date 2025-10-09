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

use crate::{ParserStatement, ParserStatementKind, Program};

pub fn build_graph(program: &Program, statements: &[ParserStatement]) {
    for stmt in statements {
        match &stmt.kind {
            ParserStatementKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => {}

            ParserStatementKind::Output {
                name: (),
                value_type: (),
            } => {}

            ParserStatementKind::State { vars } => {}

            ParserStatementKind::StructDecl {
                name: (),
                inherits: (),
                body: (),
            } => {}

            ParserStatementKind::ProtocolDecl {
                name,
                inherits,
                body,
            } => {}

            _ => (),
        }
    }
}

pub fn build_struct_and_protocol_graph(program: &Program, statements: &[ParserStatement]) {}
