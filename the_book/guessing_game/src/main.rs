use std::io; // bring input-output library into scope
use rand::Rng; //bring Rng trait into scope

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}!");

    println!("Enter your guess:");

    let mut guess: String = String::new(); // mutable types allow variable values to change

    io::stdin()
        .read_line(&mut guess)//move value into guess, returns a Result type
        .expect("Failed to read guess"); // handle exceptions; method of Result type

    println!("You guessed: {guess}");
}
