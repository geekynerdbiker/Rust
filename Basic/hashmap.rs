use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::new();

    map1.insert("A", 45);
    map1.insert("D", 68);
    map1.insert("G", 71);

    // Randomly printed.
    print!("map1 = {:#?}", map1);
    //Overwrite
    map1.insert("A", 65);
    // Randomly printed.
    print!(" map1 = {:#?}", map1);

    let v1 = vec!["alpha", "beta", "delta", "omega"];
    let v2 = vec![11, 22, 44];

    let mut map2 :HashMap<_, _> = v1.iter().zip(v2.iter()).collect();

    // Randomly printed.
    print!(" map2 = {:#?}", map2);
    // Add value of empty key.
    map2.entry(&"alpha").or_insert(&44);
    map2.entry(&"omega").or_insert(&66);
    // Randomly printed.
    print!(" map2 = {:#?}", map2);

    let mut map3 = HashMap::new();
    let text = "apple banana cat cat banana cat apple cat dog";

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }

    // Randomly printed.
    print!(" map3 = {:#?}", map3);

    // v1 and v2 is still alive.
}