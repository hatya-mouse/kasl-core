use kasl::{
    ExprToken, ParserIfArm, ParserScopeStmt, ParserScopeStmtKind, Range, parser_ast::ParserTypeName,
};

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
    value_type: Option<ParserTypeName>,
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
    value_type: Option<ParserTypeName>,
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
            else_range: Some(Range::zero()),
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
