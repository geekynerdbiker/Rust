use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let answer = rand::thread_rng().gen_range(1, 101);
    println!("Guess Number!");

    loop {
        println!("Input Number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Couldn't Read.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Number: {}", guess);

        match guess.cmp(&answer) {
            Ordering::Less => println!("It's low"),
            Ordering::Greater => println!("It's high"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
