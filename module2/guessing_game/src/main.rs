// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
// A crate is a collection of Rust source code files
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type, which creates a new,empty string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents)

        let guess: u32 = match guess.trim().parse() { //The parse method on strings converts a string to another type.
            Ok(num) => num,
            Err(_) => continue, //The underscore, _, is a catch-all value; in this example, we’re saying we want to match all Err values
        }; 

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("guess higher"),
            Ordering::Greater => println!("guess lower"),
            Ordering::Equal => {
                println!("congrats you won");
                break;
            }
        }
    }
}

// cargo doc --open
