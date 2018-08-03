use std::io;
fn main() {
    println!("Enter a word : ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Enter valid value");
    word = word.trim().to_lowercase();
    let mut char = word.chars();
    let char = char.next().unwrap();
    if char.is_alphabetic() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("Piglatin : {}-hay", word);
            },
            _ => {
                println!("Piglatin : {}-{}ay", &word[1..], &word[0..1]);
            }
        }
    }
}