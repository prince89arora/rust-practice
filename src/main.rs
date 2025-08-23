// use std::fs::File;
// use std::io::{Error, Read};
// use std::io::ErrorKind;

#[warn(unused_variables)]
fn main() {
    // let number_val: u8 = 50;
    // let char = number_val as char;
    //
    // println!("Character is: {}", char);

    //const BASE_VAL: f32 = 12.05;

    // ================= Operations =====================//
    // let mut val1: f32 = 41.40156;
    // let val2: f32 = 30.042399128193219381212831836128736;
    //
    // println!("Val1 to degree {:.2}", val1.to_radians());
    // println!("Val1 to degree {:.2}", val2.to_degrees());
    //
    // val1 = 30.0;
    // println!("POWI {}", f32::powi(val1, 2));
    // ================= Operations =====================//

    // ================= Lifetime =====================//
    // let a: &str = "test";
    // let b: &str = "test1";
    // let _outer_val = bigger_string(a, b);
    // // In this bigger_string can be used anywhere in code and arguments can have very
    // // different scope. To solve that with use of lifetime. Return variable will have minimum
    // // scope from arguments.
    // print!("{}", _outer_val);
    // ================= Lifetime =====================//


    // ================= Closure =====================//
    // let prefix = "Hi, ";
    // let with_name = |name: &str| -> String {
    //     format!("{} {}", prefix, name)
    // };
    //
    // println!("{}", with_name("Prince"));
    // ================= Closure =====================//


    // ================= Error Handling =====================//
    // let list = vec![1,2,3,4,5,6];
    // println!("{}", list[8]);

    // // Handle error without propagation
    // let file_name = "./test.json";
    // match File::open(file_name) {
    //     Ok(file) => {
    //         println!("Reading file {:#?}", file);
    //     }
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => {
    //                 match File::create("./test.json") {
    //                     Ok(file) => { println!("Create new file {:#?}", file); }
    //                     Err(error) => {println!("Error: {:#?}", error); }
    //                 }
    //             }
    //             _ => {
    //                 println!("Error: {:#?}", error);
    //             }
    //
    //         }
    //     }
    // }


    // // Error propagation
    // let file_name = "./test.json";
    // match read_file(file_name) {
    //     Ok(contents) => {
    //         println!("{}", contents);
    //     }
    //     Err(error) => {
    //         println!("{:#?}", error);
    //     }
    // }
    // ================= Error Handling =====================//


}

// /// Function to find gigger string from 2 strings and to show how lifetime works
// /// * a
// /// * b
// fn bigger_string<'a>(a: &'a str, b: &'a str) -> &'a str {
//
//     //! &'a str     = to define lifetime for a reference type.
//     //! &'a mut str = to define lifetime for a mutable reference type.
//
//     if a.len() > b.len() { a } else { b }
// }

// /// Function to read a file content as string. Propagate if there is any error
// /// * file_name = file path as string slice.
// fn read_file(file_name: &str) -> Result<String, Error> {
//     let mut file = File::open(file_name)?;
//     let mut file_data_string = String::new();
//     file.read_to_string(&mut file_data_string)?;
//     Ok(file_data_string)
// }