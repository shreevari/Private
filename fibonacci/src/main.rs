use std::io;

fn main() {
    println!("Enter a number");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Enter valid input");
    let num: i32 = num.trim().parse().expect("Not a valid number");
    let mut fib_prev = 0;
    let mut fib_curr = 1;
    for _x in 0..num {
        let temp = fib_curr;
        fib_curr += fib_prev;
        fib_prev = temp;
    }
    println!("Fibonacci number is {}", fib_curr);
}
