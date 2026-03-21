use crate::{OperatorID, ParserScopeStmt};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct OpBodyMap {
    pub op_map: HashMap<OperatorID, Vec<ParserScopeStmt>>,
}

impl OpBodyMap {
    pub fn register(&mut self, op_id: OperatorID, body: Vec<ParserScopeStmt>) {
        self.op_map.insert(op_id, body);
    }

    pub fn get_body(&self, op_id: &OperatorID) -> Option<&Vec<ParserScopeStmt>> {
        self.op_map.get(op_id)
    }
}
