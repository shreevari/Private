use std::io;
fn main() {
    println!("1");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    let input: u8 = input.trim().parse().expect("Not a number");
    if input == 2 {
        println!("3");
    } else {
        println!("2");
    }
}
