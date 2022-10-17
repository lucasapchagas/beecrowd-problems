use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let input_seconds : i32 = input.trim().parse().unwrap();
    let mut remaining;

    let hours : i32 = input_seconds / 3600;
    remaining = input_seconds % 3600;

    let minutes : i32 = remaining / 60;
    remaining = remaining % 60;

    println!("{}:{}:{}", hours, minutes, remaining);
}
