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

use crate::ast_nodes::{Expr, ScopeID, VariableID, symbol_table::LValue};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Block {
    pub body: Vec<Statement>,
    pub scope_id: ScopeID,
}

impl Block {
    pub fn new(scope_id: ScopeID) -> Self {
        Self {
            body: Vec::new(),
            scope_id,
        }
    }

    pub fn set_stmt(&mut self, stmts: Vec<Statement>) {
        self.body = stmts;
    }

    pub fn get_scope_id(&self) -> ScopeID {
        self.scope_id
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Statement {
    Block {
        block: Block,
    },
    LocalVar {
        var_id: VariableID,
    },
    LocalConst {
        var_id: VariableID,
    },
    Assign {
        target: LValue,
        value: Expr,
    },
    Expression {
        expr: Expr,
    },
    If {
        main: IfArm,
        else_ifs: Vec<IfArm>,
        else_block: Option<Block>,
    },
    Return {
        value: Option<Expr>,
    },
    Loop {
        count: u32,
        body: Block,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct IfArm {
    pub condition: Expr,
    pub block: Block,
}

impl IfArm {
    pub fn new(condition: Expr, block: Block) -> Self {
        Self { condition, block }
    }
}
