use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total: i32 = 0;
    
    for line in reader.lines() {
        let digits: String = line?.chars().filter(|char| char.is_digit(10)).collect();
        let first_last_digit = format!("{}{}", digits.chars().nth(0).unwrap(), digits.chars().last().unwrap());
        let first_last_digit: i32 = first_last_digit.parse().unwrap();

        total += first_last_digit;
    }

    println!("Total: {total}");

    Ok(())
}
