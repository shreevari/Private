use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_insensitive: bool,
}

impl Config {
	pub fn new (args : &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough parameters");
		}
		let query = args[1].clone();
		let filename = args[2].clone();
		let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config{ query, filename, case_insensitive })
	}
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	
	let results = if config.case_insensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
	for line in results {
		println!("{}", line);
	}

	Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut matches = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			matches.push(line);
		}
	}
	matches
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut matches = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			matches.push(line);
		}
	}
	matches
}

/*pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
	fs::read_to_string("")?;
	Ok(())
}
*/
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_config() {
		let query = String::from("test");
		let filename = String::from("poem.txt");
		let args = vec!["".to_string(), query.clone(), filename.clone()];
		assert_eq!(Config{ query, filename, case_insensitive: true }, Config::new(&args).unwrap());
	}

	#[test]
	fn runs() {
		let args = vec![String::new(), "test".to_string(), "poem.txt".to_string()];
		if let Err(e) = run(Config::new(&args).unwrap()) {
			panic!("Does not run : {}", e);
		}
	}

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(
			vec!["Rust: ", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}