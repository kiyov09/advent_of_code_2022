use std::{ops::RangeInclusive, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_4.txt";
const PAIR_DELIMITER: char = ',';

// Pairs of sections
struct Pair {
    section_1: RangeInclusive<u32>,
    section_2: RangeInclusive<u32>,
}

impl Pair {
    fn is_there_complete_overlapping(&self) -> bool {
        if self.section_1.start() <= self.section_2.start()
            && self.section_1.end() >= self.section_2.end()
        {
            return true;
        }

        if self.section_2.start() <= self.section_1.start()
            && self.section_2.end() >= self.section_1.end()
        {
            return true;
        }

        false
    }

    fn is_there_overlap(&self) -> bool {
        for item in *self.section_1.start()..=*self.section_1.end() {
            if self.section_2.contains(&item) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(PAIR_DELIMITER).unwrap();

        let mut first = first.split('-').map(|i| i.parse::<u32>().unwrap());
        let mut second = second.split('-').map(|i| i.parse::<u32>().unwrap());

        Ok(Self {
            section_1: RangeInclusive::new(first.next().unwrap(), first.next().unwrap()),
            section_2: RangeInclusive::new(second.next().unwrap(), second.next().unwrap()),
        })
    }
}

struct Challenge {
    pairs: Vec<Pair>,
}

impl Challenge {
    fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        Self {
            pairs: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn process_task_1(&self) {
        let overlaping_count =
            self.pairs
                .iter()
                .fold(0, |acc, pair| match pair.is_there_complete_overlapping() {
                    true => acc + 1,
                    false => acc,
                });

        println!(
            "Assigments that contains others fully: {}",
            overlaping_count
        );
    }

    fn process_task_2(&self) {
        let overlaping_count =
            self.pairs
                .iter()
                .by_ref()
                .fold(0, |acc, pair| match pair.is_there_overlap() {
                    true => acc + 1,
                    false => acc,
                });

        println!("Assigments that contains others: {}", overlaping_count);
    }
}

pub fn task_1() {
    Challenge::new().process_task_1();
}

pub fn task_2() {
    Challenge::new().process_task_2()
}
