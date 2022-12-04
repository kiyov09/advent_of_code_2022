use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_3.txt";

// Represents each rucksack
struct Rucksack {
    first_half: String,
    second_half: String,
}

impl Rucksack {
    fn new(line: &str) -> Self {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        Self {
            first_half: first_half.to_string(),
            second_half: second_half.to_string(),
        }
    }

    fn item_in_both_halves(&self) -> char {
        for item in self.first_half.chars() {
            if self.second_half.contains(item) {
                return item;
            }
        }

        panic!("Error in the input data")
    }
}

fn item_priority(c: char) -> i32 {
    let as_number = c as i32;
    if as_number > 90 {
        // is lowercase letter
        as_number - 96 // a is 97 in ascii
    } else {
        // is uppercase letter
        as_number - 38 // A is 65 in ascii
    }
}

pub fn task_1() {
    let input = get_input_content(INPUT_PATH);

    let sum = input.lines().map(Rucksack::new).fold(0, |acc, item| {
        let in_both = item.item_in_both_halves();
        let priority = item_priority(in_both);

        acc + priority
    });

    println!("Sum of priorities: {}", sum);
}

pub fn task_2() {}
