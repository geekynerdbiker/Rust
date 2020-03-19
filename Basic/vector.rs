struct Coor {
    x :isize,
    y :isize,
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v2 :Vec<isize> = Vec::new();
    let mut v3:Vec<Coor> = Vec::new();

    v2.push(7);
    // v1.push(3);
    // not mutable

    println!("Size of v1 is {}.", v1.len());

    v2.push(11);

    match v2.get(1) {
        Some(&num) => println!("Good, It's {}!", &num),
        None => println!("Bad"),
    }

    v2.push(1);
    v3.push(Coor {
        x: 2,
        y: 4,
    });

    println!("\"The First Coordinate: ({}, {})\"", v3[0].x, v3[0].y);
}