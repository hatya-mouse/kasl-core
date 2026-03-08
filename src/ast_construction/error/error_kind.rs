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
    /// Payload: The name of the struct field that is defined in the top level
    /// Struct field is defined in the top level though it should be defined in a struct.
    TopLevelStructField,

    /// Title: VarNotFound
    /// Payload: The name of the variable that is not found
    /// Variable is not found.
    VarNotFound,

    /// Title: FuncNotFound
    /// Payload: The name of the function that is not found
    /// Function is not found.
    FuncNotFound,

    /// Title: PrefixOpNotFound
    /// Payload: The symbol of the prefix operator that is not found
    /// Prefix operator is not found.
    PrefixOpNotFound,

    /// Title: InfixOrPostfixOpNotFound
    /// Payload: The symbol of the infix or postfix operator that is not found
    /// Infix or postfix operator is not found.
    InfixOrPostfixOpNotFound,

    /// Title: OpNotAssociative
    /// Payload: The symbol of the operator that is not associative
    /// Non-associative operator is used consecutively.
    OpNotAssociative,

    /// Title: NoReturnFuncInExpr
    /// Payload: The name of the function that has no return type
    /// Function without return type is used in an expression.
    NoReturnFuncInExpr,

    /// Title: MemberAccessOnPrimitive
    /// Payload: None
    /// Member access expression on a primitive type.
    MemberAccessOnPrimitive,

    /// Title: MemberFieldNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member field of the struct is not found.
    MemberFieldNotFound,

    /// Title: MemberFuncNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member function of the struct is not found.
    MemberFuncNotFound,

    /// Title: ArgOrderIncorrect
    /// Payload: The name of the function and the name of the argument that is out of order
    /// Argument order is incorrect.
    ArgOrderIncorrect,

    /// Title: DuplicateArg
    /// Payload: The name of the function and the name of the duplicate argument
    /// The same argument is given more than once.
    DuplicateArg,

    /// Title: ExtraArg
    /// Payload: The name of the function
    /// Too many arguments are given.
    ExtraArg,

    /// Title: MissingArg
    /// Payload: The name of the function
    /// Not enough arguments are given.
    MissingArg,

    /// Title: MissingArgLabel
    /// Payload: The name of the function
    /// A label of the argument is missing, but the argument requires a label.
    MissingArgLabel,

    /// Title: TypeAnnotationMismatch
    /// Payload: The type of the annotation and the type of the expression
    /// The type annotation does not match the type of the expression.
    TypeAnnotationMismatch,

    /// Title: InvalidStructStmt
    /// Payload: The kind of the statement that is invalid
    /// An unexpected statement was found in the struct body.
    InvalidStructStmt,

    /// Title: TypeNotFound
    /// Payload: The name of the type that is not found
    /// The type is not found in the type registry.
    TypeNotFound,

    /// Title: NoTypeAnnotationOrDefVal
    /// Payload: None
    /// Both type annotation and default value are missing for the parameter.
    NoTypeAnnotationOrDefVal,

    /// Title: GlobalFuncCannotBeStatic
    /// Payload: The name of the function
    /// `static` keyword is applied to a global function.
    GlobalFuncCannotBeStatic,

    /// Title: WrongParamCountForInfix
    /// Payload: The symbol of the operator and the number of parameters
    /// An infix operator definition has a wrong number of parameters.
    WrongParamCountForInfix,

    /// Title: WrongParamCountForPrefix
    /// Payload: The symbol of the operator and the number of parameters
    /// A prefix operator definition has a wrong number of parameters.
    WrongParamCountForPrefix,

    /// Title: WrongParamCountForPostfix
    /// Payload: The symbol of the operator and the number of parameters
    /// A postfix operator definition has a wrong number of parameters.
    WrongParamCountForPostfix,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
