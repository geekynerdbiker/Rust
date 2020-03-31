struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mix_up<V, W> (self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point = Point { x: 3, y: 1 };

    println!("( point.get_x(), point.y ) : ({}, {})", point.get_x(), point.y);

    let point1 = Point2 { x: 3, y: 1.0 };
    let point2 = Point2 { x: "Hello", y: 'z' };

    let point3 = point1.mix_up(point2);

    println!("(point3.x, point3.y) = ({}, {})", point3.x, point3.y);
}