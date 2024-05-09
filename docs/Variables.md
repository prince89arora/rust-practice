# Variables

`let` can be used to declare variables.

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


## Scope

- Variables are only valid for their scope.
- Variables are removed after a block or their scope.
- Simple {} can also be use to execute some code and create a new/inner scope.

```rust

let x = 1;
{
    let x = 2;
    println!("{}", x); // Prints 2
}
println!("{}", x); // Prints 1
```

- Variable shadow can also be done with in same scope.

```rust
let mut v = 2;
let v= v;
```

```rust
let v = "2";
let v = 2;
```


## Memory Safety

- Rust provides memory safety at compile time.
- Variables must be initialized before use.




