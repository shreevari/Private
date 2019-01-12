use std::io;
//use std::ascii::AsciiExt;
fn main() {
	let stdin = io::stdin();
	let mut alphabet = String::new();
	stdin.read_line(&mut alphabet).unwrap();
	let mut input = String::new();
	stdin.read_line(&mut input).unwrap();

	for letter in alphabet.chars() {
		for c in input.chars().filter(|a| a.eq_ignore_ascii_case(&letter)) {
			print!("{}", c);
		} 
	}
}