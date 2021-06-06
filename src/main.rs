mod control_flow;
mod print_helper;
mod types;
mod strings;
mod touples;
mod array;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_type = &args[1];
    if run_type == "print" {
        print_helper::run();
    } else if run_type == "control" {
        control_flow::run();
    } else if run_type == "types" {
        types::run();
    } else if run_type == "string" {
        strings::run();
    } else if run_type == "touple" {
        touples::run();
    } else if run_type == "array" {
        array::run();
    }


}
