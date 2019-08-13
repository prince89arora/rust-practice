mod print_helper;
mod types;
mod strings;
mod touples;
mod array;
mod vectors;

fn main() {
    println!("Hello, world!");
    print_helper::run();
    types::run();
    strings::run();

    touples::run();
    array::run();
    vectors::run();
}
