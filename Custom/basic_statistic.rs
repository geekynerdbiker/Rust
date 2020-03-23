use std::io;

fn main() {
    let mut v1 = Vec::new();

    let mut var = input_stream();
    while var > 0 {
        v1.push(var);
        var = input_stream();
    };

    println!("{}", average(v1));
}

fn input_stream() -> isize {
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Couldn't Read.");

    let num: isize = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => -1,
    };

    num
}

// Just Sum
fn average(v :Vec<isize>) -> isize {
    let mut sum = 0;
    let len = v.len();

    for i in 0..len {
        match v.get(i) {
            Some(&num) => sum += num,
            None => panic!("Index Overflow."),
        }
    }
    sum
}