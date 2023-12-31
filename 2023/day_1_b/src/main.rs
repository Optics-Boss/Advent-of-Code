use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut new_line = line.clone();
        let mut word = String::new();

        line.chars().for_each(|char| {
          word.push(char);
          let result = turn_words_to_integers(&word);
          
          new_line = new_line.replace(result.0, result.1);
          word = word.replace(result.0, result.1);

          println!("{}", word)  
        });

        println!("{}", new_line);
            
        let digits: String = new_line.chars().filter(|char| char.is_digit(10)).collect();
        let first_last_digit = format!("{}{}", 
            digits.chars().nth(0).unwrap(),
            digits.chars().last().unwrap()
        );

        println!("{}", first_last_digit);

        let first_last_digit: i32 = first_last_digit.parse().unwrap();
        total += first_last_digit;
    }

    println!("Total: {total}");

    Ok(())
}

pub fn turn_words_to_integers(text: &str) -> (&str, &str){
        if text.contains("nine") {
            return ("nine", "9")
        } else if text.contains("eight") {
            return ("eight", "8")
        } else if text.contains("seven") {
            return ("seven", "7")
        } else if text.contains("six") {
            return ("six", "6")
        } else if text.contains("five") {
            return ("five", "5")
        } else if text.contains("four") {
            return ("four", "4")
        } else if text.contains("three") {
            return ("three", "3")
        } else if text.contains("two") {
            return ("two", "2")
        } else if text.contains("one") {
            return ("one", "1")
        } 

        ("", "")
}

