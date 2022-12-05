use std::{collections::VecDeque, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_5.txt";

#[derive(Debug)]
struct StacksManager {
    stacks: Vec<VecDeque<char>>,
}

impl StacksManager {
    fn new(len: usize) -> Self {
        Self {
            stacks: vec![VecDeque::new(); len],
        }
    }

    fn insert_into_stacks(&mut self, v: &[char]) {
        v.iter().enumerate().for_each(|(index, item)| {
            if *item != ' ' {
                self.stacks[index].push_front(*item)
            }
        })
    }

    fn process(&mut self, command: &Command) {
        let Command { count, from, to } = command;

        let from = &mut self.stacks[((*from as u32) - 1) as usize];
        let mut temp = vec![];

        for _ in 0..*count {
            if let Some(item_to_move) = from.pop_back() {
                temp.push(item_to_move);
            }
        }

        let to = &mut self.stacks[((*to as u32) - 1) as usize];
        for item in temp {
            to.push_back(item);
        }
    }

    fn tops(&self) {
        let tops =
            self.stacks
                .iter()
                .fold(String::with_capacity(self.stacks.len()), |mut acc, item| {
                    if let Some(c) = item.back() {
                        acc.push(*c);
                    }
                    acc
                });

        println!("Tops of the stacks: {}", tops);
    }
}

#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s
            .split_whitespace()
            .filter_map(|item| item.parse::<usize>().ok());

        Ok(Self {
            count: pieces.next().unwrap(),
            from: pieces.next().unwrap(),
            to: pieces.next().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Challenge {
    manager: StacksManager,
    commands: Vec<Command>,
}

impl Challenge {
    fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        let mut input_lines = input.lines();

        let first_line = input_lines.next().unwrap();
        let mut processed = Challenge::process_line(first_line);

        let mut manager = StacksManager::new(processed.len());
        manager.insert_into_stacks(&processed);

        loop {
            let line = input_lines.next().unwrap();
            if line.starts_with(" 1") {
                break;
            }

            processed = Challenge::process_line(line);
            manager.insert_into_stacks(&processed);
        }

        // Skip the empty line
        input_lines.next();

        let mut commands: Vec<Command> = vec![];
        for line in input_lines {
            commands.push(line.parse::<Command>().unwrap());
        }

        Self { manager, commands }
    }

    fn process_line(s: &str) -> Vec<char> {
        s.replace('[', "")
            .replace(']', "")
            .replace("  ", " ")
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk[0])
            .collect()
    }

    fn task_1(&mut self) {
        for command in &self.commands {
            self.manager.process(command);
        }
        self.manager.tops();
    }
}

pub fn task_1() {
    Challenge::new().task_1();
}
pub fn task_2() {}
