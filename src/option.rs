fn main() {
    let name = String::from("John");
    let ch = name.chars().nth(9);

    let letter = match ch {
        Some(ch) => ch.to_string(),
        None => String::from("No character found")
    };

    println!("Letter is -- {}", letter);
}