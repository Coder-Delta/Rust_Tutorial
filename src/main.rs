// I will learn about let, match, methods, associated functions, external crates, and more! In the following chapters,
// I will build a guessing game that prompts the user to guess a number between 1 and 100.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess: String = String::new();
        //mutable variable to store user input by default variables are immutable but to mutate we use mut keyword

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
