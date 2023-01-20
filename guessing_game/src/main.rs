use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    println!("Guess the number game! {}", secret_number);


    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad input, try again!");
                continue
            }
        };

        println!("Your guess is: {number}");

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

