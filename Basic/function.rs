fn main() {
    print_hello();
    add_printer(3, 5);

    let y = {
        let x = 4;
        x + 9
    };

    println!("y = {}, tuple_return.0 = {}, tuple_return.1 = {}", y, returner(y).0, returner(y).1);
}

fn print_hello() {
    println!("Hello!");
}

fn add_printer( x :i32, y :i32 ) {
    println!("Sum is {}.",x+y);
}

fn returner(x: i32) -> (i32, i32) {
    (x+7, x+55)
}