use std::{io, cmp::Ordering}; // bring input-output library into scope
use rand::Rng; //bring Rng trait into scope


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is {secret_number}!");

    loop {
        println!("Enter your guess:");

        let mut guess: String = String::new(); // mutable types allow variable values to change

        io::stdin()
            .read_line(&mut guess)//move value into guess, returns a Result type
            .expect("Failed to read guess"); // handle exceptions; method of Result type

        // shadow the previous guess with an unsigned integer version
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //parse returned an Ok value
            Err(_) => continue, //parse returned an error value; underscore is a catch-all
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
