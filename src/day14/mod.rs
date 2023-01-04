use std::{collections::HashSet, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_14.txt";

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Cell(u32, u32);

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
                            let cell_a = cells_pair[0].parse::<Cell>().unwrap();
                            let cell_b = cells_pair[1].parse::<Cell>().unwrap();

                            if cell_a.0 == cell_b.0 {
                                // MOVE in the Y
                                if cell_a.1 < cell_b.1 {
                                    for new_y in cell_a.1 + 1..cell_b.1 {
                                        acc.insert(Cell(cell_a.0, new_y));
                                    }
                                } else {
                                    for new_y in cell_b.1 + 1..cell_a.1 {
                                        acc.insert(Cell(cell_a.0, new_y));
                                    }
                                }
                            }

                            if cell_a.1 == cell_b.1 {
                                // MOVE in the X
                                if cell_a.0 < cell_b.0 {
                                    for new_x in cell_a.0 + 1..cell_b.0 {
                                        acc.insert(Cell(new_x, cell_a.1));
                                    }
                                } else {
                                    for new_x in cell_b.0 + 1..cell_a.0 {
                                        acc.insert(Cell(new_x, cell_a.1));
                                    }
                                }
                            }

                            acc.insert(cell_a);
                            acc.insert(cell_b);

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
        let mut test_cell = *cell;

        // Test down
        test_cell.1 += 1;

        if self.cells.get(&test_cell).is_none() {
            // println!("Going down: {:?}", test_cell);
            return Some(test_cell);
        }

        // Test left
        test_cell.0 -= 1;

        if self.cells.get(&test_cell).is_none() {
            // println!("Going left: {:?}", test_cell);
            return Some(test_cell);
        }

        // Test right
        test_cell.0 += 2;

        if self.cells.get(&test_cell).is_none() {
            // println!("Going right: {:?}", test_cell);
            return Some(test_cell);
        }

        // println!("Staying: {:?}", cell);
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
