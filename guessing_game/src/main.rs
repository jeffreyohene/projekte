// Number guesser. First Rust program - jeffreyohene
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("You thief, you dare show yourself at the gates of the treasury of the great King Midas? Make a guess!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("What is your guess, thief?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("You dare enter a wrong code type? Enter a nmber between 1 - 100");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Not quite!"),
            Ordering::Greater => println!("Too big. Try again!"),
            Ordering::Equal => {
                println!("Welcome master, take whaever you want, I am yor humble servant!!");
                break;
            }
        }
    }
}