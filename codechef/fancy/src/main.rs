use std::io;
use std::ascii::AsciiExt;
fn main() {
	let mut n = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut n).expect("Failed to read line");
	let n: u32 = n.trim().parse().expect("NaN");
	let mut lines = String::new();
	for _i in 0..n {
		stdin.read_line(&mut lines).expect("Failed to read input line");
	}
	for line in lines.lines() {
		let mut present = false;
		if let Some(_) = line.split_whitespace().filter(|str| str.eq_ignore_ascii_case("not")).next() {
			println!("Real Fancy");
		} else {
			println!("regularly fancy");
		}	
	}
	
}