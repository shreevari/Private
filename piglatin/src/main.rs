use std::io;
fn main() {
    println!("PIGLATIN");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    	.expect("Input could not be read");
   	input = input.to_lowercase();
    input = String::from(input.trim());
    let mut character = input.chars();
    let character = character.next().unwrap();
    if character.is_alphabetic() {
    		match character {
	    		'a' | 'e' | 'i' | 'o' | 'u' => {
	    			println!("Piglatin : {}-hay", input);
	    		},
    			_ => {
    				println!("Piglatin : {}-{}ay", &input[1..], &input[0..1]);
    			},
    		}
    	}
}
