use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let km : i32 = input.trim().parse().unwrap();
    println!("{} minutos", km * 2);
}
