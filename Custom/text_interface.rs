use std::io;

fn main() {
    check_str(&input_stream());
}

fn input_stream() -> String {
    let mut s = String::new();

    io::stdin().read_line(&mut s)
        .expect("Input Error.");

    s
}

fn check_str(s :&str) {
    let mut v = Vec::new();

    for word in s.split_whitespace() {
        v.push(word);
    }

    let add = match v.get(0) {
        Some(str) => str,
        None => panic!("Input Error"),
    };

    let to = match v.get(2) {
        Some(str) => str,
        None => panic!("Input Error"),
    };

    if add.matches("add") && to.matches("to") {
        print!("Success");
    }
}