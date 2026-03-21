use crate::{FunctionID, ParserScopeStmt};
use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize)]
pub struct FuncBodyMap {
    pub func_map: HashMap<FunctionID, Vec<ParserScopeStmt>>,
}

impl FuncBodyMap {
    pub fn register(&mut self, func_id: FunctionID, body: Vec<ParserScopeStmt>) {
        self.func_map.insert(func_id, body);
    }

    pub fn get_body(&self, func_id: &FunctionID) -> Option<&Vec<ParserScopeStmt>> {
        self.func_map.get(func_id)
    }
}
