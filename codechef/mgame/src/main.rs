use std::io;
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
		let mut inputs = line.split_whitespace();
		let n: u32 = inputs.next().unwrap().parse().unwrap();
		let p: u32 = inputs.next().unwrap().parse().unwrap();
		let count: usize = (0..n)
			.map(|i| (0..i)
				.map(|j| (0..j)
					.map(|k| 
						println!("{}", k)
						)
					)
				).count();
		println!("{}", count);	
	}
	
}