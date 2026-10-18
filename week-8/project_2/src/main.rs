use std::io;

fn main() {
    let mut users: Vec<(String, usize)> = Vec::new();
   'outer: loop  {
    println!("\nEnter your name");
    let name = input();
    println!("\nEnter years of programming experience");
    let years: usize = input().parse().expect("Invalid input");
    let user = (name,years);
    users.push(user);

println!("\nIs there any other candidate for the intervview. (Y/N)");    
loop {
    let repeat = input().to_uppercase();
    match repeat.as_str() {
        "Y" | "YES" => break,
        "N" | "NO" => break 'outer,
        _ => {
            println!("\nInvalid input");
            println!("Try again");
        }
    }
   }
  };

  let mut highest: Vec<usize> = Vec::new();
  let mut most_exp = 0;
  
  for i in 0..users.len() {
    if users[i].1 > most_exp {
        highest.clear();
        most_exp = users[i].1;
        highest.push(i);
    }
    else if users[i].1 == most_exp {
        most_exp = users[i].1;
        highest.push(i);

    }
  }

  for i in 0..highest.len() {
    println!("\nCongratulations {} on getting the job!", users[highest[i]].0);
  }  
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}
