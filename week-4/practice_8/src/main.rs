fn main() {
   //while true

   let mut x:i8 = 0;

   loop {
       x += 1;
       println!("x = {}", x);

       if x == 15 {
        break
       }
   }
}
