use std::io;
fn main() {
    let mut input: String = String::new();
    println!("Temperature Conversion");
    println!("1 - Convert Fahrenheit to Celsius");
    println!("2 - Convert Celsius to Fahrenheit");
    println!("Enter your choice (1 or 2): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a number.");
            return;
        }
    };
    match choice {
        1 => convert_fahrenheit_to_celsius(input_value("Enter temperature in Fahrenheit: ")),
        2 => convert_celsius_to_fahrenheit(input_value("Enter temperature in Celsius: ")),
        _ => println!("Invalid choice! Please enter 1 or 2."),
    }
}

fn input_value(prompt: &str) -> f64 {
    let mut input: String = String::new();
    println!("{prompt}");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse input")
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) {
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{} is equal to {}", format_temperature(fahrenheit, "°F"), format_temperature(celsius, "°C"));
}

fn convert_celsius_to_fahrenheit(celsius: f64) {
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{} is equal to {}", format_temperature(celsius, "°C"), format_temperature(fahrenheit, "°F"));
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn format_temperature(temp: f64, unit: &str) -> String {
    format!("{:.2} {}", temp, unit)
}
