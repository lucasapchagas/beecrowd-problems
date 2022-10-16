use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let input_notes : i32 = input.trim().parse().unwrap();
    println!("{}", input_notes);

    let mut remaining : i32; 

    let mut notes : i32 = input_notes / 100;
    remaining = input_notes % 100;

    println!("{} nota(s) de R$ 100,00", notes);

    notes = remaining / 50;
    remaining = remaining % 50;

    println!("{} nota(s) de R$ 50,00", notes);

    notes = remaining / 20;
    remaining = remaining % 20;

    println!("{} nota(s) de R$ 20,00", notes);

    notes = remaining / 10;
    remaining = remaining % 10;

    println!("{} nota(s) de R$ 10,00", notes);
    
    notes = remaining / 5;
    remaining = remaining % 5;

    println!("{} nota(s) de R$ 5,00", notes);

    notes = remaining / 2;
    remaining = remaining % 2;

    println!("{} nota(s) de R$ 2,00", notes);

    println!("{} nota(s) de R$ 1,00", remaining);
}