/*
    Enums in rust have default value to int (index)
    Each enum value can have different types.
*/


use std::any::Any;

enum Enum1 {
    ONE = 0,
    TWO,
    THREE {name: String, age: i32}
}

fn test () {
    // println!("Enum1::ONE:\t {}", Enum1::ONE as i32);

    let test = Enum1::THREE {name: String::from("John"), age: 30};

    println!("Enum1::THREE:\t {}", Enum1::THREE as {name: String, age: i32});

    println!("Enum1::THREE:\t {}", (test as Enum1::THREE).name);


}