mod enums;
mod option;
mod r#match;
mod strings;

#[warn(unused_variables)]
fn main() {
    // let number_val: u8 = 50;
    // let char = number_val as char;
    //
    // println!("Character is: {}", char);

    //const BASE_VAL: f32 = 12.05;

    let mut val1: f32 = 41.40156;
    let val2: f32 = 30.042399128193219381212831836128736;

    println!("Val1 to degree {:.2}", val1.to_radians());
    println!("Val1 to degree {:.2}", val2.to_degrees());

    val1 = 30.0;
    println!("POWI {}", f32::powi(val1, 2));


}