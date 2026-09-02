fn main() {
	let p:f64 = 1000.00;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	//simple interest
	let si = p * ( r / 100.0 ) * t;
	println!("Simple Interest is {}", si) ;

	let a = si + p;
	println!("Amount is {}", a);
}