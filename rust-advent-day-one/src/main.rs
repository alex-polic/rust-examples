use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("calories.txt").expect("Could not open file");
    let buffered_reader = BufReader::new(file);
    let mut bunny_count = 0;
    let mut max_calories = -1;
    let mut max_calories_index = -1;
    let mut current_calories_count = 0;
    
    for line in buffered_reader.lines() {
        let line_text = line.expect("Cannot read current line");
        if line_text.eq("") {
            if current_calories_count > max_calories {
                max_calories = current_calories_count;
                max_calories_index = bunny_count;
            }
            println!("{},{}, {}", current_calories_count, max_calories, max_calories_index);

            current_calories_count = 0;
            bunny_count += 1;
            continue;
        }
        current_calories_count += line_text.parse::<i32>().unwrap();
    }

    if current_calories_count > max_calories {
        max_calories = current_calories_count;
        max_calories_index = bunny_count;
    }
    println!("{}", max_calories_index);
}
