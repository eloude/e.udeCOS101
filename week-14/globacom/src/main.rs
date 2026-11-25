use std::io::Read;
use std::fs::File;

fn main() {
    println!("What is your role in this company?");
loop {
    let mut role = String::new();
    std::io::stdin().read_line(&mut role).expect("Invaliud input");
    let role = role.trim().to_lowercase();

        match role.as_str() {
        "admin" |"administrator" => break admin(),
        "project manager" => break p_manager(),
        "employee" => break employee(),
        "customer" => break customer(),
        "vendor" => break vendor(),
        _ => {
            println!("\nInvalid role");
            println!("Try again\n");
        }
    }
  };
}

fn read_file(mut file: File) {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Invaliud input");
    println!("{}",contents);
}

fn admin() {
    let file = File::open("globacom_db.sql").expect("Invalid input");
    read_file(file);
}

fn p_manager() {
    let file = File::open("project_tb.sql").expect("Invalid input");
    read_file(file);
}

fn employee() {
    let file = File::open("staff_tb.sql").expect("Invalid input");
    read_file(file);
}

fn customer() {
    let file = File::open("customer_tb.sql").expect("Invalid input");
    read_file(file);
}

fn vendor() {
    let file = File::open("dataplan_tb.sql").expect("Invalid input");
    read_file(file)
}