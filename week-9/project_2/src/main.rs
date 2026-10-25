use std::io::Write;
use std::io;


fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("couldnt create file");
    file.write_all("NAME,MATRIC NO,DEPARTMENT,LEVEL\n".as_bytes()).expect("could not write");

    'outer: loop {

    let name = std_name();
    let mat_number = matric();
    let dep = department();
    let lvl = level();

    let student_id = format!("\n{},{},{},{}",name,mat_number,dep,lvl);

    let student = vec![name,mat_number,dep,lvl.to_string()];

    println!("\nName: {}", student[0]);
    println!("Matric No: {}", student[1]);
    println!("Department: {}", student[2]);
    println!("Level: {}", student[3]);


    println!("\nAre these details correct");
    loop {
    let correct = input().to_uppercase();
    match correct.as_str() {
        "YES" | "Y" => break,
        "NO" | "N" => continue 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
     }
   }

    file.write_all(student_id.as_bytes()).expect("could not write");

    println!("\nDo you want to input the details for any other students");
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
  println!("\nYour file has been created successfully");
}



fn input() ->String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}



fn std_name() ->String {

    println!("\nEnter your first name");
    let mut first_name = input();

    println!("\nEnter your last name");
    let last_name = input();

    first_name.push(' ');
    first_name.push_str(last_name.as_str());

    first_name
}




fn matric()->String {
    println!("\nEnter your matric number(eg: ACC10211111)");

    let m_number = loop {

    let mat = input().to_uppercase();
    match mat.len() {
        11 => break mat,
        _ => {
            println!("\nInput an appropriate matric number");
        }
    }
  };
  m_number.to_string()
}



fn department()->String {
    println!("\nEnter your department(eg: Software Engineering)");
    let dep = input();
    dep
}




fn level() -> u16{
    println!("\nWhat level are you in currently(eg: 100,200)");
    loop {
    let lvl:u16 = input().parse().expect("Invalid input"); 
    match lvl {
        100 => break 100,
        200 => break 200,
        300 => break 300,
        400 => break 400,
        500 => break 500,
        _ => {
            println!("\nEnter a valid level(eg: 100,200)");
        }
    }
  }
}


