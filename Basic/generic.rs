fn main() {
    let list1 = vec![34, 54, 76, 12, 92, 30];
    let list2 = vec!['b', 'w', 'Q', 'z', 's', 'o'];

    let _result1 = largest_i32(&list1);
    let result2 = largest_char(&list2);

    println!("The largest is \'{}\'", result2);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut result = list[0];

    for &item in list.iter() {
        if item > result {
            result = item;
        }
    }

    result
}

// ASCII Order
fn largest_char(list: &[char]) -> char {
    let mut result = list[0];

    for &item in list.iter() {
        if item > result {
            result = item;
        }
    }

    result
}