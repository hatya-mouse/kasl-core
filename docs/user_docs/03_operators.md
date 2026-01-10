# 3. Custom Operators

In KASL, you are not limited to a predefined set of operators. You can declare your own custom operators and provide implementations for operand type combinations. This allows for expressive and readable code, especially when working with custom data structures like vectors, matrices, or complex numbers. In fact, all of KASL's built-in operators are declared and implemented using this very same syntax.

There are two kinds of operators in KASL:

* **`prefix`**: An operator that comes before a single operand (e.g., `-x`).
* **`infix`**: An operator that comes between two operands (e.g., `x + y`).

> **Note**: KASL does not support `postfix` operators for now (e.g., `x++`).

Defining and using a new operator is a two-step process:
1. **Declare its syntax properties**: Use the `operator` keyword to declare the operator's symbol, kind (`infix` or `prefix`), precedence and (for infix) associativity. This is a global declaration that tells the parser how to treat the symbol in expressions.
2. **Provide the implementation**: Use the `func` keyword to provide the actual code for the operator for specific operand types. This is how operator overloading is achieved; the compiler selects the appropriate `func` by operand types at compile time.

---

## 1. Declaring Operator Properties (`operator`)

Before you can use an operator, you must first declare its fundamental properties.

The syntax is as follows:

```kasl/docs/user_guide/03_operators.md#L301-360
// For infix operators
operator infix <symbol> {
    precedence: <integer>,
    associativity: <left | right | none>
}

// For prefix operators
operator prefix <symbol> {
    precedence: <integer>
}
```

* **`<symbol>`**: The character(s) for your operator, like `+`, `*`, or even `|>` or `^^`.
* **`precedence`**: An integer that determines the order of operations. Operators with higher precedence are evaluated first. For reference, multiplication (`*`) typically has a higher precedence than addition (`+`).
* **`associativity`**: (Infix only) Determines how operators of the same precedence are grouped in the absence of parentheses.
  * `left`: Left-associative. Example: `a - b - c` → `(a - b) - c`.
  * `right`: Right-associative. Example: `a ^ b ^ c` → `a ^ (b ^ c)`.
  * `none`: The operator cannot be chained. Example: `a < b < c` is a compile-time error; write `(a < b) && (b < c)` instead.

> **Note on Prefix Associativity**: The `associativity` property is only meaningful for `infix` operators. Specifying it for a `prefix` operator is a syntax error and will be rejected by the compiler.

### Example: Declaring an addition-like and a prefix operator

```kasl/docs/user_guide/03_operators.md#L361-404
// Declare an infix operator `+` with left-associativity.
// Precedence 10 (lower than multiplication, for example).
operator infix + {
    precedence: 10,
    associativity: left
}

// Declare a prefix negation operator `-`.
// Prefix operators typically have high precedence.
operator prefix - {
    precedence: 100
}
```

---

## 2. Providing Operator Implementations (`func`)

After an operator has been declared, it doesn't actually do anything yet. You must provide one or more `func` implementations for the operand type combinations you want to support. In KASL, operator implementations are defined at top-level using the `func` keyword; the compiler resolves which `func` to call by matching operand types.

### Infix Implementation

The syntax for implementing an `infix` operator:

```kasl/docs/user_guide/03_operators.md#L405-440
func infix <symbol>(lhs: <Type>, rhs: <Type>) -> <Return Type> {
    // function body
    // `lhs` is the left-hand side operand
    // `rhs` is the right-hand side operand
}
```

You can provide multiple `func` overloads for the same operator symbol but different type combinations. The compiler picks the best match based on the operand types in the expression.

### Prefix Implementation

The syntax for implementing a `prefix` operator:

```kasl/docs/user_guide/03_operators.md#L441-470
func prefix <symbol>(operand: <Type>) -> <Return Type> {
    // function body
    // `operand` is the value after the operator
}
```

---

## 3. Example: A Custom `Vector2` Struct

Let's see how this works by creating a `Vector2` struct and implementing the `+` and `-` (prefix) operators for it.

```kasl/docs/user_guide/03_operators.md#L471-540
// 0. Define the custom type using `struct` (KASL uses Swift-like struct syntax)
struct Vector2 {
    var x: Float = 0.0
    var y: Float = 0.0
}

// 1. Declare the global properties of the operators
operator infix + {
    precedence: 10,
    associativity: left
}

operator prefix - {
    precedence: 100
}

// 2. Implement the behavior of `+` for Vector2
func infix +(lhs: Vector2, rhs: Vector2) -> Vector2 {
    return Vector2 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y
    }
}

// 3. Implement the behavior of prefix `-` for Vector2
func prefix -(operand: Vector2) -> Vector2 {
    return Vector2 {
        x: -operand.x,
        y: -operand.y
    }
}

// 4. Usage
let v1 = Vector2 { x: 1.0, y: 2.0 }
let v2 = Vector2 { x: 3.0, y: 4.0 }

let v3 = v1 + v2  // Uses our `func infix +` -> v3 is { x: 4.0, y: 6.0 }
let v4 = -v1      // Uses our `func prefix -` -> v4 is { x: -1.0, y: -2.0 }
```
