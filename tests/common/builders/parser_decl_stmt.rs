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

use kasl::{
    ExprToken, InfixOperatorProperties, OperatorAssociativity, ParserDeclStmt, ParserDeclStmtKind,
    ParserFuncParam, ParserInputAttribute, ParserOperatorType, ParserScopeStmt,
    PostfixOperatorProperties, PrefixOperatorProperties, Range, SymbolPath,
};

pub fn func_decl(
    is_static: bool,
    name: &str,
    params: &[ParserFuncParam],
    return_type: Option<SymbolPath>,
    body: &[ParserScopeStmt],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::FuncDecl {
            is_static,
            name: name.to_string(),
            params: params.to_vec(),
            return_type,
            body: body.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn input(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
    attrs: &[ParserInputAttribute],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::Input {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
            attrs: attrs.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn output(name: &str, value_type: Option<SymbolPath>, def_val: &[ExprToken]) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::Output {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn state_var(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::StateVar {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn global_const(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::GlobalConst {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn struct_field(
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: &[ExprToken],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::StructField {
            name: name.to_string(),
            value_type,
            def_val: def_val.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn struct_decl(name: &str, body: &[ParserDeclStmt]) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::StructDecl {
            name: name.to_string(),
            body: body.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn infix_define(
    symbol: &str,
    precedence: u32,
    associativity: OperatorAssociativity,
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::InfixDefine {
            symbol: symbol.to_string(),
            props: InfixOperatorProperties {
                precedence,
                associativity,
                range: Range::zero(),
            },
        },
        range: Range::zero(),
    }
}

pub fn prefix_define(symbol: &str, precedence: u32) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::PrefixDefine {
            symbol: symbol.to_string(),
            props: PrefixOperatorProperties {
                precedence,
                range: Range::zero(),
            },
        },
        range: Range::zero(),
    }
}

pub fn postfix_define(symbol: &str, precedence: u32) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::PostfixDefine {
            symbol: symbol.to_string(),
            props: PostfixOperatorProperties {
                precedence,
                range: Range::zero(),
            },
        },
        range: Range::zero(),
    }
}

pub fn op_func(
    op_type: ParserOperatorType,
    symbol: &str,
    params: &[ParserFuncParam],
    return_type: SymbolPath,
    body: &[ParserScopeStmt],
) -> ParserDeclStmt {
    ParserDeclStmt {
        kind: ParserDeclStmtKind::OperatorFunc {
            op_type,
            symbol: symbol.to_string(),
            params: params.to_vec(),
            return_type,
            body: body.to_vec(),
        },
        range: Range::zero(),
    }
}

pub fn func_param(
    label: Option<&str>,
    name: &str,
    value_type: Option<SymbolPath>,
    def_val: Option<&[ExprToken]>,
) -> ParserFuncParam {
    ParserFuncParam {
        label: label.map(&str::to_string),
        name: name.to_string(),
        value_type,
        def_val: def_val.map(|s| s.to_vec()),
        range: Range::zero(),
    }
}
