use std::io;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	let mut count = 0;

	stdin.read_line(&mut input).expect("Failed to read line");
    
    let mut input = input.split_whitespace();
    let n: u32 = input.next().unwrap().parse().expect("Not a number");
    let k: u32 = input.next().unwrap().parse().expect("Not a number");
    
    let mut inputs = String::new();
    for _i in 0..n {
    	stdin.read_line(&mut inputs).expect("Failed to read line");
    }
    for is_divisible in inputs.split_whitespace().map(|m| m.parse::<u32>().unwrap() % k == 0) {
    	if is_divisible { count += 1 };
    } 

    println!("{}", count);
}
