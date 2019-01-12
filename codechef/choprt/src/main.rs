use std::io;
use std::cmp::Ordering;

fn main() {
    let mut n = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut n).expect("Could not read argument");
    let n: u32 = n.trim().parse().expect("Not an unsigned integer");
    for _i in 0..n {
    	let mut input = String::new();
    	stdin.read_line(&mut input).expect("Could not read first number");
    	let mut input = input.trim().split_whitespace();
    	let first_number: i32 = input.next().unwrap().parse().expect("Not an integer");
    	let second_number: i32 = input.next().unwrap().parse().expect("Not an integer");
    	
    	match first_number.cmp(&second_number) {
    		Ordering::Greater => println!(">"),
    		Ordering::Less => println!("<"),
    		Ordering::Equal => println!("="),
    	}
    }
}
