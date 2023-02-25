use rand::Rng;
use std::cmp::Ordering; // this is an enum
use std::io::stdin; // io library

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();                        // mut makes variable mutable
    println!("Guess the number game! {}", secret_number); // println! is a macro

    loop {
        guess.clear(); // clear the string
        stdin()        // stdin() gives you a handle to the STDIN of this process
            .read_line(&mut guess) // returns a result Result<T,E> where T - is ok, E is - error
            .expect("Failed to read line!");   // expect

        let number: u32 = match guess.trim().parse() { // I am working here with a Result type
            Ok(num) => num,
            Err(_) => {
                println!("Bad input, try again!");
                continue;
            }
        };

        println!("Your guess is: {number}");

        // match expression -- put it in front of the statement
        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
