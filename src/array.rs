pub fn run() {
    let arr: [i8; 4] = [1,2,3,4];

    // all element print.
    println!("{:?}", arr);

    //get memory size for an array...
    println!("Consumes {} bytes", std::mem::align_of_val(&arr));
}