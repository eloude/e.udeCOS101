
use std::io;

fn input() -> f64 {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");

    input.trim().parse().expect("Please enter a valid number")
}

fn trapezium() -> f64 {
    println!("\nEnter height:");
    let height = input();

    println!("\nEnter longer parallel side:");
    let base1 = input();

    println!("\nEnter shorter parallel side:");
    let base2 = input();

    height / 2.0 * (base1 + base2)
}

fn rhombus() -> f64 {
    println!("\nEnter first diagonal:");
    let d1 = input();

    println!("\nEnter second diagonal:");
    let d2 = input();

    0.5 * d1 * d2
}

fn parallelogram() -> f64 {
    println!("\nEnter base:");
    let base = input();

    println!("\nEnter perpendicular altitude:");
    let height = input();

    base * height
}

fn cube() -> f64 {
    println!("\nEnter side length:");
    let side = input();

    6.0 * side * side
}

fn cylinder() -> f64 {
    println!("\nEnter radius:");
    let radius = input();

    println!("\nEnter height:");
    let height = input();

    3.1416 * radius * radius * height
}

fn main() {
    println!(" SHAPE CALCULATOR ");
    println!("1 - Trapezium: area");
    println!("2 - Rhombus: area");
    println!("3 - Parallelogram: area");
    println!("4 - Cube: surface area");
    println!("5 - Cylinder: volume");
    println!("");
    println!("Enter the number that corresponds with what you want to calculate");

    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("Input error");

    let solution = match decision.trim() {
        "1" => trapezium(),
        "2" => rhombus(),
        "3" => parallelogram(),
        "4" => cube(),
        "5" => cylinder(),
        _ => {
            println!("\nInvalid choice.");
            return;
        }
    };

    println!("\nThe answer is: {}", solution);
}