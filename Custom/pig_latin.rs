use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s)
        .expect("Input Error.");

    println!("{}", pig_latin(&s));
}

fn pig_latin(s :&str) -> String{
    let mut pl = String::new();
    let ch = match s.chars().nth(0) {
        Some(ch) => ch,
        None => panic!("ERR"),
    };

    if ( ch == 'a' ) || ( ch == 'e' ) || ( ch == 'i' ) || ( ch == 'o' ) || ( ch == 'u' ) {
        pl.push_str(s);
        pl.push_str("-hay");
    }
    else {
        pl.push_str(&s[1..]);
        pl.push('-');
        pl.push(ch);
        pl.push_str("ay");
    }

    pl
}