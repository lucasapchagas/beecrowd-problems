use std::io;

fn main() {
    const RADIX: u32 = 10;

    let mut input = String::new();

    while io::stdin().read_line(&mut input).is_ok() {
        if input.trim().is_empty() {
            break;
        }

        let line = input.split(" ");
        let inputs = line.collect::<Vec<_>>();

        let a : u32 = inputs[0].trim().parse::<u32>().unwrap();
        let b : u32 = inputs[1].trim().parse::<u32>().unwrap();

        let a_string = format!("{:032b}", a);
        let b_string = format!("{:032b}", b);

        let mut result_a = String::new();

        for (i, c) in a_string.chars().enumerate() {
            let bit_a = c.to_digit(RADIX).unwrap() != 0;
            let bit_b = b_string.chars().nth(i).unwrap().to_digit(RADIX).unwrap() != 0;

            if bit_a ^ bit_b {
                result_a.push('1');
                continue;
            }

            result_a.push('0');
        }

        println!("{}", u32::from_str_radix(&result_a, 2).unwrap());
        input.clear();
    }
}