use std::io;

fn main() {
    let mut degree = String::new();
    io::stdin().read_line(&mut degree)
    	.expect("Enter a valid input");
    let degree: f64 = match degree.trim().parse() {
    	Ok(num) => num,
    	Err(_) => 0.0
    };
    println!("Choose to convert to \n1 => Celsius \n2 => Fahrenheit");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
    	.expect("Enter valid input");
    let choice: i32 = choice.trim().parse()
    	.expect("Invalid input");
    if choice==1 {
    	println!("{} degree fahrenheit in celsius is {}", degree, fahrenheit_to_celsius(degree));
    } else if choice==2 {
    	println!("{} degree celsius in fahrenheit is {}", degree, celsius_to_fahrenheit(degree));
    } else {
    	println!("{} degree is {} degree", degree, degree);
    }
}

fn fahrenheit_to_celsius(mut degree: f64) -> f64 {
	degree -= 32.0;
	degree /= 1.8;
	degree
}

fn celsius_to_fahrenheit(mut degree: f64) -> f64 {
	degree *= 1.8;
	degree += 32.0;
	degree
}