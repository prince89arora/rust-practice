# Strings

Rust has total 6 types of strings. But we care about only 2 of them.

## String slice (str) 

- Literal string is always a borrowed string slice.
- Cannot be modified.


## String

- Data can be modified.

```rust
let msg = "abc".to_string();

// or 

let msg = String::from("abc");
```




