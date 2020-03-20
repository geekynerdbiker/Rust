fn main() {
    let mut s1 = String::new();
    s1 = String::from("Hello");

    s1.push_str(", World");
    s1.push('!');

    println!("{}", s1);
}