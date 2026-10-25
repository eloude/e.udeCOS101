use std::io::Write;
use std::io;

fn main() {
    let mut file = std::fs::File::create("efcc_merge.csv").expect("could not create file");
    file.write_all("COMMISSIONER,MINISTRY,ZONE\n".as_bytes()).expect("could'nt write");

    'outer: loop {
    let vec_mster = minister();
    let vec_mstry = mnstry();
    let vec_z = zone();

    for i in 0..vec_mster.len() {
        file.write_all(vec_mster[i].as_bytes()).expect("could'nt write");
        file.write_all(vec_mstry[i].as_bytes()).expect("could'nt write");
        file.write_all(vec_z[i].as_bytes()).expect("could'nt write");
    }

    println!("\nDo you still have any ministers you want to add");
    loop {

    let repeat = input().to_uppercase();
    match repeat.as_str() {
        "YES" | "Y" => break,
        "NO" | "N" => break 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
    }
   }
 }
}

fn input()->String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}

fn minister()->Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    'outer: loop {

    println!("\nEnter the full name of the convicted minister");
    let mut name = input();
    name.push(',');
    
    vec.push(name);

    println!("\nDo you want to input any other name");

    loop {

    let repeat = input().to_uppercase();
    match repeat.as_str() {
        "YES" | "Y" => break,
        "NO" | "N" => break 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
    }
   }
 };
 vec
}

fn mnstry()->Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    'outer: loop {

    println!("\nEnter the ministry in the same order as the names");
    let mut ministry = input();
    ministry.push(',');
    
    vec.push(ministry);

    println!("\nDo you want to input any other ministry");
    
    loop {

    let repeat = input().to_uppercase();
    match repeat.as_str() {
        "YES" | "Y" => break,
        "NO" | "N" => break 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
    }
   }
 };
 vec
}

fn zone()->Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    'outer: loop {

    println!("\nEnter the zone(s) of the convicted minister(s)");
    let mut zone = input();
    zone.push('\n');
    
    vec.push(zone);

    println!("\nDo you want to input any other zone");
    
    loop {

    let repeat = input().to_uppercase();
    match repeat.as_str() {
        "YES" | "Y" => break,
        "NO" | "N" => break 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
    }
   }
 };
 vec
}