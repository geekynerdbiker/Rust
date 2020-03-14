// Don't know about crate yet.
use crate::UsState::{Oklahoma, New_Mexico, Utah, Washington, Colorado};

#[derive(Debug)]
// recommand upper camel case name.
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, California, Colorado, Connecticut, Delaware,
    Florida, Georgia, Hawaii, Idaho, Illinois, Indiana, Iowa, Kansas, Kentucky, Louisiana,
    Maine, Maryland, Massachusetts, Michigan, Minnesota, Mississippi, Missouri, Montana,
    Nebraska, Nevada, New_Hampshire, New_Jersey, New_Mexico, New_York, North_Carolina,
    North_Dakota, Ohio, Oklahoma, Oregon, Pennsylvania, Rhode_Island, South_Carolina,
    South_Dakota, Tennessee, Texas, Utah, Vermont, Virginia, Washington, West_Virginia,
    Wisconsin, Wyoming,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
    Setter,
}

fn main() {
    let penny = Coin::Penny;
    let mut result = value_in_cents(penny);

    println!("penny is {} cent.", &result);

    let q1 = value_in_cents(Coin::Quarter(Oklahoma));
    let q2 = value_in_cents(Coin::Quarter(New_Mexico));
    let q3 = value_in_cents(Coin::Quarter(Utah));
    let q4 = value_in_cents(Coin::Quarter(Washington));
    let q5 = value_in_cents(Coin::Quarter(Colorado));
}

fn value_in_cents(coin: Coin) -> isize {
    match coin {
        Coin::Penny => {
            println!("Lucky Coin!\n");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        // Binding.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 1,
    }
}