use std::io;

fn main() {
	let stdin = io::stdin();
	let mut t = String::new();
	stdin.read_line(&mut t).expect("1");
	let mut t: u8 = t.trim().parse().expect("2");
	
	while t > 0 {

		let mut details = String::new();
		stdin.read_line(&mut details).expect("3");
		let mut details = details.split_whitespace().skip(1).map(|num| num.parse().expect("4"));
		let a: u8 = details.next().expect("5");
		let b: u8 = details.next().expect("6");
		
		let mut bob = true;

		let mut inputs = String::new();
		stdin.read_line(&mut inputs).expect("7");
		let mut inputs: Vec<u64> = inputs.split_whitespace().filter_map(
			|num| {
				let m = num.parse().expect("8");
				if m % a as u64 == 0 || m % b as u64 == 0 {
					Some(m)
				} else { None }
			}).collect();
		let mut n = inputs.len();
		while n > 0 {
			bob = !bob;
			let mut index = 0;
			let mut lost = true;

			let mut element = match bob {
				false => a as u64,
				true => b as u64,
			};
			for i in 0..n {
				if inputs[i] == 0 {
					continue;
				}
				if inputs[i] % element == 0 {
					index = i;
					element = inputs[i];
					lost = false;
				}
			}
			if lost {
				break;	
			} else {
				inputs[index] = 0;	
			}
			n = n - 1;
		}
		if !bob {
			println!("ALICE");
		} else {
			println!("BOB");
		}
		//End stuff
		t = t - 1;
	}
}