use std::io;

fn main() {

   'outer: loop {

    let mut name = String::new();
    let mut age = String::new();
    
    
    println!("\nPlease enter the employee's name"); 
    io::stdin().read_line(&mut name).expect("Invalid input");
    let name = name.trim();

    println!("\nEnter the employee's age");
    io::stdin().read_line(&mut age).expect("Enter an appropriate string input");
    let age:u8 = age.trim().parse().expect("Enter a number");


    println!("\nAre they experienced ?\n(1) if yes and (0) if no");

    loop {
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Enter an appropriate string input");
    let exp:u8 = exp.trim().parse().expect("Enter either (1) or (0)");

     if exp == 0{
      println!("\nAnnual incentive for {} is going to be ₦100,000", name);
        break
     }

     else if exp == 1{
       match age {
            ..=29 => println!("\nAnnual incentive for {} is going to be ₦1,300,000", name),
            30..=39 => println!("\nAnnual incentive for {} is going to be ₦1,480,000", name),
            40.. => println!("\nAnnual incentive for {} is going to be ₦1,560,000", name),
        }

        break
     }

     else {
        println!("\nplease enter either (1) or (0)");
     }
    }

    println!("\nWould u like to get the incentive of any other employee ?\nInput (1) if yes and (0) if no");

    loop {

   let mut repeat = String::new();
    io::stdin().read_line(&mut repeat).expect("Put in either (1) or (0)");
    let repeat:i8 = repeat.trim().parse().expect("Put in either (1) or (0)");
    

    if repeat == 0 {
        println!("\nThanks for using my incentive calculator!!!");
        break 'outer;
    }

    else if repeat == 1 {
        break;
    }

    else {
        println!("\nENTER IN STRICTLY (1) OR (0)");
        }
    }
  }
}
