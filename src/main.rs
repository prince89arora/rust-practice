use log::log;

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