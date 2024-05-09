# Functions

- `fn` to declare a function.
- Snake case recommended to use for function names.
- `->` is use to define return type.

```rust
fn do_something(var1: i32) -> i32 {
   return 10;
}
```
    
- If trying to return a value directly then return key work is not needed. This is called `Tail Expression`.
```rust
fn do_something() -> i32 {
   10;
}
```

- Does not support different types for same argument. (Marcos supports this)
- Arguments should be in correct order.