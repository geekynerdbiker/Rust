fn main() {
    let tup = (100, 31.9, "Third");
    let (var1, var2, var3) = tup;
    let (var4, var5, var6) = (tup.2, tup.0, tup.1);

    println!("var1 = {}, var2 = {}, var3 = {}", var1, var2, var3);
    println!("var4 = {}, var5 = {}, var6 = {}", var4, var5, var6);

    let arr1 = [3; 3];
    let arr2: [i32; 5] = [ 1, 9, 2, 8, 3];

    println!("\narr1[0] = {}, arr2[1] = {}", arr1[0], arr2[1]);
}