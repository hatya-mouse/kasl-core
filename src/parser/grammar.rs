use crate::{
    ExprToken, ExprTokenKind, InfixOperatorProperties, OperatorAssociativity, ParserDeclStmt,
    ParserDeclStmtKind, ParserFuncCallArg, ParserFuncParam, ParserIfArm, ParserInputAttribute,
    ParserOperatorType, ParserScopeStmt, ParserScopeStmtKind, PostfixOperatorProperties,
    PrefixOperatorProperties, Range, SymbolPath, SymbolPathComponent,
    namespace_registry::ImportPath, parser_ast::ParserTypeName,
};

peg::parser!(pub grammar kasl_parser() for str {
    pub rule parse() -> Vec<ParserDeclStmt>
        = decl_stmts()

    // --- STATEMENTS ---

    rule decl_stmts() -> Vec<ParserDeclStmt>
        = __? statements:(decl_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule scope_stmts() -> Vec<ParserScopeStmt>
        = __? statements:(scope_stmt() ** ((_? "\n" _?)+)) __? { statements }

    rule decl_stmt() -> ParserDeclStmt
        = import_statement()
        / typealias_statement()
        / func_decl_statement()
        / input_statement()
        / output_statement()
        / state_var_statement()
        / global_let_statement()
        / struct_field_statement()
        / struct_decl_statement()
        / operator_definition_statement()
        / operator_func_statement()
        / expected!("STATEMENT")

    rule scope_stmt() -> ParserScopeStmt
        = return_statement()
        / local_var_statement()
        / local_let_statement()
        / loop_statement()
        / if_statement()
        / block_statement()
        / assign_statement()
        / expr_statement()
        / expected!("STATEMENT")

    rule import_statement() -> ParserDeclStmt
        = start:position!() "import" _ path:import_path() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::Import { path },
            }
        }

    rule typealias_statement() -> ParserDeclStmt
    = start:position!() "typealias" _ alias:identifier() _? "=" _? target:type_name() end:position!() {
        ParserDeclStmt {
            range: Range::n(start, end),
            kind: ParserDeclStmtKind::Typealias { alias, target }
        }
    }

    rule func_decl_statement() -> ParserDeclStmt
        = start:position!() is_static:("static" __)? "func" __ name:identifier() __? "(" __? params:(func_param() ** comma()) __? comma()? ")" __?
        return_type:("->" __? t:type_name() { t })? __? "{"
        __? body:scope_stmts() __?
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

    rule global_let_statement() -> ParserDeclStmt
        = start:position!() "let" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::GlobalConst { name, value_type, def_val }
            }
        }

    rule struct_field_statement() -> ParserDeclStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::StructField { name, value_type, def_val }
            }
        }

    rule struct_decl_statement() -> ParserDeclStmt
        = start:position!() "struct" _ name:identifier() __? "{"
        __? body:decl_stmts() __?
        "}" end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::StructDecl {
                    name,
                    body
                }
            }
        }

    // --- PARSER SCOPE STMTS ---

    rule local_var_statement() -> ParserScopeStmt
        = start:position!() "var" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::LocalVar { name, value_type, def_val }
            }
        }

    rule local_let_statement() -> ParserScopeStmt
        = start:position!() "let" _ name:identifier() value_type:(_? ":" _? t:type_name() { t })? _? "=" _? def_val:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::LocalConst { name, value_type, def_val }
            }
        }

    rule assign_statement() -> ParserScopeStmt
        = start:position!() target:lvalue_expression() _ "=" _ value:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Assign { target, value }
            }
        }

    rule if_statement() -> ParserScopeStmt
        = start:position!() main:if_arm()
        else_ifs:(__? "else" _ ifCond:if_arm() { ifCond })*
        else_arm:(
            else_start:position!() __? "else" __? "{"
            __? else_body:scope_stmts() __?
            "}" else_end:position!() {
                (else_body, Range::n(else_start, else_end))
            }
        )?
        end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::If {
                    main,
                    else_ifs,
                    else_range: else_arm.as_ref().map(|a| a.1),
                    else_body: else_arm.map(|a| a.0),
                }
            }
        }

    rule if_arm() -> ParserIfArm
        = start:position!() "if" _ condition:oneline_expression() __? "{"
        __? body:scope_stmts() __?
        "}" end:position!() {
            ParserIfArm { condition, body, range: Range::n(start, end) }
        }

    rule expr_statement() -> ParserScopeStmt
        = start:position!() expr:oneline_expression() end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Expression { expr }
            }
        }

    rule loop_statement() -> ParserScopeStmt
        = start:position!() "loop" _ count:oneline_expression() __? "{"
        __? body:scope_stmts() __?
        "}" end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Loop { count, body }
            }
        }

    rule block_statement() -> ParserScopeStmt
        = start:position!() "{" __? statements:scope_stmts() __? "}" end:position!() {
            ParserScopeStmt {
                range: Range::n(start, end),
                kind: ParserScopeStmtKind::Block { statements }
            }
        }

    // --- OPERATOR PROPERTIES ---

    rule precedence_prop() -> u32
        = "precedence" _? ":" _? value:integer() { value }

    rule associativity_prop() -> OperatorAssociativity
        = "associativity" _? ":" _? value:(
            "left" { OperatorAssociativity::Left }
            / "right" { OperatorAssociativity::Right }
            / "none" { OperatorAssociativity::None }
        ) { value }

    rule infix_properties() -> InfixOperatorProperties
        = start:position!() precedence:precedence_prop() __? comma() __? associativity:associativity_prop() end:position!() {
            InfixOperatorProperties { precedence, associativity, range: Range::n(start, end) }
        }
        / start:position!() associativity:associativity_prop() __? comma() __? precedence:precedence_prop() end:position!() {
            InfixOperatorProperties { precedence, associativity, range: Range::n(start, end) }
        }

    rule prefix_properties() -> PrefixOperatorProperties
        = start:position!() precedence:precedence_prop() end:position!() {
            PrefixOperatorProperties { precedence, range: Range::n(start, end) }
        }

    rule postfix_properties() -> PostfixOperatorProperties
        = start:position!() precedence:precedence_prop() end:position!() {
            PostfixOperatorProperties { precedence, range: Range::n(start, end) }
        }

    // Operator Definition
    rule operator_definition_statement() -> ParserDeclStmt
        = start:position!() "operator" _ kind:(
            "infix" _ symbol:operator() __? "{" __? props:infix_properties() __? "}" {
                ParserDeclStmtKind::InfixDefine { symbol, props }
            }
            / "prefix" _ symbol:operator() __? "{" __? props:prefix_properties() __? "}" {
                ParserDeclStmtKind::PrefixDefine { symbol, props }
            }
            / "postfix" _ symbol:operator() __? "{" __? props:postfix_properties() __? "}" {
                ParserDeclStmtKind::PostfixDefine { symbol, props }
            }
        ) end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind,
            }
        }

    // Operator Function
    rule operator_func_statement() -> ParserDeclStmt
        = start:position!() "func" _ op_type:("infix" { ParserOperatorType::Infix } / "prefix" { ParserOperatorType::Prefix } / "postfix" { ParserOperatorType::Postfix }) _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:type_name() __? body:("{"
        __? body:scope_stmts() __?
        "}" { body }) end:position!() {
            ParserDeclStmt {
                range: Range::n(start, end),
                kind: ParserDeclStmtKind::OperatorFunc { op_type, symbol, params, return_type, body },
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
        = start:position!() entries:((label:(n:identifier() __? ":" __? { n })? value:multiline_expression() __? end:position!() {
            ParserFuncCallArg { label, value, range: Range::n(start, end) }
        }) ** comma()) __? comma()? {
            entries
        }

    // --- EXPRESSIONS ---

    pub rule oneline_expression() -> Vec<ExprToken>
        = expr:expr_token() ++ (_?) { expr } / expected!("EXPRESSION")

    pub rule multiline_expression() -> Vec<ExprToken>
        = expr:expr_token() ++ (__?) { expr } / expected!("EXPRESSION")

    pub rule bracket_content() -> Vec<ExprToken>
        = expr:bracket_content_token() ++ (__?) { expr } / expected!("EXPRESSION")

    pub rule lvalue_expression() -> Vec<ExprToken>
        = expr:lvalue_token() ++ (__?) { expr } / expected!("EXPRESSION")

    rule expr_token() -> ExprToken
        = start:position!() kind:(
            operator_token()
            / literal()
            / func_call()
            / identifier_token()
            / parenthesized_token()
            / dot_token()
            / bracketed_token()
        ) end:position!() {
            ExprToken { range: Range::n(start, end), kind }
        }

    rule bracket_content_token() -> ExprToken
        = start:position!() kind:(
            operator_token()
            / literal()
            / func_call()
            / identifier_token()
            / parenthesized_token()
            / dot_token()
            / bracketed_token()
            / semicolon()
            / colon()
        ) end:position!() {
            ExprToken { range: Range::n(start, end), kind }
        }

    rule lvalue_token() -> ExprToken
        = start:position!() kind:(
            dot_token()
            / identifier_token()
            / bracketed_token()
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
        = id:identifier() { ExprTokenKind::Identifier(id) }

    rule operator_token() -> ExprTokenKind
        = op:operator() { ExprTokenKind::Operator(op) }

    rule parenthesized_token() -> ExprTokenKind
        = "(" __? expr:multiline_expression() __? ")" { ExprTokenKind::Parenthesized(expr) }

    rule bracketed_token() -> ExprTokenKind
        = "[" __? inner:bracket_content() __? "]" { ExprTokenKind::Bracketed(inner) }

    rule dot_token() -> ExprTokenKind
        = "." { ExprTokenKind::Dot }

    rule semicolon() -> ExprTokenKind = ";" { ExprTokenKind::Semicolon }
    rule colon() -> ExprTokenKind = "," { ExprTokenKind::Comma }

    // --- MISCELLANEOUS ---

    rule identifier() -> String
        = quiet!{
            !reserved()
            n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() }
        }
        / expected!("identifier")

    rule type_name() -> ParserTypeName
        = first:identifier() extensions:(_? "." _? id:identifier() _? { id })* {
            let first = SymbolPathComponent::new(first);
            let extensions = extensions.into_iter().map(SymbolPathComponent::new).collect();
            ParserTypeName::SymbolPath(SymbolPath::with(vec![first]).extended(extensions))
        }
        / "[" __? t:type_name() __? ";" __? i:integer() __? "]" {
            ParserTypeName::Array(Box::new(t), i)
        }

    rule operator() -> String
        = quiet!{ op:$(['+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!' | '?' | '%' | '|' | '&' | '@' | '~']+) { op.to_owned() } }
        / expected!("operator")

    rule import_path() -> ImportPath
        = path:identifier() ** (_? "/" _?) {
            ImportPath {
                path: path.into_iter().collect(),
            }
        }

    rule integer() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule decimal() -> f32
        = n:$(['0'..='9']+) "." d:$(['0'..='9']+) {
            (n.to_owned() + "." + d).parse::<f32>().unwrap()
        }

    rule boolean() -> bool
        = quiet!{ "true" { true } / "false" { false } }
        / expected!("boolean")

    rule reserved()
        = ("input" / "output" / "var" / "let" / "state" / "static" / "func" / "return" / "if" / "else"
            / "struct" / "operator" / "infix" / "prefix" / "postfix") !['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

    rule comment() = "//" (!['\n'] [_])* &['\n']

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() = quiet!{([' ' | '\t'] / comment())+}

    rule __() = quiet!{([' ' | '\t' | '\n'] / comment())+}
});
