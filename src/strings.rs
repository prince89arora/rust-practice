//primitive strings are fixed length strings in memory.
//String = growable, Head allocated data structure, used when need to modify string data.

pub fn run() {
    // Growable String..
    let mut hello = String::from("Hello!!");
    println!("{}", hello);
    println!("Length: {}", hello.len());

    //push char
    hello.push(' ');
    //push string
    hello.push_str("There...");
    println!("{}", hello);
}