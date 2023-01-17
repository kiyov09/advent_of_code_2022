use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_18.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube(usize, usize, usize);

impl FromStr for Cube {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        Ok(Self(
            iter.next().unwrap().parse()?,
            iter.next().unwrap().parse()?,
            iter.next().unwrap().parse()?,
        ))
    }
}

struct Challenge {
    cubes: Vec<Cube>,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        Self {
            cubes: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    pub fn get_surface(&self) -> usize {
        let mut surface = self.cubes.len() * 6;

        let mut done = HashSet::new();

        for cube in self.cubes.iter() {
            self.cubes.iter().for_each(|c| {
                if done.contains(&(cube, c)) || done.contains(&(c, cube)) {
                    return;
                }

                let delta_x = cube.0.abs_diff(c.0);
                let delta_y = cube.1.abs_diff(c.1);
                let delta_z = cube.2.abs_diff(c.2);

                match (delta_x == 1, delta_y == 1, delta_z == 1) {
                    (true, false, false) => {
                        if delta_y == 0 && delta_z == 0 {
                            surface -= 2;
                        }
                    }
                    (false, false, true) => {
                        if delta_x == 0 && delta_y == 0 {
                            surface -= 2;
                        }
                    }
                    (false, true, false) => {
                        if delta_x == 0 && delta_z == 0 {
                            surface -= 2;
                        }
                    }
                    _ => (),
                };

                done.insert((c, cube));
            });
        }

        surface
    }
}

pub fn task_1() {
    let ch = Challenge::new();
    println!("Surface: {}", ch.get_surface());
}
pub fn task_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_test() {
        let ch = Challenge::new();
        assert_eq!(ch.get_surface(), 64);
    }
}
