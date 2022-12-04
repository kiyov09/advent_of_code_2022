use std::collections::HashSet;

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
        let first_set: HashSet<char> = self.first_half.chars().collect();
        let second_set: HashSet<char> = self.second_half.chars().collect();

        *first_set.intersection(&second_set).next().unwrap()
    }
}

// Group of 3 elves
struct Group {
    a: Rucksack,
    b: Rucksack,
    c: Rucksack,
}

impl Group {
    fn badge_item(&self) -> char {
        let a_set: HashSet<char> = format!("{}{}", self.a.first_half, self.a.second_half)
            .chars()
            .collect();
        let b_set: HashSet<char> = format!("{}{}", self.b.first_half, self.b.second_half)
            .chars()
            .collect();
        let c_set: HashSet<char> = format!("{}{}", self.c.first_half, self.c.second_half)
            .chars()
            .collect();

        let a_and_b: Vec<&char> = a_set.intersection(&b_set).collect();

        for item in a_and_b {
            if b_set.intersection(&c_set).any(|x| x == item) {
                return *item;
            }
        }

        panic!("Input error");
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

pub fn task_2() {
    let input = get_input_content(INPUT_PATH);

    let sum = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| Group {
            a: Rucksack::new(chunk[0]),
            b: Rucksack::new(chunk[1]),
            c: Rucksack::new(chunk[2]),
        })
        .fold(0, |acc, group| {
            let badge = group.badge_item();
            acc + item_priority(badge)
        });

    println!("Sum of badges priorities: {}", sum);
}
