mod print_helper;
mod types;
mod strings;

fn main() {
    println!("Hello, world!");
    print_helper::run();
    types::run();
    strings::run();
}
