fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is: {}", num);
}
fn mutate_num_to_zero(num:&mut i32) {
    *num = *num*0;
    println!("parametr num value is {}", num);
}