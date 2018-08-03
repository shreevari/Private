use std::collections::HashMap;
use std::io;
fn main() {
    println!("HashMap and vectors : Text interface to add employees to departments");
    let mut departments = HashMap::new();
    loop{
    	let mut instruction = String::new();
    	println!("$ ");
    	io::stdin().read_line(&mut instruction).expect("Reading failed");
	    instruction = instruction.trim().to_lowercase();
	    let instruction: Vec<&str> = instruction.split_whitespace().collect();
	    match instruction[0]{
	    	"add" => {
	    		let department = String::from(instruction[3]);
	    		let employees = departments.entry(department).or_insert(vec![]);
	    		employees.push(String::from(instruction[1]));
	    	},
	    	"retrieve"=>{
	    		let names = departments.get(instruction[2]).unwrap();
	    		println!("{:?}", names);
	    	}
	    	_ => break,
	    };
    }
}