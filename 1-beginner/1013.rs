use std::io;

fn main() {
    let a : i32;
    let b : i32;
    let c : i32;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");
    let input = input.split(" ");
    let inputs = input.collect::<Vec<_>>();

    a = inputs[0].trim().parse::<i32>().unwrap();
    b = inputs[1].trim().parse::<i32>().unwrap();
    c = inputs[2].trim().parse::<i32>().unwrap();

    println!("{} eh o maior", maior_ab(a, maior_ab(b, c)));
}

fn maior_ab(a: i32, b: i32) -> i32 {
    return (a + b + (a - b).abs()) / 2;
}