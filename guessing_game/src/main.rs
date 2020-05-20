use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number from 1-100!");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too small..".bold().yellow()),
            Ordering::Greater => println!("{}", "Too large..".bold().yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".bold().green());
                break;
            }
        }
    }
}
