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

mod infix_operator;
mod op_body_map;
mod postfix_operator;
mod prefix_operator;

pub use infix_operator::{
    InfixOperator, InfixOperatorProperties, InfixQuery, InfixQueryRef, OperatorAssociativity,
};
pub use op_body_map::OpBodyMap;
pub use postfix_operator::{
    PostfixOperator, PostfixOperatorProperties, PostfixQuery, PostfixQueryRef,
};
pub use prefix_operator::{PrefixOperator, PrefixOperatorProperties, PrefixQuery, PrefixQueryRef};

use crate::OperatorID;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct OperatorContext {
    infix_operator_properties: HashMap<String, InfixOperatorProperties>,
    infix_operators: HashMap<OperatorID, InfixOperator>,
    infix_ids: HashMap<InfixQuery, OperatorID>,

    prefix_operator_properties: HashMap<String, PrefixOperatorProperties>,
    prefix_operators: HashMap<OperatorID, PrefixOperator>,
    prefix_ids: HashMap<PrefixQuery, OperatorID>,

    postfix_operator_properties: HashMap<String, PostfixOperatorProperties>,
    postfix_operators: HashMap<OperatorID, PostfixOperator>,
    postfix_ids: HashMap<PostfixQuery, OperatorID>,
}

impl OperatorContext {
    // -- REGISTER FUNCTIONS --

    pub fn register_infix_func(&mut self, infix: InfixOperator, id: OperatorID) {
        self.infix_operators.insert(id, infix);
    }

    pub fn register_infix_properties(
        &mut self,
        symbol: String,
        properties: InfixOperatorProperties,
    ) {
        self.infix_operator_properties.insert(symbol, properties);
    }

    pub fn register_prefix_func(&mut self, prefix: PrefixOperator, id: OperatorID) {
        self.prefix_operators.insert(id, prefix);
    }

    pub fn register_prefix_properties(
        &mut self,
        symbol: String,
        properties: PrefixOperatorProperties,
    ) {
        self.prefix_operator_properties.insert(symbol, properties);
    }

    pub fn register_postfix_func(&mut self, postfix: PostfixOperator, id: OperatorID) {
        self.postfix_operators.insert(id, postfix);
    }

    pub fn register_postfix_properties(
        &mut self,
        symbol: String,
        properties: PostfixOperatorProperties,
    ) {
        self.postfix_operator_properties.insert(symbol, properties);
    }

    // -- PROPERTIES GETTER FUNCTIONS --

    pub fn get_infix_props(&self, symbol: &str) -> Option<&InfixOperatorProperties> {
        self.infix_operator_properties.get(symbol)
    }

    pub fn get_prefix_props(&self, symbol: &str) -> Option<&PrefixOperatorProperties> {
        self.prefix_operator_properties.get(symbol)
    }

    pub fn get_postfix_props(&self, symbol: &str) -> Option<&PostfixOperatorProperties> {
        self.postfix_operator_properties.get(symbol)
    }

    // -- ID GETTER FUNCTIONS --

    pub fn get_infix_id(&self, query: InfixQueryRef) -> Option<OperatorID> {
        self.infix_ids.get(&query).copied()
    }

    pub fn get_prefix_id(&self, query: PrefixQueryRef) -> Option<OperatorID> {
        self.prefix_ids.get(&query).copied()
    }

    pub fn get_postfix_id(&self, query: PostfixQueryRef) -> Option<OperatorID> {
        self.postfix_ids.get(&query).copied()
    }

    // -- OPERATOR FUNC GETTER FUNCTIONS --

    pub fn get_infix_op(&self, id: &OperatorID) -> Option<&InfixOperator> {
        self.infix_operators.get(id)
    }

    pub fn get_prefix_op(&self, id: &OperatorID) -> Option<&PrefixOperator> {
        self.prefix_operators.get(id)
    }

    pub fn get_postfix_op(&self, id: &OperatorID) -> Option<&PostfixOperator> {
        self.postfix_operators.get(id)
    }

    // -- OPERATOR FUNC MUTABLE GETTER FUNCTIONS --

    pub fn get_infix_op_mut(&mut self, id: &OperatorID) -> Option<&mut InfixOperator> {
        self.infix_operators.get_mut(id)
    }

    pub fn get_prefix_op_mut(&mut self, id: &OperatorID) -> Option<&mut PrefixOperator> {
        self.prefix_operators.get_mut(id)
    }

    pub fn get_postfix_op_mut(&mut self, id: &OperatorID) -> Option<&mut PostfixOperator> {
        self.postfix_operators.get_mut(id)
    }

    // -- ID GETTER --

    pub fn all_infix_ids(&self) -> Vec<OperatorID> {
        self.infix_ids.values().copied().collect()
    }

    pub fn all_prefix_ids(&self) -> Vec<OperatorID> {
        self.prefix_ids.values().copied().collect()
    }

    pub fn all_postfix_ids(&self) -> Vec<OperatorID> {
        self.postfix_ids.values().copied().collect()
    }
}
