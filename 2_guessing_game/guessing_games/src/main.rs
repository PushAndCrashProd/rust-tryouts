use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random number using the rand crate.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your value: ");

        // All variables are immutable by default.
        let mut guess = String::new();


        // Like stdio in C, but here expect is required for all
        // functions that return a result which can be either
        // Ok or Error, expect either handles ok by giving value or
        // crashes if its an error.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // String interpolation is achieved by {}
        println!("You guessed: {guess}");

        // Rust allows shadowing of variables (Same name change data type).
        // it automatically does type inference and if specified
        // conversion as well.
        let guess: u32 = guess
                         .trim()
                         .parse()
                         .expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => { 
                println!("You Win!!");
                break;
            }
        }
    }
}
