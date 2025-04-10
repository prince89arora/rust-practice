# Control Flow

## If condition

- If condition is an expression in rust, that means it can return a value and we can assign it to a variable.

```rust
result = if num == 5 {
    "five"
}  else {
    "not a number"
}
```
Another example: 

```rust
num = if a { b } else { c };
```

- In case of multi condition if statement, and is evaluated first and then or.

## Option

- Rust does not have null type.
- Used to see if a value is present or not.
- Returns an enum of Some and None depending on if the value is present or not.

## Match

- Similar to switch statement in other languages.
- `match` in rust is like a pattern matching statement.
- Single condition with arrow `=>` is called arm in rust.
- Can also use range for an arm in match like `200..=300`.

## Loops

loops are unconditional in rust

Example:
```rust
loop {
    break;
}
```

Loop annotation

```rust
'loop1: loop {
    loop {
        break 'loop1;
    }    
}
```
Continue keywords is same:

```rust
'loop1: loop {
    loop {
        continue 'loop1;
    }    
}
```


## For loop

For loop can be used to iterate over any type of iterator.

```rust
for item in [1,2,3,].iter() {

}
```

range example:

```rust
for number in 0..20 {
 // this will count from 0 to 19.
}
```

```rust
for num in 0..=20 {
    // this will count from 0 to 20 
}
```

