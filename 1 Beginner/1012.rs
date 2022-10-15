use std::io;

fn main() {
    let a : f64;
    let b : f64;
    let c : f64;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    let inputs = input.collect::<Vec<_>>();

    a = inputs[0].trim().parse::<f64>().unwrap();
    b = inputs[1].trim().parse::<f64>().unwrap();
    c = inputs[2].trim().parse::<f64>().unwrap();

    println!("TRIANGULO: {:.3}", (a * c) / 2.0f64);
    println!("CIRCULO: {:.3}", 3.14159 * c.powf(2.0));
    println!("TRAPEZIO: {:.3}", ((a + b) * c) / 2.0);
    println!("QUADRADO: {:.3}", b * b);
    println!("RETANGULO: {:.3}", a * b); 
}