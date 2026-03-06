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
    FuncCallArg, ParserBodyStmt, ParserFuncCallArg, Statement, SymbolPath,
    data::SymbolID,
    error::Phase,
    resolution::expr_inference::ExprTreeBuilder,
    stmt_building::{StmtBuildingCtx, function_graph::FunctionGraphEdge},
};

impl<'a> StmtBuildingCtx<'a> {
    pub fn build_func_call_stmt(
        &mut self,
        caller_id: SymbolID,
        parsed_stmts: &mut Vec<Statement>,
        stmt: &ParserBodyStmt,
        path: &SymbolPath,
        args: &[ParserFuncCallArg],
    ) {
        let func_id = match self
            .program
            .get_id_by_path(path)
            .and_then(|ids| ids.first())
        {
            Some(parsed_target) => parsed_target,
            None => {
                self.ec
                    .func_not_found(stmt.range, Phase::StatementBuilding, &path.to_string());
                return;
            }
        };
        let target_func = match self.program.get_func(func_id) {
            Some(func) => func,
            None => {
                self.ec
                    .func_not_found(stmt.range, Phase::StatementBuilding, &path.to_string());
                return;
            }
        };

        // Push an edge to the function graph
        let edge = FunctionGraphEdge::new(caller_id, *func_id);
        self.function_graph.push(edge);

        // Check if the number of arguments is within the valid range
        let minimum_num = target_func.min_num_of_params();
        let maximum_num = target_func.max_num_of_params();
        let actual_num = args.len();

        // Throw an error if the number of arguments is out of range
        if actual_num < minimum_num {
            self.ec.not_enough_params(
                stmt.range,
                Phase::StatementBuilding,
                &path.to_string(),
                minimum_num,
                actual_num,
            );
            return;
        }

        if actual_num > maximum_num {
            self.ec.too_many_params(
                stmt.range,
                Phase::StatementBuilding,
                &path.to_string(),
                maximum_num,
                actual_num,
            );
            return;
        }

        // Get the name of the parameters
        let mut parsed_args = Vec::new();
        let i = 0;
        for arg in args {
            let parsed_value = match self.program.build_expr_tree_from_raw_tokens(
                self.ec,
                &arg.value,
                self.symbol_table,
            ) {
                Some(parsed_value) => parsed_value,
                None => return,
            };
            let arg_name = match &arg.label {
                Some(label) => match target_func.get_param_name_by_label(label) {
                    Some(name) => name,
                    None => {
                        self.ec.param_not_found(
                            stmt.range,
                            Phase::StatementBuilding,
                            &path.to_string(),
                            label,
                        );
                        return;
                    }
                },
                None => match target_func.get_param_name_by_index(i) {
                    Some(name) => name,
                    None => return,
                },
            };

            let parsed_arg = FuncCallArg {
                name: arg_name,
                value: parsed_value,
            };
            parsed_args.push(parsed_arg);
        }

        // Create a FuncCall statement
        let func_call_stmt = Statement::FuncCall {
            path: *func_id,
            args: parsed_args,
        };
        parsed_stmts.push(func_call_stmt);
    }
}
