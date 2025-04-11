fn main() {

    // arm with range check
    let number = 5;
    let result = match number {
        1..=3 => String::from("Between 1 to 3"),
        4..=5 => String::from("Between 4 to 5"),
        _ => String::from("Greater than 5")
    };
    println!("Result is {}", result);

    // arm with if check
    let result = match number {
        number if number <= 2 || number >= 5 => String::from("Number is unique"),
        _ => String::from("Number is not unique")
    };
    println!("Result is {}", result);

    // match with enum
    enum Color {
        Red,
        Green,
        Blue
    };
    let testcolor = Color::Red;
    match testcolor {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue")
    }
}