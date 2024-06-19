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


## Reference & Borrowing

Instead of moving a variable, we can use reference.

```rust
let s1 = String::from("str1");

fn do_something(s: &String) {
   
}
```

do_something function is accepting a string reference.
When passing s1 to do_something.

```rust
let s1 = String::from("str1");
do_something(&s1); // passing reference of s1 to do_something
println!("{}", s1); // s1 can be used as normal here

fn do_something(s: &String) {
   
}
```

Here so_something is borrowing the reference of s1, not the value. So, only the reference is moved in do_something function.

After the function is called, only the reference goes out of scope and dropped. Anf the borrowing ends at that point.
After that s1 can be used as normal.

- _Under the hood, Rust created a new pointer to s1 when we create a new reference._
- _Pointers are managed by Rust out of the box._
- _Rust always makes sure that the pointer are valid, using concept called **Lifetimes**, This makes sure that the references are always valid_
- _A reference can never point to null. Means, a reference can never outlive the main data._

- By default, references are immutable. Even if the values that is being referenced is mutable.
```rust
let mut mysring = String::from("abcd");
do_something(&mystring);
```

But if we make mutable reference to mutable value, then we can use the reference to change the values as well.

```rust
let mut mysring = String::from("abcd");
do_something(&mut mystring);
```

_(*mystring) astrick is used to reference to the value_

```rust
&x // this is a immutable reference to variable x
&mut x // this is a mutable reference to variable x

// Similarly with types

&i32 // this the type for immutable reference
&mut i32 // this is the type for mutable reference
```

- At a given time, we can only have 1 mutable reference for a variable. And any number of immutable references.


### Dereferencing

Dereferencing gives back access to the value. Aestrick is used for dereferencing in rust.

Example:
If x is a `&mut i32`, a mutable reference to an i32 variable. Then `*x` will give mutable access to the value. Same goes to immutable reference.


