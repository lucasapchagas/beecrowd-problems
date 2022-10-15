use std::io;

fn main() {
    let mut a : f64 = 3.14159;
    
    let mut input; 
    {
        input = String::new();
        io::stdin().read_line(&mut input).expect("Reading error");    
    }

    let raio : f64 = input.trim().parse().unwrap();
    a = a * (raio * raio);

    println!("A={:.4}", a);
}