use std::io;

fn main() {
    let mut unit = String::new();
    let mut temp = 0.0;

    println!("What is the desired temperature unit? Enter Celsius or Fahrenheit.");

    unit = read_unit();

    println!("What is the temperature? Please insert numbers only.");

    temp = read_temp();

    println!("You entered {temp} °{unit}.");
}

fn read_unit() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

fn read_temp() -> f64 {
    let mut attempts = 0;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if attempts < 3 {
                    println!("Please insert numbers only.");
                } else {
                    println!("Are you kidding me? Numbers only.");
                }
                attempts += 1;
                continue;
            }
        };
        return input;
    }
}
