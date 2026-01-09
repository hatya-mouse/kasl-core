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
                        range: Range::n(6, 15),
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
            input fac = 5 #range(0, 100)
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
        // println!("{:#?}", parsed_program);
        assert!(parsed_program.is_ok());
    }
}
