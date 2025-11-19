# Syntax

## No Semicolons
AudioShader doesn't use semicolons. Instead, it uses line breaks to separate statements, which contributes to keep the code cleaner.

## Input Declaration
In AudioShader, you can easily define input variables using `input` keyword. Let's see:
```AudioShader
input in_stream: Stream // A very basic input declaration.
input gain: Float = 1.5 // Input can also have a default value.
input gain = 1.5 // If you specify a default value, you can omit the type!
```

### Input Parameters
Input declaration can take some Input Parameters for UI purposes, such as max and min values.

Input parameters are specified by `#` symbol followed by the name of the input parameter and optional parentheses for specifying the value.

Input parameters should be coming after the input declaration.

**Example:**
```AudioShader
input gain = 1.5 #range(0.0, 2.0) // This limits the range of the input value in the UI.
input gain_but_slider = 1.5 #slider #range(0.0, 2.0) // You can also write multiple parameters!
```

## Output Declaration
Output declaration is similar to input declaration, but it's much simpler and it uses `output` keyword instead of `input`.

Output declaration can't take any default value and parameters. You only need to specify the name and the type.

**Example:**
```AudioShader
output out_stream: Stream
```

## State
AudioShader has a **state** feature for declaring state variables. State variables are used to store values that persist across frames.

Note that state is only available within the **top level**!

```
state {
    delay_line = Stream()
}
```

State variables are kept while entire processing which is useful for making effects such as reverb.

## Functions
Functions are used to define reusable code blocks. They can take parameters and return values.

**Example:**
```AudioShader
func add(a: Float, b: Float) -> Float {
    return a + b
}
```

### Arguments
Functions can take arguments using the `func` keyword.

**Example:**
```AudioShader
func add(a: Float, b: Float) -> Float {
    return a + b
}
```

Function arguments can have default values. If provided, type can be omitted.

**Example:**
```AudioShader
func add(a: Float, b: Float = 0.0) -> Float {
    return a + b
}
```

With type omitted:
```AudioShader
func add(a: Float, b = 0.0) -> Float {
    return a + b
}
```

### Return Values
Functions can return values using the `return` keyword.

**Example:**
```AudioShader
func add(a: Float, b: Float) -> Float {
    return a + b
}
```

### `main()` Function
`main()` is called once per processing and it's responsible for processing the audio signal.

**Example:**
```AudioShader
func main() {
    out_stream = add(1.0, 2.0)
}
```

After the execution of `main()` function, every output variables must be assigned a value.

## Variable Declaration
Variables are declared using the `var` keyword.

You can omit the type if it can be inferred from the initializer just like the input declaration.

**Example:**
```AudioShader
var x: Float = 5 // Variable declaration.
var y = 10 // Variable declaration but the type is omitted.
```

## If
If statements are used to execute code conditionally.

**Example:**
```AudioShader
if (condition) {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
```

Here, `condition` is a boolean expression that evaluates to either `true` or `false`. For example: `a > b`

If statements can be nested, of course!

**Example:**
```AudioShader
if (condition1) {
    if (condition2) {
        // code to execute if both conditions are true
    } else {
        // code to execute if condition1 is true and condition2 is false
    }
} else {
    // code to execute if condition1 is false
}
```

## Comments
Comments are used to explain the code and make it easier to understand.

Everything starts with `//` is recognized as a comment and will be ignored by the compiler.

AudioShader doesn't have multi-line comments. However, you can use multiple single-line comments to replicate the functionality of multi-line comments.

**Example:**
```AudioShader
// This is a comment
// This is another comment
```

## Structs
Structs are used to group related data together.

Structs are defined using the `struct` keyword followed by the struct name and a list of fields.

**Example:**
```AudioShader
struct Point {
    x: Float
    y: Float
};
```

### Struct Functions
Structs can have functions associated with them.

**Example:**
```AudioShader
struct Point {
    x: Float
    y: Float

    init(x: Float, y: Float) {
        self.x = x
        self.y = y
    }

    func distance(other: Point) -> Float {
        return ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

func main() {
    var point1 = Point(x: 1.0, y: 2.0)
    var point2 = Point(x: 3.0, y: 4.0)
    var distance = point1.distance(point2) // Calculate the distance between point1 and point2
}
```

## Protocols
Protocols are a way to define a requirements of methods that a type must conform to.

```AudioShader
protocol Animal {
    func meow()
}

struct Cat: Animal {
    // Required by the Animal protocol
    Animal func meow() {
        // Cat meows
    }
}

struct Dog: Animal {
    // Required by the Animal protocol
    Animal func meow() {
        // Dog barks
    }
}
```

This is useful for creating a hierarchy of types that share common behavior.

```AudioShader
// You can pass protocol just like normal types
func meowAnimal(animal: Animal) {
    animal.meow()
}

func main() {
    var cat = Cat()
    var dog = Dog()

    // Use the same function for meowing animals
    meowAnimal(cat)
    meowAnimal(dog)
}
```

### Arithmetic Operators
In AudioShader, arithmetic operators are defined by the `Arithmetic` protocol.

**Definition of Arithmetic operators**
```AudioShader
protocol Arithmetic {
    // infix keyword means that the operator comes between the two operands.
    infix +(rhs: Self) -> Self
```

# Types

### `Float`
Basic float type for representing floating point numbers (32-bit).

### `Int`
Basic integer type for representing whole numbers.

### `Bool`
Basic boolean type for representing true or false values.

### `Array`
`Array` type has one type parameter. It can be defined by both `Array<Type>` and `[Type]`.

AudioShader `Array` doesn't specify the size of the array and you can dynamically resize it.
Also, you can define multi-dimensional arrays using the `Array` type!

### `Stream`
Stream is composed of multiple `[Float]`s.

`Stream` doesn't only represent a waveform; it may hold other data as well, such as frequency and elapsed time used in the Note Track.

**Example:** a `Stream` composed of `n` `Channel`s:
```
Stream {[ Channel #1 ], [ Channel #2 ] ... [ Channel #n ]}
```

### `Channel`
`Channel` represents a single audio channel. It can be used not only to represent audio signals, but also to represent other data such as frequency and elapsed time used in the Note Track.

# Builtin Functions

## Binary Operators

### `+`
`+` is used to add two numbers together. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 + 2 // 7
```

### `-`
`-` is used to subtract one number from another. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 - 2 // 3
```

### `*`
`*` is used to multiply two numbers together. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 * 2 // 10
```

### `/`
`/` is used to divide one number by another. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
6 / 2 // 3
```

### `%`
`%` is used to get the remainder of a division operation. It can only be used with `Int` types.

**Example:**
```AudioShader
5 % 2 // 1
```

### `==`
`==` is used to compare two values for equality. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 == 2 // false
```

### `!=`
`!=` is used to compare two values for inequality. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 != 2 // true
```

### `>`
`>` is used to compare two values for greater than. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 > 2 // true
```

### `<`
`<` is used to compare two values for less than. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 < 2 // false
```

### `>=`
`>=` is used to compare two values for greater than or equal to. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 >= 2 // true
```

### `<=`
`<=` is used to compare two values for less than or equal to. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
5 <= 2 // false
```

### `&&`
`&&` is used to perform a logical AND operation on two boolean values.

**Example:**
```AudioShader
true && false // false
```

### `||`
`||` is used to perform a logical OR operation on two boolean values.

**Example:**
```AudioShader
true || false // true
```

### `!`
`!` is used to perform a logical NOT operation on a boolean value.

**Example:**
```AudioShader
!true // false
```

### `&`
`&` is used to perform a bitwise AND operation on two `Int` values.

**Example:**
```AudioShader
5 & 2 // 0
```

### `|`
`|` is used to perform a bitwise OR operation on two `Int` values.

**Example:**
```AudioShader
5 | 2 // 7
```

### `^`
`^` is used to perform a bitwise XOR operation on two `Int` values.

**Example:**
```AudioShader
5 ^ 2 // 7
```

### `~`
`~` is used to perform a bitwise NOT operation on an `Int` value.

**Example:**
```AudioShader
~5 // -6
```

### `<<`
`<<` is used to perform a left shift operation on an `Int` value.

**Example:**
```AudioShader
5 << 2 // 20
```

### `>>`
`>>` is used to perform a right shift operation on an `Int` value.

**Example:**
```AudioShader
5 >> 2 // 1
```

## Unary Operators

### `+`
`+` is used to convert a value to its positive form. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
+5 // 5
```

### `-`
`-` is used to convert a value to its negative form. It can only be used with `Float` and `Int` values.

**Example:**
```AudioShader
-5 // -5
```

## Mathematics

### `pi()`
`pi()` is used to get the value of pi.

**Example:**
```AudioShader
pi() // 3.141592653589793
```

### `abs(value: Int | Float)`
`abs()` returns the absolute value of the given `Int` or `Float` value.

**Example:**
```AudioShader
abs(-5) // 5
```
