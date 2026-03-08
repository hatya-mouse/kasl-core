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
    /// Title: TopLevelStructField
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the struct field that is defined in the top level
    /// Struct field is defined in the top level though it should be defined in a struct.
    TopLevelStructField,

    /// Title: VarNotFound
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the variable that is not found
    /// Variable is not found.
    VarNotFound,

    /// Title: FuncNotFound
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function that is not found
    /// Function is not found.
    FuncNotFound,

    /// Title: PrefixOpNotFound
    /// Phase: GlobalDeclCollection
    /// Payload: The symbol of the prefix operator that is not found
    /// Prefix operator is not found.
    PrefixOpNotFound,

    /// Title: InfixOrPostfixOpNotFound
    /// Phase: GlobalDeclCollection
    /// Payload: The symbol of the infix or postfix operator that is not found
    /// Infix or postfix operator is not found.
    InfixOrPostfixOpNotFound,

    /// Title: OpNotAssociative
    /// Phase: GlobalDeclCollection
    /// Payload: The symbol of the operator that is not associative
    /// Non-associative operator is used consecutively.
    OpNotAssociative,

    /// Title: NoReturnFuncInExpr
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function that has no return type
    /// Function without return type is used in an expression.
    NoReturnFuncInExpr,

    /// Title: MemberAccessOnPrimitive
    /// Phase: GlobalDeclCollection
    /// Payload: None
    /// Member access expression on a primitive type.
    MemberAccessOnPrimitive,

    /// Title: MemberNotFound
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member is not found.
    MemberNotFound,

    /// Title: ArgOrderIncorrect
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function and the name of the argument that is out of order
    /// Argument order is incorrect.
    ArgOrderIncorrect,

    /// Title: DuplicateArg
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function and the name of the duplicate argument
    /// The same argument is given more than once.
    DuplicateArg,

    /// Title: ExtraArg
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function
    /// Too many arguments are given.
    ExtraArg,

    /// Title: MissingArg
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function
    /// Not enough arguments are given.
    MissingArg,

    /// Title: MissingArgLabel
    /// Phase: GlobalDeclCollection
    /// Payload: The name of the function
    /// A label of the argument is missing, but the argument requires a label.
    MissingArgLabel,

    /// Title: TypeAnnotationMismatch
    /// Phase: GlobalDeclCollection
    /// Payload: The type of the annotation and the type of the expression
    /// The type annotation does not match the type of the expression.
    TypeAnnotationMismatch,

    /// Title: InvalidStructStmt
    /// Phase: GlobalDeclCollection
    /// Payload: The kind of the statement that is invalid
    /// An unexpected statement was found in the struct body.
    InvalidStructStmt,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
