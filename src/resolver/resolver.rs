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

use crate::{ParserStatement, Program, ProtocolType, StructType, parser_ast::ParserStatementKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResolverError {
    offset: usize,
    message: String,
}

pub struct Resolver {
    statements: Vec<ParserStatement>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: Vec<ParserStatement>) -> Result<(), ResolverError> {
        let mut program = Program::new();
        self.statements = statements;

        let (structs, protocols) = self.get_types(&self.statements);
        program.structs = structs;
        program.protocols = protocols;

        Ok(())
    }

    pub fn get_types(&self, stmts: &Vec<ParserStatement>) -> (Vec<StructType>, Vec<ProtocolType>) {
        let mut structs = Vec::new();
        let mut protocols = Vec::new();

        for stmt in stmts {
            match &stmt.kind {
                ParserStatementKind::StructDecl {
                    name,
                    inherits: _,
                    body: _,
                } => {
                    structs.push(StructType::new(name.clone()));
                }
                ParserStatementKind::ProtocolDecl {
                    name,
                    inherits: _,
                    body: _,
                } => {
                    protocols.push(ProtocolType::new(name.clone()));
                }
                _ => {}
            }
        }

        (structs, protocols)
    }
}
