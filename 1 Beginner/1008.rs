use std::io; 

fn main() {
    let n : i32;
    let mut x : f64;
    let mut input;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    n = input.trim().parse::<i32>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = input.trim().parse::<f64>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x *= input.trim().parse::<f64>().unwrap();

    println!("NUMBER = {}\nSALARY = U$ {:.2}", n, x);
}