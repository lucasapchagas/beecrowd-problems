use std::io;

fn main() {
    let inputs;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    inputs = input.collect::<Vec<_>>();

    let a : i32 = inputs[0].trim().parse::<i32>().unwrap();
    let b : i32 = inputs[1].trim().parse::<i32>().unwrap();
    let c : i32 = inputs[2].trim().parse::<i32>().unwrap();
    let d : i32 = inputs[3].trim().parse::<i32>().unwrap();

    if (b > c) & (d > a) & ((c + d) > (a + b)) & (c > 0) & (d > 0) & (a % 2 == 0){
        println!("Valores aceitos");
    } else {
        println!("Valores nao aceitos");
    }
}
