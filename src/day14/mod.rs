use std::{collections::HashSet, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_14.txt";

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Cell(i32, i32);

impl Default for Cell {
    fn default() -> Self {
        Self(500, Default::default())
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Self(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ))
    }
}

#[derive(Debug)]
struct Cave {
    cells: HashSet<Cell>,
}

impl Cave {
    pub fn insert_rock_line(cells: &mut HashSet<Cell>, start: &Cell, end: &Cell) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        let mut new_cell = Cell(start.0 + dx, start.1 + dy);

        while new_cell != *end {
            cells.insert(new_cell);

            new_cell.0 += dx;
            new_cell.1 += dy;
        }
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cells: s
                .lines()
                .map(|line| {
                    line.split(" -> ").collect::<Vec<_>>().windows(2).fold(
                        HashSet::new(),
                        |mut acc, cells_pair| {
                            let start = cells_pair[0].parse::<Cell>().unwrap();
                            let end = cells_pair[1].parse::<Cell>().unwrap();

                            Cave::insert_rock_line(&mut acc, &start, &end);

                            acc.insert(start);
                            acc.insert(end);

                            acc
                        },
                    )
                })
                .fold(HashSet::from([Cell(500, 0)]), |mut acc, set| {
                    acc.extend(set);
                    acc
                }),
        })
    }
}

impl Cave {
    pub fn process(&mut self) {
        let max_y = self.cells.iter().max_by_key(|c| c.1).unwrap().1;

        loop {
            let mut falling_cell = Cell::default();

            loop {
                if falling_cell.1 > max_y {
                    return;
                }

                match self.new_cell_pos(&falling_cell) {
                    Some(new_pos) => {
                        falling_cell = new_pos;
                        continue;
                    }
                    None => {
                        // println!("Inserting: {:?}", new_cell);
                        self.cells.insert(falling_cell);
                        break;
                    }
                }
            }
        }
    }

    pub fn process_with_floor(&mut self) {
        let max_y = self.cells.iter().max_by_key(|c| c.1).unwrap().1 + 2;

        loop {
            let mut falling_cell = Cell::default();

            loop {
                if falling_cell.1 == max_y - 1 {
                    self.cells.insert(falling_cell);
                    break;
                }

                match self.new_cell_pos(&falling_cell) {
                    Some(new_pos) => {
                        falling_cell = new_pos;
                        continue;
                    }
                    None => {
                        // println!("Inserting: {:?}", new_cell);
                        self.cells.insert(falling_cell);
                        if falling_cell == Cell(500, 0) {
                            return;
                        }
                        break;
                    }
                }
            }
        }
    }

    fn new_cell_pos(&self, cell: &Cell) -> Option<Cell> {
        for dx in [0, -1, 1] {
            let test_cell = Cell(cell.0 + dx, cell.1 + 1);

            if self.cells.get(&test_cell).is_none() {
                return Some(test_cell);
            }
        }

        None
    }
}

struct Challenge {
    cave: Cave,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);

        Self {
            cave: input.parse().unwrap(),
        }
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();

    let rocks_count = ch.cave.cells.len() - 1;
    ch.cave.process();

    let sands = ch.cave.cells.len() - rocks_count - 1;
    println!("Sand count: {}", sands);
}

pub fn task_2() {
    let mut ch = Challenge::new();

    let rocks_count = ch.cave.cells.len();
    ch.cave.process_with_floor();

    let sands = ch.cave.cells.len() - rocks_count + 1;
    println!("Sand count with floor: {}", sands);
}
