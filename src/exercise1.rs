

pub fn run() {
    let mut missiles = 8;
    let tofire = 2;
    println!("Starting to fire the missiles");
    println!("Firring {} out of {} missiles ", tofire, missiles);

    missiles = missiles - tofire;

    println!("Remianing Missiles {}", missiles);
}