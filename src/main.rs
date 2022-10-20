use std::io;

fn main() {
    let mut unit = String::new();

    println!("What is the desired temperature unit? Enter Celsius or Fahrenheit.");

    unit = read_unit();

    println!("You entered {unit}.");
}

fn read_unit() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}
