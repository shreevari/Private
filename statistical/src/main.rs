use std::collections::HashMap;
use std::io;
fn main() {
    println!("Hello, world!");
    let mut values: Vec<i32> = Vec::new();
    let mut n = String::new();
    let mut num: i32;
    println!("Enter the number of values you wish to input : ");
    io::stdin().read_line(&mut n)
    	.expect("Enter a valid number");
    let n: i32 = n.trim().parse().unwrap();
    for _i in 0..n {	
    	let mut input_number = String::new();
    	io::stdin().read_line(&mut input_number)
    		.expect("Enter a valid input");
    	num = input_number.trim().parse().unwrap();
    	values.push(num);
    }
    println!("Mean : {}", mean(&values));
    println!("Median : {}", median(&mut values));
    println!("Mode : {}", mode(&values));
}

fn mean(list: &Vec<i32>)-> i32 {
	let mut sum = 0;
	for num in list.iter(){
		sum += num;
	}
	sum = sum /  list.len() as i32;
	sum
}

fn median(list: &mut Vec<i32>)-> i32 {
	list.sort();
	list[list.len()/2]
}

fn mode(list: &Vec<i32>)-> i32 {
	let mut frequency = HashMap::new();
	for item in list.iter(){
		let count = frequency.entry(item).or_insert(0);
		*count += 1;
	}
	let mut max: i32 = 0;
	let mut max_key = 0; 
	for (key, value) in frequency{
		if value>=max {
			max = value;
			max_key = *key;
		}
	}
	max_key
}