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

    //fixed length string
    let _fixed_string = "Hello"; //primitive string

    //misc methods

    //contains
    println!("Contains '!' {}", hello.contains("!"));

    //replace
    println!("Replace '!' {} ", hello.replace("!", "@"));

    //loop by white space
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

//    assertion testing
    assert_eq!(16, hello.len());

}