use std::{fs::File, io::Read};

fn main() {
    // day_one_task_one();
    day_one_task_two();
}

fn get_input_content(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}

fn day_one_task_one() {
    let mut most_calories = 0;
    let mut current_count = 0;

    let file_contents = get_input_content("./day_one_task_one_input.txt");

    file_contents.lines().for_each(|line| {
        if line.is_empty() {
            if current_count > most_calories {
                most_calories = current_count;
            }
            current_count = 0;
        } else {
            let calories = line.parse::<i32>().unwrap();
            current_count += calories;
        }
    });

    println!("Most calories: {}", most_calories);
}

fn day_one_task_two() {
    let mut calories: Vec<i32> = Vec::new();
    let file_contents = get_input_content("./day_one_task_one_input.txt");

    file_contents.lines().fold(0, |acc, line| {
        if line.is_empty() {
            calories.push(acc);
            0
        } else {
            let calories = line.parse::<i32>().unwrap();
            acc + calories
        }
    });

    calories.sort();
    let result: i32 = calories.iter().rev().take(3).sum();

    println!("Calories: {}", result);
}
