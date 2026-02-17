//
// © 2025 Shuntaro Kasatani
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

use kasl::kasl_parser;

#[test]
fn complex_program_parses() {
    let program = r#"
            // Inputs / Outputs
            input sr: Int = 44100
            input gain = 0.8
            output out_signal: Float = 0.0

            // State
            state {
                buffer: Float = 0.0
                index: Int = 0
            }

            // Protocol for effects
            protocol Effect {
                func process(_ in_value: Float) -> Float
            }

            // Delay struct implementing Effect
            struct Delay: Effect {
                var feedback: Float = 0.5

                init(feedback: Float) {
                    self.feedback = feedback
                }

                Effect func process(_ in_value: Float) -> Float {
                    return in_value * self.feedback
                }
            }

            // Wrapper types with literal bindings
            struct Int {
                var raw: CompInt = CompInt()

                // Bind integer literal to this type
                intliteral init(_ value: CompInt) {
                    self.raw = value
                }
            }

            struct Float {
                var raw: CompFloat = 0.0

                floatliteral init(_ value: CompFloat) {
                    self.raw = value
                }
            }

            // Global operator properties
            operator infix ** {
                associativity: right,
                precedence: 3
            }

            // Specific impl for Int ** Int
            func infix **(lhs: Int, rhs: Int) -> Int {
                return lhs * rhs
            }

            // Define a prefix operator (unary minus)
            operator prefix -

            func prefix -(operand: Float) -> Float {
                return operand * -1.0
            }

            // Another infix operator to check precedence differences
            operator infix + {
                associativity: left,
                precedence: 1
            }

            func infix +(lhs: Float, rhs: Float) -> Float {
                return lhs + rhs
            }

            // Test functions and main
            func multiply_ints(_ a: Int, _ b: Int) -> Int {
                return a * b
            }

            func main() {
                // literal usage: int and float
                var a = 2       // Int literal -> Int (via intliteral binding)
                var b: Int = 3
                var p = a ** b  // test infix ** with precedence

                var f = 2.5     // Float literal -> Float
                var g = -gain   // prefix operator on identifier

                // mixing: ensure + has lower precedence than **
                var mix = 1 + 2 ** 3 + 4

                // call effect
                var d = Delay(feedback: 0.7)
                out_signal = d.process(1.0)

                // ensure functions parse
                var prod = multiply_ints(a, b)
            }

            // A separate struct implementing the protocol
            struct Multiplier: Effect {
                var value: Int = 1

                init(_ value: Int) {
                    self.value = value
                }

                Effect func process(_ in_value: Float) -> Float {
                    return in_value * Float(self.value)
                }
            }
        "#;

    let parsed = kasl_parser::parse(program);
    assert!(parsed.is_ok(), "parser failed: {:?}", parsed.err());
}
