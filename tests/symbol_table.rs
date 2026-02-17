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

use kasl::{
    SymbolTable, error::ErrorCollector, kasl_parser, table_construction::build_symbol_table,
};

#[test]
fn table_generation() {
    let program = "input integer: Int = 14
        input fac = 5
        output out_value: Int = 0

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

    let parsed_program = kasl_parser::parse(program).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut ec = ErrorCollector::new();
    build_symbol_table(&mut ec, &mut symbol_table, &parsed_program);

    symbol_table.get_func("main");
}
