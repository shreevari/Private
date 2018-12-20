fn main() {
    println!("The twelve days of Christmas.\nBy Bing Crosby");

    let verses = [
        ("first", "a partridge"),
        ("second", "two turtle doves"),
        ("third", "three French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five golden rings"),
        ("sixth", "six geese a-layin"),
        ("seventh", "seven swans a-swimmin"),
        ("eigth", "eight maids a-milkin"),
        ("ninth", "nine lords a-leapin"),
        ("tenth", "ten ladies dancin"),
        ("eleven", "eleven pipers pipin"),
        ("twelfth", "twelve drummers drummin"),
    ];
    let length = 12;
    for i in 0..length {
        println!(
            "\n\nOn the {} day of Christmas, My true love gave to me ",
            verses[i].0
        );
        let mut count = 1;
        for j in (0..=i).rev() {
            print!("{}, ", verses[j].1);
            if count % 3 == 0 {
                println!();
            }
            count += 1;
        }
        print!("in a pear tree");
    }
}
