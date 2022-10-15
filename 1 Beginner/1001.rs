use std::io;

fn main() {
    let mut input;
    
    input = String::new();
    
    io::stdin().read_line(&mut input).expect("Reading error");
    let mut x: i32 = input.trim().parse().unwrap();

    input = String::new();

    io::stdin().read_line(&mut input).expect("Reading error");
    let y: i32 = input.trim().parse().unwrap();

    x += y;

    println!("X = {}", x);
}