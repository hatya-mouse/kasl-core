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

use kasl::{ExprToken, ParserIfArm, ParserScopeStmt, ParserScopeStmtKind, Range, SymbolPath};

pub fn block(statements: &[ParserScopeStmt]) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::Block {
            statements: statements.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn local_var(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::LocalVar {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn local_const(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::LocalConst {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn assign(target: &[ExprToken], value: &[ExprToken]) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::Assign {
            target: target.to_vec(),
            value: value.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn expression(expr: &[ExprToken]) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::Expression {
            expr: expr.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn if_stmt(
    main: ParserIfArm,
    else_ifs: &[ParserIfArm],
    else_body: Option<&[ParserScopeStmt]>,
) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::If {
            main,
            else_ifs: else_ifs.to_vec(),
            else_body: else_body.map(|s| s.to_vec()),
        },
        range: Range::zero(),
    }
}

pub fn return_stmt(value: Option<&[ExprToken]>) -> ParserScopeStmt {
    ParserScopeStmt {
        kind: ParserScopeStmtKind::Return {
            value: value.map(|v| v.to_vec()),
        },
        range: Range::zero(),
    }
}

pub fn if_arm(condition: &[ExprToken], body: &[ParserScopeStmt]) -> ParserIfArm {
    ParserIfArm {
        condition: condition.to_vec(),
        body: body.to_vec(),
        range: Range::zero(),
    }
}
