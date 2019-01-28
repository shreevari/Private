use std::io;

fn main() {
	let stdin = io::stdin();
	let mut x = String::new();
	stdin.read_line(&mut x).expect("Failed to read line");
	let x: u32 = x.trim().parse().expect("Failed to read MAX");
	let mut n = String::new();
	stdin.read_line(&mut n).expect("Failed to read line");
	let n: u32 = n.trim().parse().expect("Failed to read number");
	let mut inputs = String::new();
	for _i in 0..n {
		stdin.read_line(&mut inputs).expect("Failed to read monthly usage");
	}
	let sum: u32 = inputs.split_whitespace().map(|num| num.parse::<u32>().unwrap()).sum();
	println!("{}", x * (n + 1) - sum);
}