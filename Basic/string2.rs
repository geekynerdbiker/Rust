fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("안녕하세요");

    println!("{}, {}", s1.len(), s2.len());

    for c in s1.chars() {
        print!("{} ", c);
    }
    println!();
    for c in s2.chars() {
        print!("{} ", c);
    }
    println!();

    for b in s1.bytes() {
        print!("{} ", b);
    }
    println!();
    for b in s2.bytes() {
        print!("{} ", b);
    }
    println!();
}