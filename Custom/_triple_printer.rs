fn main() {
    for i in 1..1000 {
        for j in 1..1000 {
            for k in 1..1000 {
                let mut a = i; let mut b = 1;

                while b < j && a <= k {
                    a *= i;
                    b += 1;
                }

                if a == k {
                    println!("({}, {}, {})", i, j, k);
                }
            }
        }
    }
}
