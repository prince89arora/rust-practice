pub fn run() {
    let mut vec_example: Vec<i32> = vec![1,2,3,4,5,5];

    vec_example.insert(0, 2);
    println!("{:?}", vec_example);
    println!("size {}", vec_example.len());
    vec_example.insert( vec_example.len(), 5 );

    println!("After Insertion....");
    println!("{:?}", vec_example);
    println!("size {}", vec_example.len());

    //push element at the end
    vec_example.push(10);
    let option = vec_example.pop().unwrap();
    println!("Get last element {}", option);

}