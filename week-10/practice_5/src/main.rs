fn main() {
    // a list of nos
    let x = vec![15,25,35,45,55];
    borrow_vector(&x);//passing reference
    println!("Printing the value from main() x[0]={}", x[0]);
    println!("*******************************");
}

fn borrow_vector(x:&Vec<i32>) {
    println!("********************************");
    println!("inside print_vector function {:?}", x);
    println!("---------------------------------");
}
