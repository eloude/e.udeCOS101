fn main () {
	let at:f64 = 450000.0;
	let am:f64 = 1500000.0;
	let ah:f64 = 750000.0;
	let ad:f64 = 2850000.0;
	let aa:f64 = 250000.0;
	let qt:f64 = 2.0;
	let qm:f64 = 1.0;
	let qh:f64 = 3.0;
	let qd:f64 = 3.0;
	let qa:f64 = 1.0;

	let qty = qt + qm + qa + qd + qh;

	let sum = (at * qt) + (am * qm) + (ah * qh) + (ad * qd) + (aa * qa);
	println!("The sum is {}", sum);

	let avg = sum / qty;
	println!("The average is {}", avg);
}