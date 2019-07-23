//primitive strings are fixed length strings in memory.
//String = growable, Head allocated data structure, used when need to modify string data.

pub fn run() {

    let hello = String::from("Hello!!");
    println!("{}", hello);
    println!("Length: {}", hello.len());
}