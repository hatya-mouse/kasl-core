use std::collections::HashMap;

use crate::{
    ExprToken, FuncParam, InputAttribute, LiteralBind, ProtocolRequirement, StateVar, Statement,
};

peg::parser!(pub grammar kash() for str {
    pub rule parse() -> Vec<Statement>
        = statements()

    // --- STATEMENTS ---

    rule statements() -> Vec<Statement>
        = statement() ** "\n"

    rule statement() -> Statement
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
        / infix_statement()
        / prefix_statement()
        / postfix_statement()

    rule func_decl_statement() -> Statement
        = "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) ")" _? return_type:("->" _? t:identifier() { t })? __? "{"
        __? body:statements() __?
        "}" {
            Statement::FuncDecl { name, params, return_type, body }
        }

    rule return_statement() -> Statement
        = "return" _ value:expression() {
            Statement::Return { value }
        }

    rule input_statement() -> Statement
        = "input" _ name:identifier() value_type:(_? ":" _ t:identifier() { t })? def_val:(_ "=" _ v:expression() { v })? _ attrs:(input_attr() ** _) {
            Statement::Input { name, value_type, def_val, attrs }
        }

    rule output_statement() -> Statement
        = "output" _ name:identifier() _? ":" _ value_type:identifier() {
            Statement::Output { name, value_type }
        }

    rule var_statement() -> Statement
        = "var" _ name:identifier() value_type:(_? ":" _ t:identifier() { t })? _ "=" _ def_val:expression() {
            Statement::Var { name, value_type, def_val }
        }

    rule func_call_statement() -> Statement
        = name:id_chain() _ "(" __? args:(expression() ** comma()) ")" {
            Statement::FuncCall { name, args }
        }

    rule state_statement() -> Statement
        = "state" _ "{" __ vars:(state_var() ** "\n") __ "}" {
            Statement::State { vars }
        }

    rule assign_statement() -> Statement
        = property:id_chain() _ "=" _ value:expression() {
            Statement::Assign { property, value }
        }

    rule if_statement() -> Statement
        = "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" {
            Statement::If { condition, body }
        }

    rule if_else_statement() -> Statement
        = "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" __ "else" __ "{"
        __? else_body:statements() __?
        "}" {
            Statement::IfElse { condition, body, else_body }
        }

    rule struct_decl_statement() -> Statement
        = "struct" _ name:identifier() _? ":" _? inherits:(identifier() ** comma()) "{"
        __? body:statements() __?
        "}" {
            Statement::StructDecl { name, inherits, body }
        }

    rule protocol_decl_statement() -> Statement
        = "protocol" _ name:identifier() _? ":" _? inherits:(identifier() ** comma()) "{"
        __? requires:(protocol_requirement() ** "\n") __?
        "}" {
            Statement::ProtocolDecl { name, inherits, requires }
        }

    rule init_statement() -> Statement
        = literal_bind:literal_bind()? "init" _? "(" _? params:(func_param() ** comma()) ")" __? "{"
        __? body:statements() __?
        "}" {
            Statement::Init { literal_bind, params, body }
        }

    rule infix_statement() -> Statement
        = "infix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() __? "{"
        attrs:infix_attrs()
        "}" __? ":" __? "{"
        __? body:statements() __?
        "}" {
            Statement::Infix { symbol, params, return_type, attrs, body }
        }

    rule prefix_statement() -> Statement
        = "prefix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() __? "{"
        __? body:statements() __?
        "}" {
            Statement::Prefix { symbol, params, return_type, body }
        }

    rule postfix_statement() -> Statement
        = "postfix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() __? "{"
        __? body:statements() __?
        "}" {
            Statement::Postfix { symbol, params, return_type, body }
        }

    // --- STATEMENT HELPERS ---

    // Function Parameter
    rule func_param() -> FuncParam
        = label:(label:identifier() _ { label })? name:identifier() value_type:(_? ":" _ t:identifier() { t })? def_val:(_ "=" _ v:expression() { v })? {
            FuncParam { label, name, value_type, def_val }
        }

    // Input Attribute
    rule input_attr() -> InputAttribute
        = "#" name:identifier() opt_args:("(" _? args:(expression() ** comma()) ")" { args })? {
            InputAttribute { name, args: match opt_args {
                None => vec![],
                Some(args) => args
            } }
        }

    // State Variable
    rule state_var() -> StateVar
        = name:identifier() value_type:(_? ":" _ t:identifier() { t })? _ def_val:(
            ("=" _ v:expression() { v })
            / expected!("Default value is required for state variables")
        ) {
            StateVar { name, value_type, def_val }
        }

    // --- Protocol Requirements ---

    rule protocol_requirement() -> ProtocolRequirement
        = protocol_func()
        / protocol_infix()
        / protocol_prefix()
        / protocol_postfix()

    rule protocol_func() -> ProtocolRequirement
        = "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) ")" _? return_type:("->" _? t:identifier() { t })? {
            ProtocolRequirement::Func { name, params, return_type }
        }

    rule protocol_infix() -> ProtocolRequirement
        = "infix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() "{"
        attrs:infix_attrs()
        "}" {
            ProtocolRequirement::Infix { symbol, params, return_type, attrs }
        }

    rule protocol_prefix() -> ProtocolRequirement
        = "prefix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() {
            ProtocolRequirement::Prefix { symbol, params, return_type }
        }

    rule protocol_postfix() -> ProtocolRequirement
        = "postfix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) ")" _? "->" _? return_type:identifier() {
            ProtocolRequirement::Postfix { symbol, params, return_type }
        }

    // Literal Binding
    rule literal_bind() -> LiteralBind
        = "intliteral" { LiteralBind::IntLiteral }
        / "floatliteral" { LiteralBind::FloatLiteral }
        / "boolliteral" { LiteralBind::BoolLiteral }

    // Infix Attributes
    rule infix_attrs() -> HashMap<String, String>
        = entries:((key:identifier() _? ":" _? value:(v:identifier() / v:number() { v }) {
            (key, value)
        }) ** comma()) {
            entries.into_iter().map(|(k, v)| (k, v)).collect()
        }

    // --- EXPRESSIONS ---

    pub rule expression() -> Vec<ExprToken>
        = (t:expr_token() __? { t })*
        / "(" _ expr:expression() _ ")" { expr }
        / expected!("expression")

    rule expr_token() -> ExprToken
        = (symbol:operator() { ExprToken::Operator(symbol) })
        / literal()
        / func_call()
        / (ids:id_chain() { ExprToken::Identifier(ids) })

    rule func_call() -> ExprToken
        = name:id_chain() _ "(" __? args:(expression() ** comma()) ")" {
            ExprToken::FuncCall { name, args }
        }

    rule literal() -> ExprToken
        = n:number() { ExprToken::Literal(n) }

    // --- TOKENS ---

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule id_chain() -> Vec<String>
        = parent:identifier() children:(dot() c:id_chain() { c })? { match children {
            Some(children) => vec![parent].into_iter().chain(children.into_iter()).collect(),
            None => vec![parent]
        } }

    rule operator() -> String
        = op:$(['+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!' | '?' | '%' | '|' | '&']) { op.to_owned() }

    rule number() -> String
        = decimal() / integer()

    rule integer() -> String
        = n:$(['0'..='9']+) { n.to_owned() }

    rule decimal() -> String
        = n:integer() "." d:$(['0'..='9']+) {
            n + "." + d
        }

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() =  quiet!{[' ' | '\t']*}

    rule __() = quiet!{[' ' | '\t' | '\n']*}
});
