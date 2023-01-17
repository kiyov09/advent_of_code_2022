use std::{num::ParseIntError, ops::Sub, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_18.txt";

#[derive(Debug, PartialEq, Eq)]
struct Cube(i32, i32, i32);

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

impl Sub for Cube {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<'a, 'b> Sub<&'b Cube> for &'a Cube {
    type Output = Cube;

    fn sub(self, rhs: &'b Cube) -> Cube {
        Cube(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
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
        let mut surface = 0;

        let deltas = [
            Cube(-1, 0, 0),
            Cube(1, 0, 0),
            // Same x, same z
            Cube(0, -1, 0),
            Cube(0, 1, 0),
            // Same x, same y
            Cube(0, 0, -1),
            Cube(0, 0, 1),
        ];

        for cube in self.cubes.iter() {
            deltas.iter().for_each(|c| {
                if !self.cubes.contains(&(cube - c)) {
                    surface += 1;
                }
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
