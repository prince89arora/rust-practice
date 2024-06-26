# Rust Documentation
Rust is a system programing language.


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