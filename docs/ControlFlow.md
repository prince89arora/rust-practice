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

