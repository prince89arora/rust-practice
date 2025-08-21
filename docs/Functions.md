# Functions

- `fn` to declare a function.
- Snake case recommended to use for function names.
- `->` is use to define return type.

```rust
fn do_something(var1: i32) -> i32 {
   return 10;
}
```
    
- If trying to return a value directly then return keyword is not needed. This is called `Tail Expression`.
```rust
fn do_something() -> i32 {
   10;
}
```

- Does not support different types for same argument. (Marcos supports this)
- Arguments should be in correct order.

## Closure

- Also called anonymous function. A function the does not have a name.
- Parameters can be passed and return type can be defined.
- Can use any variable that is in scope without explicitly passing as an argument.
 
