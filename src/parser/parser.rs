//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ExprToken, ExprTokenKind, InfixOperatorProperties, LiteralBind, OperatorAssociativity,
    ParserFuncCallArg, ParserFuncParam, ParserInputAttribute, ParserOperatorType, ParserStateVar,
    ParserStatement, ParserStatementKind, ParserSymbolPath, ParserSymbolPathComponent, Range,
};

peg::parser!(pub grammar kasl_parser() for str {
    pub rule parse() -> Vec<ParserStatement>
        = statements()

    // --- STATEMENTS ---

    rule statements() -> Vec<ParserStatement>
        = __? statements:(statement() ** ((_? "\n" _?)+)) __? { statements }

    rule statement() -> ParserStatement
        = func_decl_statement()
        / return_statement()
        / input_statement()
        / output_statement()
        / var_statement()
        / state_statement()
        / assign_statement()
        / func_call_statement()
        / if_statement()
        / if_else_statement()
        / struct_decl_statement()
        / protocol_decl_statement()
        / init_statement()
        / operator_definition_statement()
        / operator_func_statement()
        / block_statement()
        / expected!("statement")

    rule func_decl_statement() -> ParserStatement
        = start:position!() required_by:(r:id_chain() _ { r })?
        "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) comma()? ")" _?
        return_type:("->" _? t:id_chain() { t })? body:(__? "{"
        __? body:statements() __?
        "}" { body })? end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::FuncDecl { required_by, name, params, return_type, body }
            }
        }

    rule return_statement() -> ParserStatement
        = start:position!() "return" value:(_ v:expression() { v })? end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Return { value }
            }
        }

    rule input_statement() -> ParserStatement
        = start:position!() "input" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? def_val:(_? "=" _? v:expression() { v })? attrs:(_? a:input_attr() { a })* end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Input { name, value_type, def_val, attrs }
            }
        }

    rule output_statement() -> ParserStatement
        = start:position!() "output" _ name:identifier() _? ":" _? value_type:id_chain() end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Output { name, value_type }
            }
        }

    rule var_statement() -> ParserStatement
        = start:position!() required_by:(r:id_chain() _ { r })? "var" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? def_val:(_? "=" _? def_val:expression() { def_val })? end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Var { required_by, name, value_type, def_val }
            }
        }

    rule state_statement() -> ParserStatement
        = start:position!() "state" _? "{" __? vars:(state_var() ** ((_? "\n" _?)+)) __? "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::State { vars }
            }
        }

    rule assign_statement() -> ParserStatement
        = start:position!() target:id_chain() _ "=" _ value:expression() end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Assign { target, value }
            }
        }

    rule func_call_statement() -> ParserStatement
        = start:position!() name:id_chain() _? "(" __? args:func_call_args() ")" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::FuncCall { name, args }
            }
        }

    rule if_statement() -> ParserStatement
        = start:position!() "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::If { condition, body }
            }
        }

    rule if_else_statement() -> ParserStatement
        = start:position!() "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" __ "else" __ "{"
        __? else_body:statements() __?
        "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::IfElse { condition, body, else_body }
            }
        }

    rule struct_decl_statement() -> ParserStatement
        = start:position!() "struct" _ name:identifier() inherits:(_? ":" _? i:(id_chain() ** comma()) comma()? { i })? _? "{"
        __? body:statements() __?
        "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::StructDecl {
                    name,
                    inherits: match inherits {
                        Some(inherits) => inherits,
                        None => Vec::new()
                    },
                    body
                }
            }
        }

    rule protocol_decl_statement() -> ParserStatement
        = start:position!() "protocol" _ name:identifier() inherits:(_? ":" _? i:(id_chain() ** comma()) comma()? { i })? _? "{"
        __? body:statements() __?
        "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::ProtocolDecl { name, inherits: match inherits {
                    Some(inherits) => inherits,
                    None => Vec::new()
                }, body }
            }
        }

    rule init_statement() -> ParserStatement
        = start:position!() required_by:id_chain()? literal_bind:(l:literal_bind() _ { l })? "init" _? "(" _? params:(func_param() ** comma()) comma()? ")" body:(__? "{"
        __? body:statements() __?
        "}" { body })? end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Init { required_by, literal_bind, params, body }
            }
        }

        // Infix Operator Properties
        rule infix_properties() -> InfixOperatorProperties
        = precedence:precedence_prop() __? comma() __? associativity:associativity_prop() {
            InfixOperatorProperties { precedence, associativity }
        }
        / associativity:associativity_prop() __? comma() __? precedence:precedence_prop() {
            InfixOperatorProperties { precedence, associativity }
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
    rule operator_definition_statement() -> ParserStatement
        = start:position!() "operator" _ kind:(
            "infix" _ symbol:operator() __? "{" __? props:infix_properties() __? "}" {
                ParserStatementKind::InfixDefine { symbol, infix_properties: props }
            }
            / "prefix" _ symbol:operator() {
                ParserStatementKind::PrefixDefine { symbol }
            }
        ) end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind,
            }
        }

    // Operator Function
    rule operator_func_statement() -> ParserStatement
        = start:position!() "func" _ op_type:("infix" { ParserOperatorType::Infix } / "prefix" { ParserOperatorType::Prefix }) _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:id_chain() __? body:("{"
        __? body:statements() __?
        "}" { body }) end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::OperatorFunc { op_type, symbol, params, return_type, body },
            }
        }

    rule block_statement() -> ParserStatement
        = start:position!() "{" _? statements:statements() _? "}" end:position!() {
            ParserStatement {
                range: Range::n(start, end),
                kind: ParserStatementKind::Block { statements }
            }
        }

    // --- STATEMENT HELPERS ---

    // Function Parameter
    rule func_param() -> ParserFuncParam
        = start:position!() label:param_label()? name:identifier() value_type:(_? ":" _? t:id_chain() { t })? def_val:(_? "=" _? v:expression() { v })? end:position!() {
            ParserFuncParam { label, name, value_type, def_val, range: Range::n(start, end) }
        }

    rule param_label() -> String
        = label:identifier() _ { label }

    // Input Attribute
    rule input_attr() -> ParserInputAttribute
        = start:position!() "#" name:identifier() opt_args:("(" _? args:(expression() ** comma()) comma()? ")" { args })? end:position!() {
            ParserInputAttribute { name, args: match opt_args {
                None => vec![],
                Some(args) => args
            }, range: Range::n(start, end) }
        }

    // State ScopeVar
    rule state_var() -> ParserStateVar
        = start:position!() name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:expression() end:position!() {
            ParserStateVar { name, value_type, def_val, range: Range::n(start, end) }
        }

    // Function Call Argument
    rule func_call_args() -> Vec<ParserFuncCallArg>
        = start:position!() entries:((label:(n:identifier() _? ":" _? { n })? value:expression() end:position!() {
            ParserFuncCallArg { label, value, range: Range::n(start, end) }
        }) ** comma()) comma()? {
            entries
        }

    // Literal Binding
    rule literal_bind() -> LiteralBind
        = "intliteral" { LiteralBind::IntLiteral }
        / "floatliteral" { LiteralBind::FloatLiteral }
        / "boolliteral" { LiteralBind::BoolLiteral }

    // --- EXPRESSIONS ---

    pub rule expression() -> Vec<ExprToken>
        = (
            !(__? "\n" / __? ")" / __? "}")
            first:expr_token()?
            rest:(
                ops:(__? op:operator_token() { op })+
                __? token:expr_token() {
                    (ops, token)
                }
            )*
            last:operator_token()? {
                let mut tokens = match first {
                    Some(first) => vec![first],
                    None => vec![],
                };
                for (ops, token) in rest {
                    tokens.extend(ops);
                    tokens.push(token);
                }
                if let Some(op) = last { tokens.push(op); }

                tokens
            }
        )
        / l_paren_start:position!() "(" l_paren_end:position!() _ expr:expression() _ r_paren_start:position!() ")" r_paren_end:position!() {
            let mut tokens = vec![
                ExprToken { kind: ExprTokenKind::LParen, range: Range::n(l_paren_start, l_paren_end) },
                ExprToken { kind: ExprTokenKind::RParen, range: Range::n(r_paren_start, r_paren_end) }
            ];
            tokens.splice(1..1, expr);
            tokens
        }
        / expected!("expression")

    rule operator_token() -> ExprToken
        = start:position!() op:operator() end:position!() { ExprToken { range: Range::n(start, end), kind: ExprTokenKind::Operator(op) } }

    rule expr_token() -> ExprToken
        = start:position!() kind:(
            literal()
            / func_call()
            / id_chain_token()
        ) end:position!() { ExprToken { range: Range::n(start, end), kind } }


    rule func_call() -> ExprTokenKind
        = path:id_chain() _? "(" __? args:func_call_args() ")" {
            ExprTokenKind::FuncCall { path, args }
        }

    rule literal() -> ExprTokenKind
        = decimal:decimal() { ExprTokenKind::FloatLiteral(decimal) }
        / integer:integer() { ExprTokenKind::IntLiteral(integer) }
        / boolean:boolean() { ExprTokenKind::BoolLiteral(boolean)}

    rule id_chain_token() -> ExprTokenKind
        = ids:id_chain() { ExprTokenKind::Identifier(ids) }

    // --- TOKENS ---

    rule identifier() -> String
        = quiet!{
            !reserved()
            n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() }
        }
        / expected!("identifier")

    rule id_chain() -> ParserSymbolPath
        = parent:(start:position!() symbol:identifier() end:position!() {
            ParserSymbolPathComponent {
                range: Range::n(start, end),
                symbol,
            }
        }) children:(start:position!() dot() symbol:identifier() end:position!() {
            ParserSymbolPathComponent {
                range: Range::n(start, end),
                symbol,
            }
        })* {
            let mut path = vec![parent];
            path.extend(children);
            path
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
        = ("input" / "output" / "var" / "state" / "func" / "return"
        / "if" / "else" / "struct" / "init" / "protocol" / "intliteral"
        / "floatliteral" / "boolliteral" / "define" / "impl" / "infix" / "prefix") !['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

    rule comment() = "//" (!['\n'] [_])* &['\n']

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() = quiet!{([' ' | '\t'] / comment())+}

    rule __() = quiet!{([' ' | '\t' | '\n'] / comment())+}
});
