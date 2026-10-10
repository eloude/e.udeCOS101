fn main() {
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}", arr);

    for val in arr.iter() {
        println!("value is :{}", val);
    }
}
