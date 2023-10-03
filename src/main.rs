use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("The secret number is : {secret_number}");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number.to_string()) {
        Ordering::Less => println!("You guessed too small!"),
        Ordering::Equal => println!("Congratulations! You guessed correct."),
        Ordering::Greater => println!("You guessed too big!"),
    }
}
