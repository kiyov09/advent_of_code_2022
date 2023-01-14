use std::fmt::Display;

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_17.txt";

struct Jets {
    pattern: Vec<char>,
    index: usize,
}

impl Jets {
    pub fn new(p: String) -> Self {
        Self {
            pattern: p.chars().take_while(|c| c == &'>' || c == &'<').collect(),
            index: 0,
        }
    }

    pub fn get_next(&mut self) -> &char {
        if self.index >= self.pattern.len() {
            self.index = 0;
        }

        let result = self.pattern.get(self.index).unwrap();
        self.index += 1;

        result
    }
}

type Shape = Vec<Vec<bool>>;

#[derive(Clone)]
struct Rock {
    width: usize,
    height: usize,
    shape: Shape,
}

impl Rock {
    pub fn new(shape: Shape) -> Self {
        let width = shape.get(0).unwrap_or(&Vec::new()).len();
        let height = shape.len();

        Self {
            shape,
            width,
            height,
        }
    }

    pub fn all_pos(&self) -> Vec<(usize, usize)> {
        self.shape
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, v)| match v {
                    true => Some((x, y)),
                    false => None,
                })
            })
            .collect()
    }

    pub fn down_limits(&self) -> Vec<(usize, usize)> {
        let mut poss = vec![];
        for x in 0..self.width {
            let y = self.height - 1;
            if self.shape[y][x] {
                poss.push((x, self.height - y - 1));
            } else {
                poss.push((x, self.height - y));
            }
        }
        poss
    }

    pub fn right_limits(&self) -> Vec<(usize, usize)> {
        self.shape
            .iter()
            .rev()
            .enumerate()
            .map(|(y, line)| {
                let last_x = line.iter().rposition(|v| *v).unwrap();
                (last_x, y)
            })
            .collect()
    }

    pub fn left_limits(&self) -> Vec<(usize, usize)> {
        self.shape
            .iter()
            .rev()
            .enumerate()
            .map(|(y, line)| {
                let x = line.iter().position(|v| *v).unwrap();
                (x, y)
            })
            .collect()
    }
}

impl From<RockType> for Rock {
    fn from(value: RockType) -> Self {
        let shape = match value {
            RockType::LineH => vec![vec![true; 4]],
            RockType::Plus => {
                vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ]
            }
            RockType::InvertedL => vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![true, true, true],
            ],
            RockType::LineV => vec![vec![true], vec![true], vec![true], vec![true]],
            RockType::Square => vec![vec![true; 2], vec![true; 2]],
        };

        Self::new(shape)
    }
}

impl From<Rock> for Shape {
    fn from(value: Rock) -> Self {
        value.shape
    }
}

impl From<&Rock> for Shape {
    fn from(value: &Rock) -> Self {
        value.clone().shape
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.shape.iter().rev().fold(String::new(), |mut acc, v| {
                acc.push_str(
                    &v.iter()
                        .map(|v| if *v { '#' } else { '.' })
                        .collect::<String>(),
                );
                acc.push('\n');
                acc
            })
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RockType {
    LineH,
    Plus,
    InvertedL,
    LineV,
    Square,
}

#[derive(Clone, Copy)]
struct RockGen {
    last_one: RockType,
}

impl Iterator for RockGen {
    type Item = RockType;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_one = match self.last_one {
            RockType::LineH => RockType::Plus,
            RockType::Plus => RockType::InvertedL,
            RockType::InvertedL => RockType::LineV,
            RockType::LineV => RockType::Square,
            RockType::Square => RockType::LineH,
        };
        Some(self.last_one)
    }
}

struct Challenge {
    jets: Jets,
    chamber: Shape,
    rock_gen: RockGen,
}

impl Challenge {
    pub fn new(rg: Option<RockGen>) -> Self {
        let input = get_input_content(INPUT_PATH);

        Self {
            jets: Jets::new(input),
            chamber: Vec::new(),
            rock_gen: rg.unwrap_or(RockGen {
                last_one: RockType::Square,
            }),
        }
    }

    pub fn add_rock(&mut self, rock: &Rock) {
        let empty_line = vec![false; 7];
        (0..3).for_each(|_| self.chamber.push(empty_line.clone()));

        let mut new_rock_shape: Shape = rock.into();
        new_rock_shape.iter_mut().for_each(|line| {
            line.insert(0, false);
            line.insert(0, false);

            let missing = 7 - line.len();
            for _ in 0..missing {
                line.push(false);
            }
        });
        new_rock_shape.reverse();

        self.chamber.extend_from_slice(&new_rock_shape);
    }

    pub fn simulate(&mut self, rounds: usize) {
        for (rock_count, new_rock_type) in self.rock_gen.enumerate() {
            if rock_count == rounds {
                break;
            }

            let falling_rock = Rock::from(new_rock_type);
            self.add_rock(&falling_rock);

            // Left bottom point
            let mut falling_rock_pos = (2, self.chamber.len() - falling_rock.height);

            loop {
                // Handle next jet
                match self.jets.get_next() {
                    '>' => {
                        if self.move_falling_right(&falling_rock, falling_rock_pos) {
                            falling_rock_pos.0 += 1;
                        }
                    }
                    '<' => {
                        if self.move_falling_left(&falling_rock, falling_rock_pos) {
                            falling_rock_pos.0 -= 1;
                        }
                    }
                    _ => unreachable!("This jet is not possible"),
                }

                if !self.move_falling_down(&falling_rock, falling_rock_pos) {
                    break;
                }

                falling_rock_pos.1 -= 1;
            }
        }
    }

    fn move_falling_down(&mut self, falling: &Rock, pos: (usize, usize)) -> bool {
        if pos.1 == 0 {
            return false;
        }

        let poss = falling
            .down_limits()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in poss {
            if self.chamber[p.1][p.0] && self.chamber[p.1 - 1][p.0] {
                return false;
            }
        }

        let all_pos = falling
            .all_pos()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in all_pos.iter() {
            self.chamber[p.1 - 1][p.0] = self.chamber[p.1][p.0];
            self.chamber[p.1][p.0] = false;
        }

        if let Some(last) = self.chamber.last() {
            if last.iter().all(|v| v == &false) {
                self.chamber.pop();
            }
        }

        true
    }

    fn move_falling_right(&mut self, falling: &Rock, pos: (usize, usize)) -> bool {
        if pos.0 + falling.width >= self.chamber[0].len() {
            return false;
        }

        let poss = falling
            .right_limits()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in poss {
            if self.chamber[p.1][p.0] && self.chamber[p.1][p.0 + 1] {
                return false;
            }
        }

        let all_pos = falling
            .all_pos()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in all_pos.iter().rev() {
            self.chamber[p.1][p.0 + 1] = self.chamber[p.1][p.0];
            self.chamber[p.1][p.0] = false;
        }

        true
    }

    fn move_falling_left(&mut self, falling: &Rock, pos: (usize, usize)) -> bool {
        if pos.0 == 0 {
            return false;
        }

        let poss = falling
            .left_limits()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in poss {
            if self.chamber[p.1][p.0] && self.chamber[p.1][p.0 - 1] {
                return false;
            }
        }

        let all_pos = falling
            .all_pos()
            .iter()
            .map(|p| (p.0 + pos.0, p.1 + pos.1))
            .collect::<Vec<_>>();

        for p in all_pos.iter() {
            self.chamber[p.1][p.0 - 1] = self.chamber[p.1][p.0];
            self.chamber[p.1][p.0] = false;
        }

        true
    }

    fn print_chamber(&self) -> String {
        self.chamber.iter().rev().fold(String::new(), |mut acc, v| {
            acc.push_str(
                &v.iter()
                    .map(|v| if *v { '#' } else { '.' })
                    .collect::<String>(),
            );
            acc.push('\n');
            acc
        })
    }
}

pub fn task_1() {
    let mut ch = Challenge::new(None);
    ch.simulate(2022);
    println!("Chamber height: {}", ch.chamber.len());
}
pub fn task_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Jets tests
    #[test]
    fn jets_get_next() {
        let mut ch = Challenge::new(None);
        assert_eq!(
            (0..80).fold(String::new(), |mut acc, _| {
                acc.push(*ch.jets.get_next());
                acc
            }),
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
                .to_string()
        )
    }

    // Rock tests
    #[test]
    fn line_rock_creation_test() {
        let rock_shape = vec![vec![true, true, true, true]];
        let rock_shape_clone = rock_shape.clone();

        let new_rock = Rock::new(rock_shape);

        assert_eq!(new_rock.shape, rock_shape_clone);
        assert_eq!(new_rock.height, 1);
        assert_eq!(new_rock.width, 4);
    }

    #[test]
    fn plus_rock_creation_test() {
        let rock_shape = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let rock_shape_clone = rock_shape.clone();

        let new_rock = Rock::new(rock_shape);

        assert_eq!(new_rock.shape, rock_shape_clone);
        assert_eq!(new_rock.height, 3);
        assert_eq!(new_rock.width, 3);
    }

    #[test]
    fn from_rocktype_test() {
        let line_v = RockType::LineV;
        let rock = Rock::from(line_v);

        let rock_v_shape = vec![vec![true], vec![true], vec![true], vec![true]];

        assert_eq!(rock.shape, rock_v_shape);
    }

    #[test]
    fn rock_display_test() {
        let sq = RockType::Square;
        let rock = Rock::from(sq);

        assert_eq!(rock.to_string(), String::from("##\n##\n"));
    }

    #[test]
    fn rock_into_vec_test() {
        let rock = Rock::from(RockType::LineH);
        let vec: Vec<_> = rock.into();

        assert_eq!(vec, vec![vec![true; 4]]);
    }

    #[test]
    fn down_limits_test() {
        let rock = Rock::from(RockType::LineH);
        let poss = rock.down_limits();
        assert_eq!(poss, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);

        let rock = Rock::from(RockType::Plus);
        let poss = rock.down_limits();
        assert_eq!(poss, vec![(0, 1), (1, 0), (2, 1)]);

        let rock = Rock::from(RockType::InvertedL);
        let poss = rock.down_limits();
        assert_eq!(poss, vec![(0, 0), (1, 0), (2, 0)]);

        let rock = Rock::from(RockType::LineV);
        let poss = rock.down_limits();
        assert_eq!(poss, vec![(0, 0)]);

        let rock = Rock::from(RockType::Square);
        let poss = rock.down_limits();
        assert_eq!(poss, vec![(0, 0), (1, 0)]);
    }

    #[test]
    fn right_limits_test() {
        let rock = Rock::from(RockType::LineH);
        let poss = rock.right_limits();
        assert_eq!(poss, vec![(3, 0)]);

        let rock = Rock::from(RockType::Plus);
        let poss = rock.right_limits();
        assert_eq!(poss, vec![(1, 0), (2, 1), (1, 2)]);

        let rock = Rock::from(RockType::InvertedL);
        let poss = rock.right_limits();
        assert_eq!(poss, vec![(2, 0), (2, 1), (2, 2)]);

        let rock = Rock::from(RockType::LineV);
        let poss = rock.right_limits();
        assert_eq!(poss, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);

        let rock = Rock::from(RockType::Square);
        let poss = rock.right_limits();
        assert_eq!(poss, vec![(1, 0), (1, 1)]);
    }

    #[test]
    fn left_limits_test() {
        let rock = Rock::from(RockType::LineH);
        let poss = rock.left_limits();
        assert_eq!(poss, vec![(0, 0)]);

        let rock = Rock::from(RockType::Plus);
        let poss = rock.left_limits();
        assert_eq!(poss, vec![(1, 0), (0, 1), (1, 2)]);

        let rock = Rock::from(RockType::InvertedL);
        let poss = rock.left_limits();
        assert_eq!(poss, vec![(0, 0), (2, 1), (2, 2)]);

        let rock = Rock::from(RockType::LineV);
        let poss = rock.left_limits();
        assert_eq!(poss, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);

        let rock = Rock::from(RockType::Square);
        let poss = rock.left_limits();
        assert_eq!(poss, vec![(0, 0), (0, 1)]);
    }

    #[test]
    fn all_pos_test() {
        let rock = Rock::from(RockType::LineH);
        let poss = rock.all_pos();
        assert_eq!(poss, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);

        let rock = Rock::from(RockType::Plus);
        let poss = rock.all_pos();
        assert_eq!(poss, vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]);
    }

    // RockType test
    #[test]
    fn rock_gen_iterator_test() {
        let mut rg = RockGen {
            last_one: RockType::Square,
        };

        let types_yield = (0..3).map(|_| rg.next().unwrap()).collect::<Vec<_>>();

        assert_eq!(
            types_yield,
            vec![RockType::LineH, RockType::Plus, RockType::InvertedL]
        );
    }

    // Challenge tests
    #[test]
    fn add_line_rock_test() {
        let mut ch = Challenge::new(None);
        let rock = Rock::from(RockType::LineH);

        ch.add_rock(&rock);

        assert_eq!(ch.print_chamber(), "..####.\n.......\n.......\n.......\n");
    }

    #[test]
    fn add_plus_rock_test() {
        let mut ch = Challenge::new(None);
        let rock = Rock::from(RockType::Plus);

        ch.add_rock(&rock);

        assert_eq!(
            ch.print_chamber(),
            "...#...\n..###..\n...#...\n.......\n.......\n.......\n"
        );
    }
}
