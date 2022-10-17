use std::io;

fn main() {
    let mut inputs;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();

    let x1 : f64 = inputs[0].trim().parse::<f64>().unwrap();
    let y1 : f64 = inputs[1].trim().parse::<f64>().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();

    let x2 : f64 = inputs[0].trim().parse::<f64>().unwrap();
    let y2 : f64 = inputs[1].trim().parse::<f64>().unwrap();

    let result : f64 = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();

    println!("{:.4}", result);
}
