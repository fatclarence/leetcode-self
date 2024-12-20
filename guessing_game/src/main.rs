use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("In put your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("You guessed: {}", guess);
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
    }
}

