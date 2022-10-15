use std::io;

fn main() {
    let mut x : i32;
    let mut y : i32;

    let mut input;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = input.trim().parse::<i32>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = input.trim().parse::<i32>().unwrap() * x;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    y = input.trim().parse::<i32>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    y = input.trim().parse::<i32>().unwrap() * y;

    println!("DIFERENCA = {}", x - y);
}