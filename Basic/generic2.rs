struct Point1<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let point1 = Point1 { x: 4, y: 7 };
    let point2 = Point2 { x: 6, y: 5.4 };
    let point2_2 = Point2 { x: 4, y: 7 };
}