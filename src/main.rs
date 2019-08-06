mod print_helper;
mod types;
mod strings;
mod touples;

fn main() {
    println!("Hello, world!");
    print_helper::run();
    types::run();
    strings::run();

    touples::run();
}
