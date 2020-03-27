use std::io;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // panic3();
    print!("{:?}", panic_carrier4());
}

fn panic1() {
    panic!("Crash!");
}

fn panic2() {
    let v = vec![1, 2, 3];

    v[99];
}

fn panic3() {
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
}

fn panic4() {
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Couldn't create file: {:?}"),
            },
            other_error => panic!("Couldn't open file: {:?}", other_error),
        },
    };
}

fn panic5() {
    let f = File::open("file.txt").unwrap();
}

fn panic6() {
    let f = File::open("file.txt").expect("Couldn't open file.");
}

fn panic_carrier1() -> Result<String, io::Error> {
    let f = File::open("file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn panic_carrier2() -> Result<String, io::Error> {
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn panic_carrier3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn panic_carrier4() -> Result<String, io::Error> {
    let s = fs::read_to_string("file.txt")?;

    Ok(s)
}