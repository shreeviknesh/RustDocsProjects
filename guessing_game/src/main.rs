use colored::*;
use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number from 1-100!");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret {
            println!("{}", "Too small..".yellow());
        } else if guess > secret {
            println!("{}", "Too large..".red());
        } else {
            println!("{}", "You win!".green());
            break;
        }
    }
}
