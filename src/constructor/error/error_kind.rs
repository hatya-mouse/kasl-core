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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Title: DuplicateSymbol
    /// Phase: SymbolTableConstruction
    /// Payload: Name of the duplicate symbol
    DuplicateSymbol,

    /// Title: InvalidTopExpr
    /// Phase: SymbolTableConstruction
    /// Payload: Type of the expression
    /// * Throwed if there's a ParserTopLevelExpr with invalid type. This doesn't mean that the expression if in the top level.
    InvalidTopExpr,

    /// Title: InvalidParamNumbersForInfix
    /// Phase: Validation
    /// Payload: None
    InvalidParamNumbersForInfix,

    /// Title: InvalidParamNumbersForPrefix
    /// Phase: Validation
    /// Payload: None
    InvalidParamNumbersForPrefix,

    /// Title: DuplicateLiteralBind
    /// Phase: TypeResolution
    /// Payload: Type of the duplicate literal bind
    DuplicateLiteralBind,

    /// Title: VariableNotFound
    /// Phase: TypeResolution
    /// Payload: Path of the variable which could not be found
    VariableNotFound,

    /// Title: FunctionNotFound
    /// Phase: TypeResolution
    /// Payload: Path of the function which could not be found
    FunctionNotFound,

    /// Title: OperatorNotFound
    /// Phase: TypeResolution
    /// Payload: Path of the operator which could not be found
    OperatorNotFound,

    /// Title: TypeNotFound
    /// Phase: TypeResolution
    /// Payload: Path of the type which could not be found
    TypeNotFound,

    /// Title: DependencyCycle
    /// Phase: TypeResolution
    /// Payload: Path to the cycle symbols
    DependencyCycle,

    /// Title: NoLiteralBind
    /// Phase: TypeResolution
    /// Payload: Type of the literal bind
    NoLiteralBind,

    /// Title: OpCannotBeChained
    /// Phase: TypeResolution
    /// Payload: None
    /// Caused when the operator with the associativity of none is chained.
    OpCannotBeChained,

    /// Title: UnmatchedParentheses
    /// Phase: TypeResolution
    /// Payload: None
    UnmatchedParentheses,

    /// Title: ArityMismatch
    /// Phase: TypeResolution
    /// Payload: Expected and actual arity
    ArityMismatch,

    /// Title: InvalidExprSyntax
    /// Phase: TypeResolution
    /// Payload: None
    InvalidExprSyntax,

    /// Title: ParamWithoutType
    /// Phase: TypeResolution
    /// Payload: Name of the parameter which doesn't have a type annotation
    ParamWithoutType,

    /// Title: OpCannotHaveDefaultValue
    /// Phase: TypeResolution
    /// Payload: Name of the operator which has a default value
    OpCannotHaveDefaultValue,

    /// Title: TypeMismatch
    /// Phase: TypeResolution
    /// Payload: Type from the type annotation and the type of the default value
    TypeMismatch,

    /// Title: ParamNotFound
    /// Phase: StatementBuilding
    /// Payload: Function path, parameter label
    ParamNotFound,

    /// Title: TooManyParams
    /// Phase: StatementBuilding
    /// Payload: Function path, maximum number of parameters, actual number of parameters
    TooManyParams,

    /// Title: NotEnoughParams
    /// Phase: StatementBuilding
    /// Payload: Function path, required number of parameters, actual number of parameters
    NotEnoughParams,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
