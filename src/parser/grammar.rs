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

use crate::{
    ChainOp, ExprToken, ExprTokenKind, InfixOperatorProperties, OperatorAssociativity,
    ParserDeclStmt, ParserDeclStmtKind, ParserFuncCallArg, ParserFuncParam, ParserIfArm,
    ParserInputAttribute, ParserOperatorType, ParserScopeStmt, ParserScopeStmtKind, Range,
    SymbolPath, SymbolPathComponent,
};

peg::parser!(pub grammar kasl_parser() for str {
    pub rule parse() -> Vec<ParserDeclStmt>
        = top_level_stmts()

    // --- STATEMENTS ---

    rule top_level_stmts() -> Vec<ParserDeclStmt>
        = __? statements:(top_level_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule body_stmts() -> Vec<ParserScopeStmt>
        = __? statements:(body_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule top_level_stmt() -> ParserDeclStmt
        = func_decl_statement()
        / input_statement()
        / output_statement()
        / state_var_statement()
        / struct_field_statement()
        / struct_decl_statement()
        / operator_definition_statement()
        / operator_func_statement()
        / expected!("statement")

    rule body_stmt() -> ParserScopeStmt
        = return_statement()
        / local_var_statement()
        / assign_statement()
        / func_call_statement()
        / if_statement()
        / block_statement()
        / expected!("statement")

    rule func_decl_statement() -> ParserDeclStmt
        = start:position!() is_static:("static" _)? "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) comma()? ")" _?
        return_type:("->" _? t:type_name() { t })? __? "{"
        __? body:body_stmts() __?
        "}" end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::FuncDecl { is_static: is_static.is_some(), name, params, return_type, body }
            }
        }

    rule return_statement() -> ParserScopeStmt
        = start:position!() "return" value:(_ v:oneline_expression() { v })? end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Return { value }
            }
        }

    rule input_statement() -> ParserDeclStmt
        = start:position!() attrs:(__? a:input_attr() { a })* __? "input" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::Input { name, value_type, def_val, attrs }
            }
        }

    rule output_statement() -> ParserDeclStmt
        = start:position!() "output" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::Output { name, value_type, def_val }
            }
        }

    rule state_var_statement() -> ParserDeclStmt
        = start:position!() "state" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::StateVar { name, value_type, def_val }
            }
        }

    rule struct_field_statement() -> ParserDeclStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::StructField { name, value_type, def_val }
            }
        }

    rule local_var_statement() -> ParserScopeStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::LocalVar { name, value_type, def_val }
            }
        }

    rule assign_statement() -> ParserScopeStmt
        = start:position!() target:type_name() _ "=" _ value:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Assign { target, value }
            }
        }

    rule func_call_statement() -> ParserScopeStmt
        = start:position!() path:type_name() _? "(" __? args:func_call_args() ")" end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::FuncCall { path, args }
            }
        }

    rule if_statement() -> ParserScopeStmt
        = start:position!() main:if_arm()
        else_ifs:(__? "else" _ ifCond:if_arm() { ifCond })*
        else_body:body_stmts()?
        end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::If {
                    main,
                    else_ifs,
                    else_body: else_body.unwrap_or(Vec::new()),
                }
            }
        }

    rule if_arm() -> ParserIfArm
        = start:position!() "if" _ condition:oneline_expression() __? "{"
        __? body:body_stmts() __?
        "}" end:position!() {
            ParserIfArm { condition, body, range: Range::n(start, end) }
        }

    rule struct_decl_statement() -> ParserDeclStmt
        = start:position!() "struct" _ name:identifier() __? "{"
        __? body:top_level_stmts() __?
        "}" end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::StructDecl {
                    name,
                    body
                }
            }
        }

    // Infix Operator Properties
    rule infix_properties() -> InfixOperatorProperties
        = start:position!() precedence:precedence_prop() __? comma() __? associativity:associativity_prop() end:position!() {
            InfixOperatorProperties { precedence, associativity, range: Range::n(start, end) }
        }
        / start:position!() associativity:associativity_prop() __? comma() __? precedence:precedence_prop() end:position!() {
            InfixOperatorProperties { precedence, associativity, range: Range::n(start, end) }
        }

    rule precedence_prop() -> u32
        = "precedence" _? ":" _? value:integer() { value }

    rule associativity_prop() -> OperatorAssociativity
        = "associativity" _? ":" _? value:(
            "left" { OperatorAssociativity::Left }
            / "right" { OperatorAssociativity::Right }
            / "none" { OperatorAssociativity::None }
        ) { value }

    // Operator Definition
    rule operator_definition_statement() -> ParserDeclStmt
        = start:position!() "operator" _ kind:(
            "infix" _ symbol:operator() __? "{" __? props:infix_properties() __? "}" {
                ParserDeclStmtKind::InfixDefine { symbol, infix_properties: props }
            }
        ) end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind,
            }
        }

    // Operator Function
    rule operator_func_statement() -> ParserDeclStmt
        = start:position!() "func" _ op_type:("infix" { ParserOperatorType::Infix } / "prefix" { ParserOperatorType::Prefix }) _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:type_name() __? body:("{"
        __? body:body_stmts() __?
        "}" { body }) end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::OperatorFunc { op_type, symbol, params, return_type, body },
            }
        }

    rule block_statement() -> ParserScopeStmt
        = start:position!() "{" _? statements:body_stmts() _? "}" end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Block { statements }
            }
        }

    // --- STATEMENT HELPERS ---

    // Function Parameter
    rule func_param() -> ParserFuncParam
        = start:position!() label:param_label()? name:identifier() value_type:(_? ":" _? t:type_name() { t })? def_val:(_? "=" _? v:multiline_expression() { v })? end:position!() {
            ParserFuncParam { label, name, value_type, def_val, range: Range::n(start, end) }
        }

    rule param_label() -> String
        = label:identifier() _ { label }

    // Input Attribute
    rule input_attr() -> ParserInputAttribute
        = start:position!() "#" name:identifier() opt_args:("(" _? args:(multiline_expression() ** comma()) comma()? ")" { args })? end:position!() {
            ParserInputAttribute { name, args: opt_args.unwrap_or_default(), range: Range::n(start, end) }
        }

    // Function Call Argument
    rule func_call_args() -> Vec<ParserFuncCallArg>
        = start:position!() entries:((label:(n:identifier() _? ":" _? { n })? value:multiline_expression() end:position!() {
            ParserFuncCallArg { label, value, range: Range::n(start, end) }
        }) ** comma()) comma()? {
            entries
        }

    // --- EXPRESSIONS ---

    pub rule oneline_expression() -> Vec<ExprToken>
        = expr_token() ** (_?)

    pub rule multiline_expression() -> Vec<ExprToken>
        = expr_token() ** (__?)

    rule expr_token() -> ExprToken
        = start:position!() kind:(
            literal()
            / func_call()
            / identifier_token()
            / operator_token()
            / parenthesized_token()
        ) end:position!() {
            ExprToken { range: Range::n(start, end), kind }
        }
        / token:chain_token() { token }

    rule primary() -> ExprToken
        = start:position!() kind:(
            literal()
            / func_call()
            / identifier_token()
            / parenthesized_token()
        ) end:position!() {
            ExprToken { range: Range::n(start, end), kind }
        }

    // --- TOKENS ---

    rule literal() -> ExprTokenKind
        = decimal:decimal() { ExprTokenKind::FloatLiteral(decimal) }
        / integer:integer() { ExprTokenKind::IntLiteral(integer) }
        / boolean:boolean() { ExprTokenKind::BoolLiteral(boolean) }

    rule func_call() -> ExprTokenKind
        = id:identifier() _? "(" __? args:func_call_args() ")" {
            ExprTokenKind::FuncCall { name: id, args }
        }

    rule identifier_token() -> ExprTokenKind
        = id:identifier() { ExprTokenKind::Access(id) }

    rule operator_token() -> ExprTokenKind
        = op:operator() { ExprTokenKind::Operator(op) }

    rule parenthesized_token() -> ExprTokenKind
        = "(" __? expr:multiline_expression() ")" { ExprTokenKind::Pharenthesized(expr) }

    rule chain_token() -> ExprToken
    = lhs:primary() extensions:(start:position!() __? "." __? op:chain_op() end:position!() { (op, Range::n(start, end)) })+ {
            extensions.into_iter().fold(lhs, |lhs, op| {
                ExprToken { range: op.1, kind: ExprTokenKind::Chain { lhs: Box::new(lhs), op: op.0 } }
            })
        }

    rule chain_op() -> ChainOp
        = id:identifier() { ChainOp::Access(id) }
        / id:identifier() __? "(" __? args:func_call_args() __? ")" {
            ChainOp::FuncCall { name: id, args }
        }

    rule identifier() -> String
        = quiet!{
            !reserved()
            n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() }
        }
        / expected!("identifier")

    rule type_name() -> SymbolPath
        = first:identifier() extensions:(_? "." _? id:identifier() _? { id })* {
            let first = SymbolPathComponent::new(first);
            let extensions = extensions.into_iter().map(|id| SymbolPathComponent::new(id)).collect();
            SymbolPath::with(vec![first]).extended(extensions)
        }

    rule operator() -> String
        = quiet!{ op:$(['+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!' | '?' | '%' | '|' | '&']+) { op.to_owned() } }
        / expected!("operator")

    rule integer() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule decimal() -> f32
        = n:$(['0'..='9']+) "." d:$(['0'..='9']+) {
            (n.to_owned() + "." + d).parse().unwrap()
        }

    rule boolean() -> bool
        = quiet!{ "true" { true } / "false" { false } }
        / expected!("boolean")

    rule reserved()
        = ("input" / "output" / "var" / "state" / "static" / "func" / "return" / "if" / "else"
            / "struct" / "operator" / "infix" / "prefix") !['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

    rule comment() = "//" (!['\n'] [_])* &['\n']

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() = quiet!{([' ' | '\t'] / comment())+}

    rule __() = quiet!{([' ' | '\t' | '\n'] / comment())+}
});
