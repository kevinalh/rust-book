use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    // We use mut to make the variable mutable (since they're immutable by default)
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) // References are also immutable by default!
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
