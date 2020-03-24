fn main() {
    let s = "Hello";

    println!("{}", pig_latin(s));
}

fn pig_latin(s :&str) -> String{
    let mut pl = String::new();
    match s.chars().nth(0) {
        a => {
            pl.push_str(s);
            pl.push_str("-hay");
        },
        _ => {
            pl.push_str(&s[1..]);
            pl.push('-');
            pl.push(s.chars().nth(0));
            pl.push_str("ay");
        },
    }

    pl
}