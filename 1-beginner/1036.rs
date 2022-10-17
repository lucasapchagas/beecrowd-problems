use std::io;

fn main() {
    let inputs;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();

    let a : f64 = inputs[0].trim().parse::<f64>().unwrap();
    let b : f64 = inputs[1].trim().parse::<f64>().unwrap();
    let c : f64 = inputs[2].trim().parse::<f64>().unwrap();

    let delta : f64 = (b * b) - 4.0 * a * c;

    if (delta < 0.0) | ((2.0 * a) == 0.0) {
        println!("Impossivel calcular");
    } else {
        println!("R1 = {:.5}", (-b + delta.sqrt()) / (2.0 * a));
        println!("R2 = {:.5}", (-b - delta.sqrt()) / (2.0 * a));
    }
}
