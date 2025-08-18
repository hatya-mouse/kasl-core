# Compilation Order

AudioShader processes code in the following order:
1. **`wrap` Definitions**:
   - All `wrap` definitions are processed first to register types and their operations.
   - This ensures that mutual dependencies between types (e.g., `Int` and `Float`) are resolved.

2. **Struct Functions**:
   - Functions defined within structs are processed next.

3. **User's Functions**:
   - Functions defined by the user are processed after struct functions.

4. **`main()` Function**:
   - The `main()` function is compiled last, after all types and functions have been registered.

# Protocol Function Resolving
Once compiler finds an struct member function call, it will resolve the function by looking up the struct's method table.

```AudioShader
struct Cat: Meow {
    Meow func meow() {
        raw.meow()
        return 0
    }
}

func main() {
    Cat().meow()
}
```

# Compiler Types

## `CompInt`
`CompInt` is a basic type used to represent integer values in the compiler.

Every integer literal will be recognized as a `CompInt` value.

## `CompFloat`
`CompFloat` is a basic type used to represent 32-bit floating point values in the compiler.

Every floating point literal will be recognized as a `CompFloat` value.

## `CompBool`
`CompBool` is a basic type used to represent boolean values in the compiler.

Every boolean literal (`true` or `false`) will be recognized as a `CompBool` value.

# Compiler Functions

## `CompInt`
- `compIntAdd()`
- `compIntSub()`
- `compIntMul()`
- `compIntDiv()`
- `compIntMod()`
- `compIntEq()`
- `compIntNe()`
- `compIntLt()`
- `compIntLe()`
- `compIntGt()`
- `compIntGe()`
- `compIntBitAnd()`
- `compIntBitOr()`
- `compIntBitXor()`
- `compIntBitNot()`
- `compIntShl()`
- `compIntShr()`
- `compIntAsFloat()`

## `CompFloat`
- `compFloatAdd()`
- `compFloatSub()`
- `compFloatMul()`
- `compFloatDiv()`
- `compFloatMod()`
- `compFloatEq()`
- `compFloatNe()`
- `compFloatLt()`
- `compFloatLe()`
- `compFloatGt()`
- `compFloatGe()`
- `compFloatAsInt()`

## `CompBool`
- `compBoolAnd()`
- `compBoolOr()`
- `compBoolNot()`

# Builtin Type Declarations

## `Int`
```AudioShader
struct Int: Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Le, Gt, Ge, BitAnd, BitOr, BitXor, BitNot, Shl, Shr {
    v: CompInt

    init(_ v: CompInt) {
        self.v = v
    }

    Add func add(_ rhs: Int) -> Int {
        Int(compIntAdd(v, rhs.v))
    }

    Sub func sub(_ rhs: Int) -> Int {
        Int(compIntSub(v, rhs.v))
    }

    Mul func mul(_ rhs: Int) -> Int {
        Int(compIntMul(v, rhs.v))
    }

    Div func div(_ rhs: Int) -> Int {
        Int(compIntDiv(v, rhs.v))
    }
}
```
