use std::io;
fn main() {
    println!("Temperature Scale Converter");
    println!("Enter the Temperature : ");
    let mut temperature = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f64 = temperature.trim().parse().expect("invalid");
    println!("Choose the scale you want to convert to :\n 1 => Celsius\n2 => Fahrenheit\n");
    let mut choice = String::new();
    stdin.read_line(&mut choice).expect("Failed to real line");
    let choice: i32 = choice.trim().parse().expect("invalid");
    println!(
        "{}",
        match choice {
            1 => format!("{}", to_celsius(temperature)),
            2 => format!("{}", to_fahrenheit(temperature)),
            _ => "You have entered an invalid choice".to_string(),
        }
    );
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}
fn to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}
