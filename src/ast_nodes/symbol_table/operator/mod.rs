//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
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

use crate::{
    ast_nodes::OperatorID,
    error::{EK, ErrorKind},
};
use hashbrown::{HashMap, hash_map::Entry};

#[derive(Debug, Default, Clone, serde::Serialize)]
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

    next_operator_id: usize,
}

impl OperatorContext {
    pub fn generate_operator_id(&mut self) -> OperatorID {
        let id = OperatorID(self.next_operator_id);
        self.next_operator_id += 1;
        id
    }

    // --- REGISTER FUNCTIONS ---

    /// Registers an infix operator and returns its ID.
    pub fn register_infix_func(&mut self, infix: InfixOperator) -> Result<OperatorID, ErrorKind> {
        let id = self.generate_operator_id();
        // Construct an infix query
        let query = InfixQuery {
            symbol: infix.symbol.clone(),
            lhs_type: infix.lhs.value_type,
            rhs_type: infix.rhs.value_type,
        };
        match self.infix_ids.entry(query) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(id);
                // Insert the operator to the operators map
                self.infix_operators.insert(id, infix);
                Ok(id)
            }
            Entry::Occupied(_) => Err(EK::DuplicateInfixFunc),
        }
    }

    pub fn register_infix_properties(
        &mut self,
        symbol: String,
        properties: InfixOperatorProperties,
    ) -> Result<(), ErrorKind> {
        match self.infix_operator_properties.entry(symbol) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(properties);
                Ok(())
            }
            Entry::Occupied(_) => Err(EK::DuplicateInfixDefine),
        }
    }

    /// Registers an prefix operator and returns its ID.
    pub fn register_prefix_func(
        &mut self,
        prefix: PrefixOperator,
    ) -> Result<OperatorID, ErrorKind> {
        let id = self.generate_operator_id();
        // Construct a prefix query
        let query = PrefixQuery {
            symbol: prefix.symbol.clone(),
            operand_type: prefix.operand.value_type,
        };
        match self.prefix_ids.entry(query) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(id);
                // Insert the operator to the operators map
                self.prefix_operators.insert(id, prefix);
                Ok(id)
            }
            Entry::Occupied(_) => Err(EK::DuplicatePrefixFunc),
        }
    }

    pub fn register_prefix_properties(
        &mut self,
        symbol: String,
        properties: PrefixOperatorProperties,
    ) -> Result<(), ErrorKind> {
        match self.prefix_operator_properties.entry(symbol) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(properties);
                Ok(())
            }
            Entry::Occupied(_) => Err(EK::DuplicatePrefixDefine),
        }
    }

    /// Registers an postfix operator and returns its ID.
    pub fn register_postfix_func(
        &mut self,
        postfix: PostfixOperator,
    ) -> Result<OperatorID, ErrorKind> {
        let id = self.generate_operator_id();
        // Construct a postfix query
        let query = PostfixQuery {
            symbol: postfix.symbol.clone(),
            operand_type: postfix.operand.value_type,
        };
        match self.postfix_ids.entry(query) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(id);
                // Insert the operator to the operators map
                self.postfix_operators.insert(id, postfix);
                Ok(id)
            }
            Entry::Occupied(_) => Err(EK::DuplicatePostfixFunc),
        }
    }

    pub fn register_postfix_properties(
        &mut self,
        symbol: String,
        properties: PostfixOperatorProperties,
    ) -> Result<(), ErrorKind> {
        match self.postfix_operator_properties.entry(symbol) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(properties);
                Ok(())
            }
            Entry::Occupied(_) => Err(EK::DuplicatePostfixDefine),
        }
    }

    // --- PROPERTIES GETTER FUNCTIONS ---

    pub fn get_infix_props(&self, symbol: &str) -> Option<&InfixOperatorProperties> {
        self.infix_operator_properties.get(symbol)
    }

    pub fn get_prefix_props(&self, symbol: &str) -> Option<&PrefixOperatorProperties> {
        self.prefix_operator_properties.get(symbol)
    }

    pub fn get_postfix_props(&self, symbol: &str) -> Option<&PostfixOperatorProperties> {
        self.postfix_operator_properties.get(symbol)
    }

    // --- ID GETTER FUNCTIONS ---

    pub fn get_infix_id(&self, query: InfixQueryRef) -> Option<OperatorID> {
        self.infix_ids.get(&query).copied()
    }

    pub fn get_prefix_id(&self, query: PrefixQueryRef) -> Option<OperatorID> {
        self.prefix_ids.get(&query).copied()
    }

    pub fn get_postfix_id(&self, query: PostfixQueryRef) -> Option<OperatorID> {
        self.postfix_ids.get(&query).copied()
    }

    // --- OPERATOR FUNC GETTER FUNCTIONS ---

    pub fn get_infix_func(&self, id: &OperatorID) -> Option<&InfixOperator> {
        self.infix_operators.get(id)
    }

    pub fn get_prefix_func(&self, id: &OperatorID) -> Option<&PrefixOperator> {
        self.prefix_operators.get(id)
    }

    pub fn get_postfix_func(&self, id: &OperatorID) -> Option<&PostfixOperator> {
        self.postfix_operators.get(id)
    }

    // --- OPERATOR MUTABLE FUNC GETTER FUNCTIONS ---

    pub fn get_infix_func_mut(&mut self, id: &OperatorID) -> Option<&mut InfixOperator> {
        self.infix_operators.get_mut(id)
    }

    pub fn get_prefix_func_mut(&mut self, id: &OperatorID) -> Option<&mut PrefixOperator> {
        self.prefix_operators.get_mut(id)
    }

    pub fn get_postfix_func_mut(&mut self, id: &OperatorID) -> Option<&mut PostfixOperator> {
        self.postfix_operators.get_mut(id)
    }

    // --- ALL IDS GETTER FUNCTIONS ---

    pub fn all_infix_ids(&self) -> Vec<OperatorID> {
        self.infix_operators.keys().copied().collect()
    }

    pub fn all_prefix_ids(&self) -> Vec<OperatorID> {
        self.prefix_operators.keys().copied().collect()
    }

    pub fn all_postfix_ids(&self) -> Vec<OperatorID> {
        self.postfix_operators.keys().copied().collect()
    }
}
