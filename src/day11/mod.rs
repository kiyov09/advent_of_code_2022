use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_11.txt";

struct Monkey {
    no_items_inspected: u64,
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> (usize, u64)>,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect();

        let mut op = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .split_whitespace();

        let operator = op.nth(1).unwrap();
        let op: Box<dyn Fn(u64) -> u64> = match operator {
            "*" => {
                let n = op.next().unwrap().trim();
                match n {
                    "old" => Box::new(move |x: u64| x * x),
                    _ => {
                        let n = n.parse::<u64>().unwrap();
                        Box::new(move |x: u64| x * n)
                    }
                }
            }
            "+" => {
                let n = op.next().unwrap().trim().parse::<u64>().unwrap();
                Box::new(move |x: u64| x + n)
            }
            _ => panic!("Error"),
        };

        let no_to_test = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        let monkey_when_true = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        let monkey_when_false = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        let test = move |x: u64| {
            if x % no_to_test == 0 {
                (monkey_when_true as usize, x / no_to_test)
            } else {
                (monkey_when_false as usize, x / no_to_test)
            }
        };

        lines.next();

        Ok(Self {
            no_items_inspected: 0,
            items,
            op,
            test: Box::new(test),
        })
    }
}

struct Challenge {
    monkeys: Vec<Monkey>,
}

impl Challenge {
    pub fn execute(&mut self) {
        for _ in 0..20 {
            for i in 0..self.monkeys.len() {
                let mut map: Vec<Vec<u64>> = vec![vec![]; self.monkeys.len()];

                let monkey = &mut self.monkeys[i];
                monkey.no_items_inspected += monkey.items.len() as u64;

                monkey.items.iter().for_each(|item| {
                    let op_result = (monkey.op)(*item);
                    let to_monkey = (monkey.test)(op_result / 3);
                    map[to_monkey.0].push(op_result / 3)
                });
                monkey.items = VecDeque::new();

                for (idx, v) in map.iter().enumerate() {
                    v.iter()
                        .for_each(|item| self.monkeys[idx].items.push_back(*item))
                }
            }
        }

        self.monkeys
            .sort_by(|a, b| a.no_items_inspected.cmp(&b.no_items_inspected));

        let monkey_business = self
            .monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, monkey| acc * monkey.no_items_inspected);

        println!("Monkey business: {}", monkey_business);
    }
}

impl FromStr for Challenge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .lines()
            .collect::<Vec<&str>>()
            .chunks(7)
            .map(|monkey_desc| {
                monkey_desc[1..monkey_desc.len()]
                    .iter()
                    .fold(String::new(), |mut acc, line| {
                        acc.push_str(line);
                        acc.push('\n');
                        acc
                    })
                    .parse::<Monkey>()
                    .unwrap()
            })
            .collect();

        Ok(Self { monkeys })
    }
}

pub fn task_1() {
    let input = get_input_content(INPUT_PATH);
    let mut ch = input.parse::<Challenge>().unwrap();
    ch.execute();
}
pub fn task_2() {}
