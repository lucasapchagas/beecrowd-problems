use std::io;

fn main() {
    let mut v : f64 = 3.14159;
    
    let mut input;
    input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    v = (4.0/3.0) * v * input.trim().parse::<f64>().unwrap().powf(3.0);

    println!("VOLUME = {:.3}", v);
}