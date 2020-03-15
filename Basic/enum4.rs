fn main() {
    let some_u8_value = Some(3u8);

    println!("With match syntax below.");
    match some_u8_value {
        Some(3) => println!("3!"),
        _ => (),
    }

    println!("\nWith if let syntax below.");
    if let Some(3) = some_u8_value {
        println!("3!");
    } else {
        ();
    }
}
