use std::io;
use std::convert::TryInto;

fn main() {
    let mut v1 = Vec::new();

    let mut var = input_stream();
    loop {
        v1.push(var);
        var = input_stream();
    };

    println!("{}", average(v1));
}

fn input_stream() -> f64 {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Couldn't Read.");

    let num: f64 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Input Error!"),
    };

    num
}

// Just Sum
fn average(v :Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let len = v.len();

    for i in 0..len {
        match v.get(i) {
            Some(&num) => sum += num,
            None => panic!("Index Overflow."),
        }
    }
    sum / 5.0
}