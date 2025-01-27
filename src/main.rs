use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    loop {
        println!("Please input your guess.");

        // rand::rng() is used to generate a random number between 1 and 100
        let secret_number = rand::rng().random_range(1..101);

        println!("The secret number is: {}", secret_number);

        // Mut is used to make the variable mutable
        let mut guess = String::new();
        // :: is used to call a function from a module
        io::stdin()
            // & is a reference to the variable
            .read_line(&mut guess)
            // expect is used to handle errors
            .expect("Failed to read line");

        // Shadowing is used to change the type of the variable
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // match is used to handle errors and return a value based on the condition of the variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} is a placeholder for the value of the variable
        println!("You guessed: {}", guess);

        println!("The secret number is: {}", secret_number);

        // match is used to handle multiple conditions
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
