struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let point = Point { x: 3, y: 1 };

    println!("point.get_x() : {}", point.get_x());
}