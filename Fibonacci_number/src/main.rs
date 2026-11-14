use std::io;

fn main() {
    println!("Enter the nth Fibonacci number");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Fail read line");

    let n: i32 = n.trim().parse().expect("Not number");

    let fi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let psi = (1.0 - 5.0_f64.sqrt()) / 2.0;

    let res = (fi.powi(n) - psi.powi(n)) / 5.0_f64.sqrt();

    println!("The {n} Fibonacci number is: {res:.0}");
}
