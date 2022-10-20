use std::io;

fn main() {
    loop {
        let unit = read_unit();

        if unit.starts_with("f") {
            let temp = read_temp();
            celsius_to_fahrenheit(temp);
            break;
        } else if unit.starts_with("c") {
            let temp = read_temp();
            fahrenheit_to_celsius(temp);
            break;
        } else {
            println!("Please enter a valid unit.");
            continue;
        }
    }
}

fn read_unit() -> String {
    println!("What is the desired temperature unit? Enter Celsius or Fahrenheit.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_lowercase();
}

fn read_temp() -> f64 {
    println!("What is the temperature? Please insert numbers only.");

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

fn celsius_to_fahrenheit(temp: f64) {
    let temp = temp * 1.8 + 32.0;
    print_results(temp, "Fahrenheit");
}

fn fahrenheit_to_celsius(temp: f64) {
    let temp = (temp - 32.0) / 1.8;
    print_results(temp, "Celsius");
}

fn print_results(temp: f64, unit: &str) {
    println!("Result: {temp} °{unit}")
}
