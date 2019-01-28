use std::io;
fn main() {
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse::<u32>().unwrap();
    if n % 2 == 0 {
    	println!("BOB");
    } else {
    	println!("ALICE");
    }
}
