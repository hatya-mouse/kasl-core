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

#[cfg(test)]
mod expression {
    use kasl::{
        ExprToken, ExprTokenKind, InfixOperatorProperties, OperatorAssociativity, Program, Range,
        SymbolPath, SymbolPathComponent, SymbolTable, TypedToken, TypedTokenKind, get_typed_tokens,
        resolution::expr_inference::rearrange_tokens_to_rpn, symbol_path,
    };

    fn v() -> TypedToken {
        TypedToken::new(TypedTokenKind::Value(SymbolPath::comp_int()), Range::zero())
    }

    fn inf(sym: &str) -> TypedToken {
        TypedToken::new(
            TypedTokenKind::InfixOperator(sym.to_string()),
            Range::zero(),
        )
    }

    fn pre(sym: &str) -> TypedToken {
        TypedToken::new(
            TypedTokenKind::PrefixOperator(sym.to_string()),
            Range::zero(),
        )
    }

    fn lpar() -> TypedToken {
        TypedToken::new(TypedTokenKind::LParen, Range::zero())
    }

    fn rpar() -> TypedToken {
        TypedToken::new(TypedTokenKind::RParen, Range::zero())
    }

    /// Convert the TypedToken sequence into a compact string representation
    /// for easy assertions.
    fn short_repr(tokens: &[TypedToken]) -> Vec<String> {
        tokens
            .iter()
            .map(|t| match &t.kind {
                TypedTokenKind::Value(ty) => format!("V<{}>", ty),
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

        let int_type = symbol_path![SymbolPathComponent::TypeDef("Int".to_string())];
        program.set_int_literal(int_type).unwrap();

        let expr_tokens = vec![ExprToken {
            kind: ExprTokenKind::IntLiteral(5),
            range: Range::zero(),
        }];

        // Convert the token to TypedToken, and then rearrange it to RPN
        let typed_tokens = get_typed_tokens(&program, &expr_tokens, &symbol_table)
            .unwrap_or_else(|e| panic!("Couldn't convert tokens to typed tokens:\n{}", e));
        let res = rearrange_tokens_to_rpn(&program, typed_tokens)
            .unwrap_or_else(|e| panic!("Couldn't rearrange tokens to RPN order:\n{}", e));

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
        program.register_infix_properties(
            "-".to_string(),
            InfixOperatorProperties {
                precedence: 10,
                associativity: OperatorAssociativity::Left,
            },
        );

        let tokens = vec![v(), inf("-"), v(), inf("-"), v()];
        let res =
            rearrange_tokens_to_rpn(&program, tokens).expect("Expected successful RPN rearrange");

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

        program.register_infix_properties(
            "-".to_string(),
            InfixOperatorProperties {
                precedence: 10,
                associativity: OperatorAssociativity::Left,
            },
        );
        program.register_infix_properties(
            "*".to_string(),
            InfixOperatorProperties {
                precedence: 20,
                associativity: OperatorAssociativity::Left,
            },
        );

        let tokens = vec![v(), inf("-"), v(), inf("*"), v()];
        let res =
            rearrange_tokens_to_rpn(&program, tokens).expect("Expected successful RPN rearrange");

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

        program.register_prefix_operator("-".to_string());
        program.register_infix_properties(
            "*".to_string(),
            InfixOperatorProperties {
                precedence: 20,
                associativity: OperatorAssociativity::Left,
            },
        );
        program.register_infix_properties(
            "+".to_string(),
            InfixOperatorProperties {
                precedence: 10,
                associativity: OperatorAssociativity::Left,
            },
        );

        let tokens = vec![pre("-"), v(), inf("*"), v(), inf("+"), v()];
        let res =
            rearrange_tokens_to_rpn(&program, tokens).expect("Expected successful RPN rearrange");

        let got = short_repr(&res);
        let want = vec!["V<CompInt>", "pre-", "V<CompInt>", "*", "V<CompInt>", "+"];
        assert_eq!(got, want);
    }

    // --- Error case tests ---

    #[test]
    fn non_associative_chain_error() {
        // a < b < c where '<' is non-associative should error OperatorCannotBeChained
        let mut program = Program::new();
        program.register_infix_properties(
            "<".to_string(),
            InfixOperatorProperties {
                precedence: 5,
                associativity: OperatorAssociativity::None,
            },
        );

        // Chaining operator with associativity "None", which should cause an error
        let tokens = vec![v(), inf("<"), v(), inf("<"), v()];
        let res = rearrange_tokens_to_rpn(&program, tokens);
        assert!(res.is_err());

        let err = res.err().unwrap();
        assert_eq!(
            err.error_type,
            kasl::ConstructorErrorType::OperatorCannotBeChained("<".to_string())
        );
    }

    #[test]
    fn unmatched_parentheses_detected_on_drain() {
        // (a + b  -- missing closing paren -> should error UnmatchedParentheses on final drain
        let mut program = Program::new();
        program.register_infix_properties(
            "+".to_string(),
            InfixOperatorProperties {
                precedence: 10,
                associativity: OperatorAssociativity::Left,
            },
        );

        // Expression with no closing pharenthesis
        let tokens = vec![lpar(), v(), inf("+"), v()];
        let res = rearrange_tokens_to_rpn(&program, tokens);
        assert!(res.is_err());

        let err = res.err().unwrap();
        assert_eq!(
            err.error_type,
            kasl::ConstructorErrorType::UnmatchedParentheses
        );
    }

    #[test]
    fn unmatched_parentheses_right_paren_error() {
        // a + b )  -- extra right paren should be detected when encountering RParen
        let mut program = Program::new();
        program.register_infix_properties(
            "+".to_string(),
            InfixOperatorProperties {
                precedence: 10,
                associativity: OperatorAssociativity::Left,
            },
        );

        let tokens = vec![v(), inf("+"), v(), rpar()];
        let res = rearrange_tokens_to_rpn(&program, tokens);
        assert!(res.is_err());

        let err = res.err().unwrap();
        assert_eq!(
            err.error_type,
            kasl::ConstructorErrorType::UnmatchedParentheses
        );
    }

    #[test]
    fn operator_not_found_error() {
        // Using an infix operator with no registered properties should return CompilerBug
        let program = Program::new();
        let tokens = vec![v(), inf("$unknown$"), v()];
        let res = rearrange_tokens_to_rpn(&program, tokens);
        assert!(res.is_err());

        let err = res.err().unwrap();
        match err.error_type {
            kasl::ConstructorErrorType::OperatorNotFound(_) => {}
            other => panic!("expected OperatorNotFound, got {:?}", other),
        }
    }
}
