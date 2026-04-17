use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("🎯 Welcome to Guessing Game!");
    println!("I'm thinking of a number range 0 - 10... can you guess it?");
    
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=10);
    let mut attempt = 5;

    while attempt > 0 {
        print!("\n👉 Enter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to readline");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Please type a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("📉 Too small!");
            }
            std::cmp::Ordering::Greater => {
                println!("📈 Too big!");
            }
            std::cmp::Ordering::Equal => {
                println!("🎉 Correct! You won with {} attempts left!", attempt - 1);
                return;
            }
        }

        attempt -= 1;
        
        if attempt > 0 {
            println!("🧠 Attempts left: {}", attempt);
        }
    }

    println!("💀 Game Over! The number was {}", secret_number);
}