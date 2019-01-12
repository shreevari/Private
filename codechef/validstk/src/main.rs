use std::io;
fn main() {
	let stdin = io::stdin();
	let mut n = String::new();
	stdin.read_line(&mut n).expect("Could not read number of test cases");
	let n: u16 = n.trim().parse().expect("Not a number");
	for _i in 0..n {
		let mut number_of_inputs = String::new();
		stdin.read_line(&mut number_of_inputs).expect("Failed to read number of inputs");
		validate();
	}

}

fn validate() {
	
	let mut stack_count = 0;

	let mut inputs = String::new();
	io::stdin().read_line(&mut inputs).expect("Failed to read line");
	for input in inputs.split_whitespace().map(|num| num.parse::<u16>().unwrap()) {
		match input {
			0 => if stack_count > 0 {
				stack_count -= 1;
			} else {
				println!("Invalid");
				return;
			},
			1 => stack_count += 1,
			_ => println!("Wrong input"),
		}
	} 
	println!("Valid");
}