# Ownership

- There are 3 rules for ownership.

1. Each value has an owner. 
   - There is no value in system that does nto have an owner.
   - Means every data is owned by a variable.
2. Data can have only single owner. 
   - Ownership cannot be shared.
   - Other variables can borrow a value.
3. When the owner gets out of scope, the value is dropped immediately.


Example:
```rust
let s1 = String::from("str1");
let s2 = s1;

println!("{}", s1); // Compiler error
```

```rust
let s1 = String::from("str1");
```
A pointer, length and capacity is pushed into stack at this point.
Pointer points to a newly allocated bytes in heap which contains the actual value.


```rust
let s2 = s1;
```
With above line, we are assigning s2 to s1. 
This will create a new everything from s1 in stack is copied over as a new value as s2 in stack.

At this point s1 is invalidated and considered as un initialized.

This is not a shallow copy but a move.

Clone method can be used to create a copy of data without changing the ownership.

```rust
let s2 = s1.clone();
```

This will crate a new item in stack as well as copy the data in heap as well. The new item in stack will point to the newly created data in heap.