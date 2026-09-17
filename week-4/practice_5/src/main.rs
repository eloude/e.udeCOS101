use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter Your Height (in Centimetres)");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <=170.0 {
        println!("You are of average height");
    }

    else if height > 170.0 && height < 190.0 {
        println!("You are tall");
    }

    else if height < 100.0 && height > 150.0 {
        println!("You are short");
    }

    else {
        println!("Abnormal height");
    }
}
