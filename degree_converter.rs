//Fahrenheit to Celsius Or Celsius to Fahrenheit

//unSOLVED: String to f64
use std::io;

fn main() {
    print!("Input Degree (EX: 5.0): ");

    let mut temp= String::new();
    io::stdin().read_line(&mut temp)
        .expect("ERROR");

    temp.trim().split_whitespace().parse::<f64>().unwrap();

    loop {
        print!("What is it? (Fahrenheit = 0 / Celsius = 1): ");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)
            .expect("ERROR");

        if answer.trim().parse() == Ok(0) {
            println!("{} Fahrenheit Degree is converted to {} Celsius Degree.", temp, f_to_c(temp));
            break;
        } else if answer.trim().parse() == Ok(1) {
            println!("{} Celsius Degree is converted to {} Fahrenheit Degree.", temp, c_to_f(temp));
            break;
        }

        println!("Please Input 0 or 1.");
    }
}

fn f_to_c( x :f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn c_to_f( x :f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}