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
    ExprToken, ExprTokenKind, InfixOperatorProperties, OperatorAssociativity, ParserBodyStmt,
    ParserBodyStmtKind, ParserFuncCallArg, ParserFuncParam, ParserIfArm, ParserInputAttribute,
    ParserOperatorType, ParserTopLevelStmt, ParserTopLevelStmtKind, Range, SymbolPath,
    SymbolPathComponent,
};

peg::parser!(pub grammar kasl_parser() for str {
    pub rule parse() -> Vec<ParserTopLevelStmt>
        = top_level_stmts()

    // --- STATEMENTS ---

    rule top_level_stmts() -> Vec<ParserTopLevelStmt>
        = __? statements:(top_level_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule body_stmts() -> Vec<ParserBodyStmt>
        = __? statements:(body_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule top_level_stmt() -> ParserTopLevelStmt
        = func_decl_statement()
        / input_statement()
        / output_statement()
        / state_var_statement()
        / global_var_statement()
        / struct_decl_statement()
        / operator_definition_statement()
        / operator_func_statement()
        / expected!("statement")

    rule body_stmt() -> ParserBodyStmt
        = return_statement()
        / local_var_statement()
        / assign_statement()
        / func_call_statement()
        / if_statement()
        / block_statement()
        / expected!("statement")

    rule func_decl_statement() -> ParserTopLevelStmt
        = start:position!() is_static:("static" _)? "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) comma()? ")" _?
        return_type:("->" _? t:id_chain() { t })? body:(__? "{"
        __? body:body_stmts() __?
        "}" { body })? end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::FuncDecl { is_static: is_static.is_some(), name, params, return_type, body }
            }
        }

    rule return_statement() -> ParserBodyStmt
        = start:position!() "return" value:(_ v:oneline_expression() { v })? end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::Return { value }
            }
        }

    rule input_statement() -> ParserTopLevelStmt
        = start:position!() attrs:(__? a:input_attr() { a })* __? "input" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::Input { name, value_type, def_val, attrs }
            }
        }

    rule output_statement() -> ParserTopLevelStmt
        = start:position!() "output" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::Output { name, value_type, def_val }
            }
        }

    rule state_var_statement() -> ParserTopLevelStmt
        = start:position!() "state" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::StateVar { name, value_type, def_val }
            }
        }

    rule global_var_statement() -> ParserTopLevelStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::ScopeVar { name, value_type, def_val }
            }
        }

    rule local_var_statement() -> ParserBodyStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:id_chain() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::LocalVar { name, value_type, def_val }
            }
        }

    rule assign_statement() -> ParserBodyStmt
        = start:position!() target:id_chain() _ "=" _ value:oneline_expression() end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::Assign { target, value }
            }
        }

    rule func_call_statement() -> ParserBodyStmt
        = start:position!() path:id_chain() _? "(" __? args:func_call_args() ")" end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::FuncCall { path, args }
            }
        }

    rule if_statement() -> ParserBodyStmt
        = start:position!() main:if_arm()
        else_ifs:(__? "else" _ ifCond:if_arm() { ifCond })*
        else_body:body_stmts()?
        end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::If {
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

    rule struct_decl_statement() -> ParserTopLevelStmt
        = start:position!() "struct" _ name:identifier() __? "{"
        __? body:top_level_stmts() __?
        "}" end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::StructDecl {
                    name,
                    body
                }
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
    rule operator_definition_statement() -> ParserTopLevelStmt
        = start:position!() "operator" _ kind:(
            "infix" _ symbol:operator() __? "{" __? props:infix_properties() __? "}" {
                ParserTopLevelStmtKind::InfixDefine { symbol, infix_properties: props }
            }
        ) end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind,
            }
        }

    // Operator Function
    rule operator_func_statement() -> ParserTopLevelStmt
        = start:position!() "func" _ op_type:("infix" { ParserOperatorType::Infix } / "prefix" { ParserOperatorType::Prefix }) _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:id_chain() __? body:("{"
        __? body:body_stmts() __?
        "}" { body }) end:position!() {
            ParserTopLevelStmt {
                range: Range::n(start, end),
                kind: ParserTopLevelStmtKind::OperatorFunc { op_type, symbol, params, return_type, body },
            }
        }

    rule block_statement() -> ParserBodyStmt
        = start:position!() "{" _? statements:body_stmts() _? "}" end:position!() {
            ParserBodyStmt {
                range: Range::n(start, end),
                kind: ParserBodyStmtKind::Block { statements }
            }
        }

    // --- STATEMENT HELPERS ---

    // Function Parameter
    rule func_param() -> ParserFuncParam
        = start:position!() label:param_label()? name:identifier() value_type:(_? ":" _? t:id_chain() { t })? def_val:(_? "=" _? v:multiline_expression() { v })? end:position!() {
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
        = parts:( expr_element() ** (_?) ) {
            parts.into_iter().flatten().collect()
        }

    pub rule multiline_expression() -> Vec<ExprToken>
        = parts:( expr_element() ** (__?) ) {
            parts.into_iter().flatten().collect()
        }

    rule expr_element() -> Vec<ExprToken>
        = lparen_start:position!() "(" _? inner:multiline_expression() _? rparen_start:position!() ")" {
            let mut v = vec![ExprToken::lparen(Range::n(lparen_start, lparen_start + 1))];
            v.extend(inner);
            v.push(ExprToken::rparen(Range::n(rparen_start, rparen_start + 1)));
            v
        }
        / single:expr_token() { vec![single] }

    rule expr_token() -> ExprToken
        = start:position!() kind:(
            literal()
            / func_call()
            / id_chain_token()
            / operator_token()
        ) end:position!() {
            ExprToken { range: Range::n(start, end), kind }
        }

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

    rule operator_token() -> ExprTokenKind
        = op:operator() { ExprTokenKind::Operator(op) }

    // --- TOKENS ---

    rule identifier() -> String
        = quiet!{
            !reserved()
            n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() }
        }
        / expected!("identifier")

    rule id_chain() -> SymbolPath
        = parent:(start:position!() symbol:identifier() end:position!() {
            SymbolPathComponent {
                symbol,
            }
        }) children:(start:position!() dot() symbol:identifier() end:position!() {
            SymbolPathComponent {
                symbol,
            }
        })* {
            let mut components = vec![parent];
            components.extend(children);
            SymbolPath::with(components)
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
        = ("input" / "output" / "var" / "state" / "func" / "return" / "if" / "else"
            / "struct" / "operator" / "infix" / "prefix") !['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

    rule comment() = "//" (!['\n'] [_])* &['\n']

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() = quiet!{([' ' | '\t'] / comment())+}

    rule __() = quiet!{([' ' | '\t' | '\n'] / comment())+}
});
