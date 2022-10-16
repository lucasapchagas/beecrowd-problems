use std::io;

fn main() {
    let mut inputs;
    let mut result : f64 = 0.0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();
    
    result += inputs[1].trim().parse::<f64>().unwrap() * inputs[2].trim().parse::<f64>().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();
    
    result += inputs[1].trim().parse::<f64>().unwrap() * inputs[2].trim().parse::<f64>().unwrap();

    println!("VALOR A PAGAR: R$ {:.2}", result);
}