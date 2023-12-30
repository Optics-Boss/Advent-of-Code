fn main() {
    let mut sum_of_ids = 0;

    sum_of_ids += check_if_games_is_possible("Game 1: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");

    println!("{sum_of_ids}")

}

fn check_if_games_is_possible(text: &str) -> i32 {
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 13;

    let mut current_red_cubes = 0;
    let mut current_green_cubes = 0;
    let mut current_blue_cubes = 0;
    let mut game_id : i32 = 0;
    let mut text = text;

    if let Some(result) = text.find(":") {
        game_id = text[..result].split_whitespace().nth(1).unwrap().trim().parse().unwrap();
        let remove_text = format!("{}:", &text[..result]);  
        text = text.strip_prefix(&remove_text).unwrap();
    }

    println!("{}", text);

    let parts = text.split(";");

    for part in parts {
        let colors = part.split(",");

        for color in colors {
            if let Some(result) = color.find("red") {
                current_red_cubes += &color[..result].trim().parse().unwrap();
            }

            if let Some(result) = color.find("green") {
                current_green_cubes += &color[..result].trim().parse().unwrap();
            }

            if let Some(result) = color.find("blue") {
                current_blue_cubes += &color[..result].trim().parse().unwrap();
            }
        }
    } 


    println!("{current_red_cubes}");
    println!("{current_green_cubes}");
    println!("{current_blue_cubes}");

    if current_red_cubes > max_red_cubes 
        || current_green_cubes > max_green_cubes 
        || current_blue_cubes > max_blue_cubes {
        println!("Not possible");
        return 0
    } else {
        println!("Possible");
    }

    game_id 
}
