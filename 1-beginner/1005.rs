use std::io;

fn main() {
    let mut a : f64;
    let b : f64;
    
    let mut input;
    
    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    a = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");       
    b = input.trim().parse().unwrap();

    a = ((a * 3.5f64) + (b * 7.5f64)) / 11f64;

    println!("MEDIA = {:.5}", a);
}