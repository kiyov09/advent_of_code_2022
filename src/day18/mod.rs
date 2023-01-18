use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
    ops::Sub,
    str::FromStr,
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_18.txt";

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Cube(i32, i32, i32);

impl Cube {
    fn is_valid(&self) -> bool {
        return self.0 >= 0 && self.1 >= 0 && self.2 >= 0;
    }
}

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

    pub fn get_external_surface(&self) -> usize {
        let mut surface = 0;

        let min_x = self.cubes.iter().min_by_key(|c| c.0).unwrap().0 - 1;
        let min_y = self.cubes.iter().min_by_key(|c| c.1).unwrap().1 - 1;
        let min_z = self.cubes.iter().min_by_key(|c| c.2).unwrap().2 - 1;
        let max_x = self.cubes.iter().max_by_key(|c| c.0).unwrap().0 + 1;
        let max_y = self.cubes.iter().max_by_key(|c| c.1).unwrap().1 + 1;
        let max_z = self.cubes.iter().max_by_key(|c| c.2).unwrap().2 + 1;

        let mut air_spots = HashSet::new();

        // Calculate all air spots in the range
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if !self.cubes.contains(&Cube(x, y, z)) {
                        air_spots.insert(Cube(x, y, z));
                    }
                }
            }
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([Cube(min_x, min_y, min_z)]);

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

        while let Some(air) = queue.pop_front() {
            if visited.contains(&air) {
                continue;
            }

            deltas.iter().for_each(|delta| {
                let n = &air - delta;
                if self.cubes.contains(&n) {
                    surface += 1;
                }
                if air_spots.contains(&n) {
                    queue.push_back(n);
                }
            });

            visited.insert(air);
        }

        surface
    }
}

pub fn task_1() {
    let ch = Challenge::new();
    println!("Surface: {}", ch.get_surface());
}
pub fn task_2() {
    let ch = Challenge::new();
    println!("Surface water can reach: {}", ch.get_external_surface());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_test() {
        let ch = Challenge::new();
        assert_eq!(ch.get_surface(), 64);
    }
}
