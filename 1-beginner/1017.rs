use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let time : i32 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let velocity : i32 = input.trim().parse().unwrap();

    println!("{:.3}", (time * velocity) as f64 / 12.0);
}
