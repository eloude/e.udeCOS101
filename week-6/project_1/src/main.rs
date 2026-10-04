use std::io;

fn main() {

let mut total = 0.0;

'outer: loop {
    println!("\n========== RESTAURANT MENU ==========\n");
    println!("P - Poundo Yam / Edinkaiko Soup   N3200");
    println!("F - Fried Rice & Chicken          N3000");
    println!("A - Amala & Ewedu Soup            N2500");
    println!("E - Eba & Egusi Soup              N2000");
    println!("W - White Rice & Stew             N2500");
    println!("");
    println!("Enter either: P, F, A, E or W and click enter");
   
    let price = loop {
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("Enter a String input");
    order = order.trim().to_uppercase();
    
    match order.as_str() {
        "P" => break 3200.0,
        "F" => break 3000.0,
        "A" => break 2500.0,
        "E" => break 2000.0,
        "W" => break 2500.0,
        _ => {
            println!("\nInvalid order!!");
            println!("Try again");
        },
     };
   }; 

    let mut qty = String::new();
    println!("\nHow many are you ordering?");
    io::stdin().read_line(&mut qty).expect("Enter a String input");
    let qty:f64 = qty.trim().parse().expect("Enter a number");

    let i_total = price * qty;

    total += i_total;

    println!("\nAnything else? (Yes/No)");
    loop {
    let mut glutton = String::new();
    io::stdin().read_line(&mut glutton).expect("Enter a String input");
    glutton = glutton.trim().to_uppercase();

    match glutton.as_str() {
        "Y" => break,
        "YES" => break,
        "N" => break 'outer,
        "NO" => break 'outer,
        _ => println!("\nEnter either yes/no")
    }
   }
  }

let d_total;

  if total > 10000.0 {
     d_total = total * 0.05;
  }
  else {
     d_total = 0.0;
  }

  let final_total = total - d_total;

    println!("\n============== BILL ==============\n");
    println!("Total:            ₦{}", total);
    println!("Discount:         ₦{}", d_total);
    println!("Amount to pay:    ₦{}", final_total);
}