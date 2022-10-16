use std::io; 

fn main() {
    let mut x : f64;
    let mut input;

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = input.trim().parse::<f64>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    x = x + (input.trim().parse::<f64>().unwrap() * 0.15f64) ;

    println!("TOTAL = R$ {:.2}", x);
}