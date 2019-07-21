//Print helper
pub fn run() {
    println!("My name is {} {}", "Prince", "Arora");

    let message = "Test message to display...";
    println!("{}", message);

    let mut message_mut = "This is changeable message...";
    println!("Before {}", message_mut);

    message_mut = "This is changed....";
    println!("After {}", message_mut);

    const CONST_VAL:i32 = 1;
    println!("This is integer const : {}", CONST_VAL);

    //group values
    let (val1, val2) = ("value for one", 2);
    println!("val1 {}, val2 {}", val1, val2);

}