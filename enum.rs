enum Messege {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messege {
    fn call(&self) {
        //
    }
}

fn main() {
    let m = Messege::Write(String::from("Hello"));

    m.call();

    let op1 :Option<isize> = None;
    let op2 = Some("Option::Some(var)");
    // let op3 = None; doesn't work.
}