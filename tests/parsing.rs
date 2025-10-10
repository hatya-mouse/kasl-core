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
mod parsing {
    use kash::{
        ExprToken, ParserSymbolPathComponent, Range, kash_parser, parser_ast::ExprTokenKind,
    };

    /// Test parsing of chained expressions.
    #[test]
    fn chaining() {
        let object = kash_parser::expression("object");
        // println!("{:#?}", object);
        assert_eq!(
            object,
            Ok(vec![ExprToken {
                range: Range::n(0, 6),
                kind: ExprTokenKind::Identifier(vec![ParserSymbolPathComponent {
                    range: Range::n(0, 6),
                    symbol: "object".to_string(),
                }])
            }])
        );

        let object_property = kash_parser::expression("object.property");
        // println!("{:#?}", object_property);
        assert_eq!(
            object_property,
            Ok(vec![ExprToken {
                range: Range::n(0, 15),
                kind: ExprTokenKind::Identifier(vec![
                    ParserSymbolPathComponent {
                        range: Range::n(0, 6),
                        symbol: "object".to_string()
                    },
                    ParserSymbolPathComponent {
                        range: Range::n(7, 15),
                        symbol: "property".to_string()
                    }
                ])
            }])
        );
    }

    /// Test parsing of simple statements.
    #[test]
    fn easy_program() {
        let program = "input integer: Int = 14
            input fac = 5
            output out_value: Int

            struct Multiplier {
                var value = 1

                init(_ value: Int) {
                    self.value = value
                }

                func multiply(_ another: Int) -> Int {
                    return value * another
                }
            }

            func main() {
                var multiplier = Multiplier()
                out_value = multiply(multiplier)
            }

            func multiply(_ multiplier: Multiplier) -> Int {
                return multiplier.value * fac
            }
        ";

        let parsed_program = kash_parser::parse(program);
        println!("{:#?}", parsed_program);
        assert!(parsed_program.is_ok());
    }

    // Test parsing of complex statements.
    #[test]
    fn complex_program() {
        let program = "// Input declarations
            input sample_rate: Int = 44100
            input gain = 0.8
            input delay_time: Float = 0.5 #range(0.0, 1.0)

            // Output declaration
            output processed_signal: Float

            // State block
            state {
                buffer: Float = 0.0
                index: Int = 0
            }

            // Struct declaration
            struct Delay: Effect {
                var feedback: Float = 0.5

                init(feedback: Float) {
                    self.feedback = feedback
                }

                Effect func process(_ in_value: Float) -> Float {
                    return in_value * self.feedback
                }
            }

            // Protocol declaration
            protocol Effect {
                func process(_ in_value: Float) -> Float
            }

            struct Int: CompInt {
                // Literal binding
                intliteral init(_ value: CompInt) {
                    self.raw = value
                }

                // Infix operator declaration
                infix **(rhs: Int) -> Int {
                    associativity: right,
                    priority: 2
                }: {
                    return self * rhs
                }

                // Postfix operator declaration
                postfix !() -> Bool {
                    return self == 0
                }
            }

            struct Float: CompFloat {
                // Literal binding
                floatliteral init(_ value: CompFloat) {
                    self.raw = value
                }

                // Prefix operator declaration
                prefix -() -> Float {
                    return self * -1.0
                }
            }

            // Main function
            func main() {
                var delay = Delay(feedback: 0.7)
                var input_signal: Float = 1.0

                // Apply delay effect
                processed_signal = delay.process(input_signal)

                // Test infix operator
                var power = 2 ** 3

                // Test prefix operator
                var inverted_gain = -gain

                // Test postfix operator
                var is_zero = power!
            }

            // Standalone function
            func multiply(_ a: Int, _ b: Int) -> Int {
                return a * b
            }

            // Protocol conformance
            struct Multiplier: Effect {
                var value: Int = 1

                init(_ value: Int) {
                    self.value = value
                }

                func process(_ in_value: Float) -> Float {
                    return in_value * Float(self.value)
                }
            }
            ";

        let parsed_program = kash_parser::parse(program);
        println!("{:#?}", parsed_program);
        assert!(parsed_program.is_ok());
    }
}
