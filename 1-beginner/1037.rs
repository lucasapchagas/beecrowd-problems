use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let value : f64 = input.trim().parse::<f64>().unwrap();

    if (value >= 0.0) & (value <= 25.0) {
        println!("Intervalo [0,25]");
    } else if (value > 25.0) & (value <= 50.0) {
        println!("Intervalo (25,50]");
    } else if (value > 50.0) & (value <= 75.0) {
        println!("Intervalo (50,75]");
    } else if (value > 75.0) & (value <= 100.0) {
        println!("Intervalo (75,100]");
    } else {
        println!("Fora de intervalo");
    }
}
