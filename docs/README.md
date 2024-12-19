# Rust Documentation
Rust is a system programing language.

## Development Environment

### Rustup
It is a development toolchain that helps to install rust and other rust related development tools. 

## Rust Program Anatomy

- `main` function is the entry point for a rust application.
```rust
#![allow(unused_variables)]

fn main() {
    print!("This is the entry point");
}
```
- A rust application can only have single `main` function.
- ! sign after print in above example indicates that print is a `macro`.
- `#![allow(unused_variables)]` is attribute declared to tell the compile not to show any warning when we have unused variables in our code.

## Module System

- `use`: This can be used to import something from another file from same project or from any other project/library.
```rust
use std::collections::HashMap;
```

- An external package can be added in cargo yaml file under dependency section with the required version.


## Data Types

## Scalar Data Types

Holds single value

``Note: `Primitive data types` are data types that are built into the language (are base for other custom data types) and are stored on stack.``

### Numeric

#### Int

- Whole number without any fractional part. (1 is int but not 1.2)
- Can be negative, and if it can be negative than it should be defined as signed.
  - Deciding if int should be signed or unsigned depends on how big the number can be. Max size of an unsigned int is way bigger then max size of signed int.
- We can also decide how many bits we want to allocate to the int.
  - [8,16,32,64,128] are the sizes.

### Characters and Booleans

## Compound Data Types

Holds multiple values

### Arrays

- Multiple values of same type

### Tuples


## Strings



## Variables

### Declare Variables

### Casting Data Types

### Variable Immutability

### Variable Scope and Shadowing



## Important Benefits
- Provides type and memory safety, but with the size and performance characteristics of natively compiled languages like C and C++. 


## Struts

- Like classes in other programing languages.
- Can have data fields, methods and associated functions.
- Capital Camel Case.

```rust
struct Animal {
    age: i32,
    name: String,
}
```

- Struct can also be initialized directly

```rust
let dog = Animal {
   name: String::from("dog name"),
    age: 10,
}
```

- Associated functions can be used as a constructor to initiate struct with default values.
- Methods and associated functions are defined in implementation.
- `impl` keyword is used to write an implementation.

```rust
impl Animal {
    fn new() -> Self {
        Self {
            name: String::from(""),
            age: 0
        }
    }
}
```

- In above example, new is an associated function, in other languages its called as class level or constructor. (function name `new` is conventional).

_Note: `::` is called scope operator in Rust._

- Rust does not support Struts inheritance. It solves the issue using `Traits`.

## Traits

- Similar to interfaces in other languages.
- Rust takes composition over inheritance approach.

Example:
Traits can be used to define a required functionality or method for structs.

```rust
struct Fox {
    life: i32
}

trait Noisy {
    fn get_noisy(&self) -> &str;
}

impl Noisy for Fox {
    fn get_noisy(&self) -> &str {
        "This is noisy!!"
    }
}
```

- We can also define this function directly in implementation of struct, but defining that in trait will enable us to use generics. This means, a generic method can be created to accept anything that inherits a specific trait. 
- As long as either trait or the struct is defined in project, it can be implemented. Meaning the default datatypes can be used to implement a new trait defined in your project.
- Special trait called `Copy` if this is used, then it will be copied instead of move in move situation. Simple primitive types inherit this trait.
- Default behaviour of a function can be provided in a trait. 
Example:
```rust
trait Runnable {
    fn run(&self) {
        println!("Default running !!!!");
    }
}
struct Animal {}
impl Runnable for Animal {}
```

By not providing an implementation for Animal, will force to use the default behaviour for run. 

- Fields cannot be defined in a trait. 1 workaround is to define getter and setter methods.


## Collections

