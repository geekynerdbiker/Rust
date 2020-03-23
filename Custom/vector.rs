use std::io;

fn main() {
    let mut v1 = Vec::new();

    loop {
        let var = input_stream();
        if var == -1 {
            return
        }
        v1.push(var);
    }

}

fn input_stream() -> isize {
    print!("Input Number (QUIT: -1): ");

    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Couldn't Read.");

    let num: isize = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => -1,
    };
    num
}