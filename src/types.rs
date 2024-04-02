// /*
//   Integers
//   Float
//   Boolean
//   Characters
//   Tuples
//   Arrays
// */
// //Rust is statically types language.
// //But its not mandatory to assign type for every single var.
// pub fn run() {
//
//     // default is i32
//     let default_int = 1;
//     println!(" {} ", default_int);
//
//     // default is f64
//     let default_float = 1.2;
//     println!(" {} ", default_float);
//
//     // explicitly given type
//     let explicit_type_integer: i64 = 12312312;
//     println!(" {} ", explicit_type_integer);
//
//     //find max for any numeric data type.
//     println!("Max i16: {}", std::i16::MAX);
//     println!("Max i32: {}", std::i32::MAX);
//     println!("Max f32: {}", std::f32::MAX);
//
//     //boolean vars
//     let bool_default = true;
//
//     // explicitly defined
//     let bool_defined: bool = true;
//
//     // boolean from expression
//     let bool_expression = 10 > 0;
//     println!("{:?}", (bool_default, bool_defined, bool_expression));
//
//     //char
//     //characters in rust are unicode.
//
//     //character definition
//     let a_char = 'a';
//
//     //unicode char
//     let unicode_char = '\u{1F600}';
//     println!("{:?}", ( a_char, unicode_char ));
//
//
// }