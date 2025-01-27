use std::io;

fn main() {

    println!("Guess the Number!");

    println!("Please input your guess.");

    // Mut is used to make the variable mutable
    let mut guess = String::new();
    
    // :: is used to call a function from a module
    io::stdin() 
    // & is a reference to the variable
        .read_line(&mut guess) 
    // expect is used to handle errors
        .expect( "Failed to read line"); 

    // {} is a placeholder for the value of the variable
    println!("You guessed: {}", guess);

}
