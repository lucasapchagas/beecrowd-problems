use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let x : i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let y : f64 = input.trim().parse().unwrap();

    println!("{:.3} km/l", x as f64 / y);
}