use std::io;
fn main() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed to read line");
	let n: u32 = n.trim().parse().unwrap();
	for i in 1..n + 1 {
		println!("{} Abracadabra", i);
	}
}