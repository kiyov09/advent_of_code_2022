use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_9.txt";

enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('R') => Ok(Self::Right),
            Some('L') => Ok(Self::Left),
            Some('U') => Ok(Self::Up),
            Some('D') => Ok(Self::Down),
            _ => panic!("Input error"),
        }
    }
}

struct Movement(Dir, u8);

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_at(1);
        Ok(Self(dir.parse().unwrap(), amount.trim().parse()?))
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position(i32, i32);

impl Position {
    pub fn do_move(&mut self, m: &Movement) {
        match m.0 {
            Dir::Right => self.0 += 1,
            Dir::Left => self.0 -= 1,
            Dir::Up => self.1 += 1,
            Dir::Down => self.1 -= 1,
        }
    }
    pub fn follow(&mut self, other: &Position) {
        // same position
        if self.0 == other.0 && self.1 == other.1 {
            return;
        }

        // other moved to right
        if self.1 == other.1 && other.0 - self.0 > 1 {
            self.0 += 1;
            return;
        }

        // other moved to left
        if self.1 == other.1 && self.0 - other.0 > 1 {
            self.0 -= 1;
            return;
        }

        if self.1.abs_diff(other.1) > 1 {
            // self is below
            if self.1 < other.1 {
                match self.0.cmp(&other.0) {
                    std::cmp::Ordering::Less => {
                        self.0 += 1;
                        self.1 += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        self.1 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        self.0 -= 1;
                        self.1 += 1;
                    }
                }
            } else {
                // self is above
                match self.0.cmp(&other.0) {
                    std::cmp::Ordering::Less => {
                        self.0 += 1;
                        self.1 -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        self.1 -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        self.0 -= 1;
                        self.1 -= 1;
                    }
                }
            }
        }

        if self.1.abs_diff(other.1) == 1 {
            if self.0 == other.0 || self.0.abs_diff(other.0) == 1 {
                return;
            }

            if self.1 < other.1 {
                if self.0 < other.0 {
                    self.0 += 1;
                    self.1 += 1;
                } else {
                    self.0 -= 1;
                    self.1 += 1;
                }
                return;
            }

            if self.1 > other.1 {
                if self.0 < other.0 {
                    self.0 += 1;
                    self.1 -= 1;
                } else {
                    self.0 -= 1;
                    self.1 -= 1;
                }
            }
        }
    }
}

struct Challenge {
    steps: String,
    tail_unique_positions: HashSet<Position>,
}

impl Challenge {
    pub fn new() -> Self {
        Self {
            steps: get_input_content(INPUT_PATH),
            tail_unique_positions: HashSet::default(),
        }
    }

    pub fn execute(&mut self) {
        self.execute_generic(2);
    }

    pub fn execute_10_knots(&mut self) {
        self.execute_generic(10);
    }

    pub fn execute_generic(&mut self, knots_count: usize) {
        let mut knots = vec![Position(0, 0); knots_count];
        self.tail_unique_positions.insert(Position(0, 0));

        self.steps
            .lines()
            .map(|s| s.parse::<Movement>().unwrap())
            .for_each(|m| {
                (0..m.1).for_each(|_| {
                    for index in 0..knots.len() {
                        match index {
                            0 => {
                                knots[0].do_move(&m);
                            }
                            _ => {
                                let prev = knots[index - 1].clone();
                                knots[index].follow(&prev);
                            }
                        }
                    }

                    self.tail_unique_positions
                        .insert(knots.last().unwrap().clone());
                });
            });

        println!(
            "Unique positions with {knots_count} knots: {}",
            self.tail_unique_positions.len()
        );
    }
}

pub fn task_1() {
    Challenge::new().execute();
}
pub fn task_2() {
    Challenge::new().execute_10_knots();
}
