#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
    // cannot make any function in struct
}

impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }

    fn can_hold(&self, rct: &Rectangle) -> bool {
        rct.area() < self.area()
    }

    fn squre(size: usize) -> Rectangle {
        Rectangle {width: size, height: size }
    }
}

fn main() {
    let rct = Rectangle { width: 30, height: 50 };
    let rct2 = Rectangle { width: 20, height: 60 };

    println!(
        "{:?} can hold {:?}? \n{}\n",
        rct, rct2, rct.can_hold(&rct2)
    );

    let sq = Rectangle::squre(5);
    println!("{:#?} is squre!", sq);
}

// {:?} is print in 1 line.
// {:#?} is print more dynamic.
// both need #[derive(Debug)]