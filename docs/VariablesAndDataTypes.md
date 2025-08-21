# Variables

`let` can be used to declare variables.

- Rust does not have null data type.

```
let bunnies = 2
```
- Variables can be declared without explicit types.
- Rust is strongly types language, but it allows us to leave type for a variable when rust can identify the type.

```
let binnues: i32 = 2
```
- To declare type `:` is used.

**To declare multiple variables**

```
let (foo, bar) = (8,10);
```



- Data on right hand side can be de-structured to initialize variables on left hand side.
- Also called tuple pattern.

## Immutable

By default all vars are immutable. But can be declared as mutable.

Reasons - Safety (Less bugs), Concurrency (Easier to share data for multiple threads), Speed

### Mutable var

```rust
let mut mutableVar = 2;    
```

### More Immutability

```rust
const CONST_VAR: i32 = 30;    
```

- Case should be screaming snake case.
- var type is required.
- Value cannot set to a function result.
- These are inline when used. (When used, they are replaced with their value at compile time.)
- They can be declared anywhere but valid for entire program.


## Scope

- Variables are only valid for their scope.
- Variables are removed after a block or their scope.
- Simple {} can also be used to execute some code and create a new/inner scope.

```rust

let x = 1;
{
    let x = 2;
    println!("{}", x); // Prints 2
}
println!("{}", x); // Prints 1
```
- Declaring a variable with same name as outer scope is called `shadowing`.
- Variable shadow can also be done with in same scope.
- Datatype, mutability or other aspects of the variable can change when shadowing.

```rust
let mut v = 2;
let v= v;
```

```rust
let v = "2";
let v = 2;
```

## Lifetime

- **Generic Lifetime Annotations** are used to define relationship for scope of multiple references.
- It does not change actual scope of a variable.
- If same lifetime annotation is use for multiple arguments and the return type. In this case the smallest lifetime from arguments will be assigned to the return type. 


## Memory Safety

- Rust provides memory safety at compile time.
- Variables must be initialized before use.

## Data Types

### Data type casting
Rust does not perform any type of implicit type casting.

To explicitly type cast data `as` can be used.
```rust
let num1: f32 = 1.0;
let num2: u8 = 1;
let casted_num = num2 as f32;
let result = num1/num2;

print!("{}", result);
```

Explicit casting is supported until the is a clear way ot handle it. Example: u8 can casted to a char since rust
can use a character representation of that number.

### Scalar Types

#### Integer
- Unsigned and signed.
- Unsigned starts with u and signed starts with i.
- from 8 bit to size (u8,u16,u32,u64,u128,usize).
- Same for signed (i8,i16,i32,i64,i128,isize).

#### Float

- Numeric value with fractions.
- Sizes are `f32` and `f64`.

#### Characters and Booleans

- Characters in rust are 4 byte in size. It uses unicode-32. Can contain alot of characters.
- Booleans are true/false.

### Compound Types

#### Arrays

- Limited to size 32, above that it loses some functionality.
- Live on stack by default and are fixed size.
- Most of the time vectors will be useful.
- to declare with size and type

```rust
let myarray: [u8; 4];
```
this defined the type of item and size as 4.


#### Tuples

- Can contain different data types.
- Data can be accessed using index.

```rust
let location: (&str, f64, f64) = ("Delhi", 12.23456, -23.345678);
print!("location: {}, Lat: {}, Long: {}", location.0, location.1, location.2);
```

- Other way to read values from tuple is to deconstruct.

```rust
let (name, lat, long) = location;
```

#### Strings

- There are 2 types of string in rust. `String` and `&str`.
- `String`
    - Mutable
    - Stored on heap
- `&str`
    - Immutable
    - Stored on heap, stack or embedded in compiled code.

- Concatenating string slices `&str` will result in `String`.







