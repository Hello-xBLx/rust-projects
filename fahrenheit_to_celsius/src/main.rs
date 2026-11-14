use std::io;

fn main() {
	println!("Enter the temperature in Fahrenheit to convert it to Celsius");
	let mut fahrenheit = String::new();

	io::stdin()
		.read_line(&mut fahrenheit)
		.expect("Fail read line");

	let fahrenheit: f64 = fahrenheit.trim().parse().expect("Enter a number");

	let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

	println!("Your number in degrees Celsius is: {celsius:.2}");
}