use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_1.txt";

pub fn task_1() {
    let mut most_calories = 0;
    let mut current_count = 0;

    let file_contents = get_input_content(INPUT_PATH);

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

pub fn task_2() {
    let mut calories: Vec<i32> = Vec::new();
    let file_contents = get_input_content(INPUT_PATH);

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
