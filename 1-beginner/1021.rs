use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading error");

    let mut input_notes_float : f64 = input.trim().parse().unwrap();
    let input_notes : i32 = input_notes_float as i32;

    println!("NOTAS:");

    input_notes_float = input_notes_float - input_notes as f64;

    let mut notes : i32 = input_notes / 100;
    let mut remaining : i32 = input_notes % 100;

    println!("{} nota(s) de R$ 100.00", notes);

    notes = remaining / 50;
    remaining = remaining % 50;

    println!("{} nota(s) de R$ 50.00", notes);

    notes = remaining / 20;
    remaining = remaining % 20;

    println!("{} nota(s) de R$ 20.00", notes);

    notes = remaining / 10;
    remaining = remaining % 10;

    println!("{} nota(s) de R$ 10.00", notes);
    
    notes = remaining / 5;
    remaining = remaining % 5;

    println!("{} nota(s) de R$ 5.00", notes);

    notes = remaining / 2;
    remaining = remaining % 2;

    println!("{} nota(s) de R$ 2.00", notes);

    println!("MOEDAS:");

    println!("{} moeda(s) de R$ 1.00", remaining);

    let mut coins = input_notes_float * 100f64;
    let mut remaining_coins : f64;
    
    coins = coins / 50f64;
    remaining_coins = (input_notes_float * 100f64) % 50f64;
    
    println!("{} moeda(s) de R$ 0.50", coins as i32);

    coins = remaining_coins / 25f64;
    remaining_coins = remaining_coins % 25f64;

    println!("{} moeda(s) de R$ 0.25", coins as i32);

    coins = remaining_coins / 10f64;
    remaining_coins = remaining_coins % 10f64;

    println!("{} moeda(s) de R$ 0.10", coins as i32);

    coins = remaining_coins / 05f64;
    remaining_coins = remaining_coins % 05f64;

    println!("{} moeda(s) de R$ 0.05", coins as i32);

    println!("{} moeda(s) de R$ 0.01", remaining_coins as i32);
}