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

use kasl::{
    ExprToken, ExprTokenKind, InfixOperatorProperties, OperatorAssociativity, Program, Range,
    SymbolPath, SymbolPathComponent, SymbolTable, TypedToken, TypedTokenKind,
    error::{EK, ErrorCollector, Pl},
    get_typed_tokens,
    resolution::{
        expr_inference::{build_expr_tree_from_rpn, rearrange_tokens_to_rpn},
        type_resolver::resolve_types,
    },
    symbol_path,
    table_construction::build_symbol_table,
    type_collection::collect_all_types,
};

/// Create a new value token.
fn v() -> TypedToken {
    TypedToken::new(
        TypedTokenKind::Value {
            expr_token: ExprToken {
                kind: ExprTokenKind::IntLiteral(0),
                range: Range::zero(),
            },
            value_type: SymbolPath::comp_int(),
        },
        Range::zero(),
    )
}

/// Create a new infix operator token.
fn inf(sym: &str) -> TypedToken {
    TypedToken::new(
        TypedTokenKind::InfixOperator(sym.to_string()),
        Range::zero(),
    )
}

/// Create a new prefix operator token.
fn pre(sym: &str) -> TypedToken {
    TypedToken::new(
        TypedTokenKind::PrefixOperator(sym.to_string()),
        Range::zero(),
    )
}

/// Create a new left parenthesis token.
fn lpar() -> TypedToken {
    TypedToken::new(TypedTokenKind::LParen, Range::zero())
}

/// Create a new right parenthesis token.
fn rpar() -> TypedToken {
    TypedToken::new(TypedTokenKind::RParen, Range::zero())
}

/// Convert the TypedToken sequence into a compact string representation
/// for easy assertions.
fn short_repr(tokens: &[TypedToken]) -> Vec<String> {
    tokens
        .iter()
        .map(|t| match &t.kind {
            TypedTokenKind::Value {
                value_type: ty,
                expr_token: _,
            } => format!("V<{}>", ty),
            TypedTokenKind::PrefixOperator(s) => format!("pre{}", s),
            TypedTokenKind::InfixOperator(s) => s.clone(),
            TypedTokenKind::LParen => "(".to_string(),
            TypedTokenKind::RParen => ")".to_string(),
        })
        .collect()
}

#[test]
fn only_variable() {
    let mut program = Program::new();
    let symbol_table = SymbolTable::new();
    let mut ec = ErrorCollector::new();

    let int_type = symbol_path![SymbolPathComponent::TypeDef("Int".to_string())];
    program.set_int_literal(&mut ec, int_type, Range::zero());

    let expr_tokens = vec![ExprToken {
        kind: ExprTokenKind::IntLiteral(5),
        range: Range::zero(),
    }];

    // Convert the token to TypedToken, and then rearrange it to RPN
    let typed_tokens = match get_typed_tokens(&mut ec, &program, &symbol_table, &expr_tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't convert tokens to typed tokens:\n{:#?}", ec),
    };
    let res = match rearrange_tokens_to_rpn(&mut ec, &program, typed_tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't rearrange tokens to RPN order:\n{:#?}", ec),
    };

    let got = short_repr(&res);
    let expected = vec!["V<Int>"];
    assert_eq!(got, expected);
}

#[test]
fn simple_subtraction() {
    // a - b - c
    // => (a - b) - c
    // => RPN: a b - c -
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_infix_operator(
        "-".to_string(),
        InfixOperatorProperties {
            precedence: 10,
            associativity: OperatorAssociativity::Left,
        },
    );

    let tokens = vec![v(), inf("-"), v(), inf("-"), v()];
    let res = match rearrange_tokens_to_rpn(&mut ec, &program, tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't rearrange tokens to RPN order:\n{:#?}", ec),
    };

    let got = short_repr(&res);
    let want = vec!["V<CompInt>", "V<CompInt>", "-", "V<CompInt>", "-"];
    assert_eq!(got, want);
}

#[test]
fn sub_and_mul() {
    // a - b * c
    // => a - (b * c)
    // => RPN: a b c * -
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_infix_operator(
        "-".to_string(),
        InfixOperatorProperties {
            precedence: 10,
            associativity: OperatorAssociativity::Left,
        },
    );
    program.register_infix_operator(
        "*".to_string(),
        InfixOperatorProperties {
            precedence: 20,
            associativity: OperatorAssociativity::Left,
        },
    );

    let tokens = vec![v(), inf("-"), v(), inf("*"), v()];
    let res = match rearrange_tokens_to_rpn(&mut ec, &program, tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't rearrange tokens to RPN order:\n{:#?}", ec),
    };

    let got = short_repr(&res);
    let want = vec!["V<CompInt>", "V<CompInt>", "V<CompInt>", "*", "-"];
    assert_eq!(got, want);
}

#[test]
fn prefix_and_infix() {
    // -a * b + c
    // => ((-a) * b) + c
    // => RPN: a pre- b * c +
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_prefix_operator("-".to_string());
    program.register_infix_operator(
        "*".to_string(),
        InfixOperatorProperties {
            precedence: 20,
            associativity: OperatorAssociativity::Left,
        },
    );
    program.register_infix_operator(
        "+".to_string(),
        InfixOperatorProperties {
            precedence: 10,
            associativity: OperatorAssociativity::Left,
        },
    );

    let tokens = vec![pre("-"), v(), inf("*"), v(), inf("+"), v()];
    let res = match rearrange_tokens_to_rpn(&mut ec, &program, tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't rearrange tokens to RPN order:\n{:#?}", ec),
    };

    let got = short_repr(&res);
    let want = vec!["V<CompInt>", "pre-", "V<CompInt>", "*", "V<CompInt>", "+"];
    assert_eq!(got, want);
}

// --- Error case tests ---

#[test]
fn non_associative_chain_error() {
    // a < b < c where '<' is non-associative should error OperatorCannotBeChained
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_infix_operator(
        "<".to_string(),
        InfixOperatorProperties {
            precedence: 5,
            associativity: OperatorAssociativity::None,
        },
    );

    // Chaining operator with associativity "None", which should cause an error
    let tokens = vec![v(), inf("<"), v(), inf("<"), v()];
    rearrange_tokens_to_rpn(&mut ec, &program, tokens);
    assert!(ec.has_error());
    assert!(ec.has_error_kind(EK::OpCannotBeChained, Pl::Str("<".to_string())));
}

#[test]
fn unmatched_parentheses_detected_on_drain() {
    // (a + b  -- missing closing paren -> should error UnmatchedParentheses on final drain
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_infix_operator(
        "+".to_string(),
        InfixOperatorProperties {
            precedence: 10,
            associativity: OperatorAssociativity::Left,
        },
    );

    // Expression with no closing pharenthesis
    let tokens = vec![lpar(), v(), inf("+"), v()];
    rearrange_tokens_to_rpn(&mut ec, &program, tokens);
    assert!(ec.has_error());
    assert!(ec.has_error_kind(EK::UnmatchedParentheses, Pl::None));
}

#[test]
fn unmatched_parentheses_right_paren_error() {
    // a + b )  -- extra right paren should be detected when encountering RParen
    let mut program = Program::new();
    let mut ec = ErrorCollector::new();

    program.register_infix_operator(
        "+".to_string(),
        InfixOperatorProperties {
            precedence: 10,
            associativity: OperatorAssociativity::Left,
        },
    );

    let tokens = vec![v(), inf("+"), v(), rpar()];
    rearrange_tokens_to_rpn(&mut ec, &program, tokens);
    assert!(ec.has_error());
    assert!(ec.has_error_kind(EK::UnmatchedParentheses, Pl::None));
}

#[test]
fn operator_not_found_error() {
    // Using an infix operator with no registered properties should return CompilerBug
    let program = Program::new();
    let mut ec = ErrorCollector::new();

    let tokens = vec![v(), inf("$unknown$"), v()];
    rearrange_tokens_to_rpn(&mut ec, &program, tokens);
    assert!(ec.has_error());
    assert!(ec.has_error_kind(EK::OperatorNotFound, Pl::Str("$unknown$".to_string())));
}

#[test]
fn complex_expression_test() {
    // Expression: `(foo_bar(a + 2) * -b) - (c ^ (d + e))`
    // Expected RPN: `foo_bar b pre- * c d e + ^ -` (as a string representation)
    //
    // This test stress-tests the integration of the parser, get_typed_tokens,
    // and rearrange_tokens_to_rpn.

    // 1. --- Setup ---
    let mut program = Program::new();
    let mut symbol_table = SymbolTable::new();
    let mut ec = ErrorCollector::new();

    // Set types for literals and variables
    let int_type = symbol_path![SymbolPathComponent::TypeDef("Int".to_string())];
    // let float_type = symbol_path![SymbolPathComponent::TypeDef("Float".to_string())];
    program.set_int_literal(&mut ec, int_type.clone(), Range::zero());

    // Build a symbol table by parsing a small program that declares the needed symbols.
    // Use top-level inputs and a valid function name (no dot in identifier).
    let program_src = r#"
operator infix + {
    precedence: 10,
    associativity: left
}
func infix + (lhs: Int, rhs: Int) -> Int {}

operator infix - {
    precedence: 10,
    associativity: left
}
func infix - (lhs: Float, rhs: Int) -> Float {}

operator infix * {
    precedence: 20,
    associativity: left
}
func infix * (lhs: Float, rhs: Int) -> Float {}

operator infix ^ {
    precedence: 30,
    associativity: right
}
func infix ^ (lhs: Int, rhs: Int) -> Int {}

operator prefix -
func prefix - (value: Int) -> Int {}

struct Int {}
struct Float {}
func foo_bar(x: Int) -> Float { }
input a: Int = 0
input b: Int = 0
input c: Int = 0
input d: Int = 0
input e: Int = 0
"#;

    let parsed_program = kasl::kasl_parser::parse(program_src)
        .unwrap_or_else(|e| panic!("Failed to parse helper program: {}", e));
    build_symbol_table(&mut ec, &mut symbol_table, &parsed_program);
    collect_all_types(&mut program, &symbol_table);
    resolve_types(&mut ec, &mut program, &symbol_table);

    // 2. --- Parsing ---
    // Parse the string directly using the `kasl_parser::expression` rule
    let expr_str = "(foo_bar(a + 2) * -b) - (c ^ (d + e))";
    let expr_tokens = kasl::kasl_parser::oneline_expression(expr_str)
        .unwrap_or_else(|e| panic!("Parser failed: {}", e));

    // 3. --- Typing & RPN Conversion ---
    let typed_tokens = match get_typed_tokens(&mut ec, &program, &symbol_table, &expr_tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't convert tokens to typed tokens:\n{:#?}", ec),
    };

    let rpn_tokens = match rearrange_tokens_to_rpn(&mut ec, &program, typed_tokens) {
        Some(tokens) => tokens,
        None => panic!("Couldn't convert typed tokens to RPN:\n{:#?}", ec),
    };

    // 4. --- Validation ---
    let got = short_repr(&rpn_tokens);
    let want = vec![
        "V<Float>", // Result of foo.bar(a + 2)
        "V<Int>",   // b
        "pre-",     // - (prefix)
        "*",        // *
        "V<Int>",   // c
        "V<Int>",   // d
        "V<Int>",   // e
        "+",        // +
        "^",        // ^
        "-",        // - (infix)
    ];

    assert_eq!(got, want, "The RPN sequence did not match the expectation.");

    let expr_result = build_expr_tree_from_rpn(&mut ec, &program, &symbol_table, rpn_tokens);
    match expr_result {
        Some(_) => (),
        None => panic!("Couldn't build expression tree from the tokens:\n{:#?}", ec),
    }
}
