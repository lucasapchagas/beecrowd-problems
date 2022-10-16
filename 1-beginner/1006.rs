use std::io;

fn main() {
    let mut x : f64;
    
    let mut input;
    
    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = input.trim().parse::<f64>().unwrap() * 2f64;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");       
    x = x + input.trim().parse::<f64>().unwrap() * 3f64;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");       
    x = x + input.trim().parse::<f64>().unwrap() * 5f64;

    x = x / 10f64;

    println!("MEDIA = {:.1}", x);
}