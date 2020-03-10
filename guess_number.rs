use std::io;

fn main() {
    println!("Guess Number!");
    println!("Input Number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Couldn't Read.");

    println!("Your Number: {}", guess);
}
