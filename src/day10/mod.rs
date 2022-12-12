use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_10.txt";

#[derive(Debug)]
enum Inst {
    Noop,
    Addx(i32),
}

impl FromStr for Inst {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let _inst = split.next().expect("Should have a instruction");

        match split.next() {
            Some(amount) => Ok(Inst::Addx(amount.parse::<i32>()?)),
            None => Ok(Inst::Noop),
        }
    }
}

#[derive(Debug)]
struct Program {
    insts: VecDeque<Inst>,
    x_values: Vec<i32>,
}

impl Program {
    pub fn execute(&mut self) {
        let mut x = 1;
        let mut on_hold = None;

        loop {
            self.x_values.push(x);

            if on_hold.is_some() {
                let to_change = on_hold.unwrap();
                x += to_change;

                on_hold = None;
                continue;
            }

            let next_inst = self.insts.pop_front();
            match next_inst {
                Some(inst) => match inst {
                    Inst::Noop => continue,
                    Inst::Addx(temp) => on_hold = Some(temp),
                },
                None => break,
            }
        }
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            insts: s.lines().map(|l| l.parse::<Inst>().unwrap()).collect(),
            x_values: vec![],
        })
    }
}

pub fn task_1() {
    let input = get_input_content(INPUT_PATH);
    let mut p: Program = input.parse().unwrap();

    p.execute();

    let sum: i32 = p
        .x_values
        .iter()
        .enumerate()
        .fold(0, |mut acc, (idx, i)| match idx + 1 {
            factor @ (20 | 60 | 100 | 140 | 180 | 220) => {
                acc += factor as i32 * *i;
                acc
            }
            _ => acc,
        });

    println!("Sum of signal strengths: {}", sum);
}

pub fn task_2() {
    let input = get_input_content(INPUT_PATH);
    let mut p: Program = input.parse().unwrap();

    p.execute();

    let screen = p
        .x_values
        .chunks(40)
        .take(6)
        .map(|chunk| {
            let mut sprite_pos = 0..=2;

            chunk
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc, (idx, i)| {
                    if *i - 1 < 0 {
                        sprite_pos = 0..=2;
                    } else {
                        sprite_pos = (*i as usize) - 1..=(*i as usize) + 1;
                    }

                    if sprite_pos.contains(&idx) {
                        acc.push('#');
                    } else {
                        acc.push('.');
                    }

                    acc
                })
        })
        .fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc.push('\n');
            acc
        });

    println!("CTR display:\n{}", screen); // Uppercase letters are: FBURHZCH
}
