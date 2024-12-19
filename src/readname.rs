fn main() {
    let mut name: String = Default::default();
    println!("Please enter your name: ");

    std::io::stdin().read_line(&mut name).expect("Name not provided!!!");

    name = name.trim_end().parse().unwrap();
    println!("{} , is that provided as name.", name);
}