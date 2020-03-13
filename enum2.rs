enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn main() {
    let penny = Coin::Penny;
    let result = value_in_cents(penny);

    println!("penny is {} cent.", result);
}

fn value_in_cents(coin: Coin) -> isize {
    match coin {
        Coin::Penny => {
            println!("Lucky Coin!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25,
    }
}
