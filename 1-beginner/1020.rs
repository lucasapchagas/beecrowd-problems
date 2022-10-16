use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let input_days : i32 = input.trim().parse().unwrap();
    
    let years : i32 = input_days / 365;
    let months : i32 = (input_days % 365) / 30;
    let days : i32 = (input_days % 365) % 30;

    println!("{} ano(s)", years);
    println!("{} mes(es)", months);
    println!("{} dia(s)", days);
}
