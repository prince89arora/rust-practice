pub fn run() {
    //1. Simple if condition
    let is_true: bool = true;
    if is_true {
        println!("If case");
    } else {
        println!("Else case");
    }

    //2. condition as expression assignment.
    let result: i32 = if is_true {
        1
    } else {
        0
    };
    println!("Result from assignment {}", result);

}