use std::io;
use std::collections::HashMap;
fn main() {
	println!("How many numbers do you want to enter ?");
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Invalid input");
	let n: i32 = n.trim().parse().expect("Enter valid number");
	let mut values: Vec<i32> = Vec::new();
	for _i in 0..n {
		let mut num = String::new();
		io::stdin().read_line(&mut num).expect("Invalid input");
		let num: i32 = num.trim().parse().expect("Enter valid number");
		values.push(num);
	}
	let mean = mean(&values);
	println!("Mean : {}", mean);
	let median = median(&mut values);
	println!("Median : {}", median);
	let mode = mode(&values);
	println!("Mode : {}", mode);
}

fn mean(values: &Vec<i32>) -> f64 {
	let mut mean = 0.0;
	for value in values {
		mean += *value as f64;
	}
	mean /= values.len() as f64;
	mean
}

fn median(values: &mut Vec<i32>) -> i32 {
	values.sort();
	values[values.len()/2]
}

fn mode(values: &Vec<i32>) -> i32 {
	let mut frequency: HashMap<i32, i32> = HashMap::new();
	for value in values {
		let mut num = frequency.entry(*value).or_insert(0);
		*num += 1;
	}
	let mut max = 0;
	let mut mode = 0;
	for (key, value) in frequency.iter() {
		if *value > max {
			max = *value;
			mode = *key;
		}
	}
	mode
}