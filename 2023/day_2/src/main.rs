use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum_of_ids = 0;

    for line in reader.lines() {
        let line = line?;
        sum_of_ids += check_if_games_is_possible(&line);
    }

    println!(" Total : {sum_of_ids}");
    
    Ok(())
}

fn check_if_games_is_possible(text: &str) -> i32 {
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;
    let mut game_id : i32 = 0;
    let mut text = text;

    if let Some(result) = text.find(":") {
        game_id = text[..result].split_whitespace().nth(1).unwrap().trim().parse().unwrap();
        let remove_text = format!("{}:", &text[..result]);  
        text = text.strip_prefix(&remove_text).unwrap();
    }

    let parts = text.split(";");

    for part in parts {
        let colors = part.split(",");

        for color in colors {
            if let Some(result) = color.find("red") {
                let red_color : i32 = color[..result].trim().parse().unwrap(); 
                if  red_color > max_red_cubes {
                    return 0
                }
            }

            if let Some(result) = color.find("green") {
                let green_color : i32 = color[..result].trim().parse().unwrap(); 
                if green_color > max_green_cubes {
                    println!("{}", color[..result].trim());
                    return 0
                }
            }

            if let Some(result) = color.find("blue") {
                let blue_color : i32 = color[..result].trim().parse().unwrap(); 
                if blue_color > max_blue_cubes {
                    println!("{}", color[..result].trim());
                    return 0
                }
            }
        }
    } 

    game_id 
}
