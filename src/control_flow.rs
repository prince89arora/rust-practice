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


    //loops
    //1. Simple loop
    let mut counter: i32 = 0;
    loop {
        counter+=1;
        println!("Counter is: {} ", counter);
        if counter == 5 {
            break;
        }
    }

    //2. return value from loop
    counter = 0;
    let result_loop_return = loop {
        counter+=1;
        if counter == 4 {
            break counter + 1;
        }
    };
    println!("Counter from assignment {}", result_loop_return);

    //3. while loop.
    let while_list_ex = [1,2,3,4,5,6];
    let mut index_while = 0;
    while index_while < 6 {
        println!("While loop item: {}", while_list_ex[index_while]);
        index_while += 1;
    }

}