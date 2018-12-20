extern crate csv;
use std::env;
use std::process;
use std::error::Error;

fn main() {
	if let Err(err) = run() {
		println!("{}", err);
		process::exit(1);
	}
}

fn run() -> Result<(), Box<Error>> {
	let file_path = get_first_arg()?;
//	let file = File::open(file_path)?;
	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_path(file_path)?;
	for result in rdr.records() {
		println!("{:?}", result?);
	}
	Ok(())
}
/*
fn get_first_arg() -> Result<OsString, Box<Error>> {
	match env::args_os().nth(1) {
		None => Err (From::from("Expected 1 argument but got none")),
		Some(file_path) => Ok(file_path),
	}
}
*/
fn get_first_arg() -> Result<String, Box<Error>> {
	match env::args().nth(1) {
		None => Err (From::from("Expected 1 argument but got none")),
		Some(file_path) => Ok(file_path),
	}
}