use std::io;

fn main() {

    //outer loop to make it possible for it to be asked another quesion

    'outer: loop{

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    


   // input x²
    println!("\nInput a number for the coefficient of x²: ");
    io::stdin().read_line(&mut input1).expect("Put in an appropriate number value");
    let a:f64 = input1.trim().parse().expect("Put in an appropriate number value");

   // input x
    println!("\nInput a number for the coefficient of x: ");
    io::stdin().read_line(&mut input2).expect("Put in an appropriate number value");
    let b:f64 = input2.trim().parse().expect("Put in an appropriate number value");

   // input constant  
    println!("\nInput the value of the constant: ");
    io::stdin().read_line(&mut input3).expect("Put in an appropriate number value");
    let c:f64 = input3.trim().parse().expect("Put in an appropriate number value");

  // find d
    let d:f64 = b * b - 4.0 * a * c ;


  // if statement and if a = 0


  if a == 0.0 {
    println!("\nThe coefficient of x² cannot be 0 in a quadratic equation");
  }

  else {
      
  // if real root  
    if d > 0.0 {
        let x1:f64 = (-b + d.sqrt()) / (2.0 * a);
        let x2:f64 = (-b - d.sqrt()) / (2.0 * a);

        println!("\nX1 = {} \nX2 = {}", x1, x2);
    } 

   // if equal root
    else if d == 0.0{
        let x:f64 = -b / (2.0 *a);

        println!("\nX = {} (twice)", x);
    }


   // if imaginary roots
   else {
    println!("\nThis equation has no real roots.");
   } 

}

   // prompt to make it repeat

   println!("\nDo you have another equation for me ?\n(Input (1) if yes and (0) if no)");

   // inner loop to make the input either 1 or 0 
   loop {

   let mut repeat = String::new();
    io::stdin().read_line(&mut repeat).expect("Put in either (1) or (0)");
    let repeat:i8 = repeat.trim().parse().expect("Put in either (1) or (0)");
    

    if repeat == 0 {
        println!("\nThanks for using my quadratic equation calculator!!!");
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