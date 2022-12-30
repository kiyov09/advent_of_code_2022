use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_12.txt";

#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Copy)]
struct Point(usize, usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
struct Pos {
    point: Point,
    value: u8,
}

struct Map {
    grid: Vec<Vec<Pos>>,
    start: Pos,
    end: Pos,
}

impl Map {
    pub fn find_path(&self, initial: &Pos) -> Option<u32> {
        // This map will hold the min dist to the Pos
        let mut shortest: HashMap<&Pos, u32> = HashMap::new();

        // The dist to initial (itself) is 0
        shortest.insert(initial, 0);

        // Queue with the next elements to visit
        let mut to_visit: VecDeque<&Pos> = VecDeque::new();

        // Add all neighbors to the queue to be visited
        to_visit.extend(self.get_neighbors(initial).iter());

        while let Some(curr_pos) = to_visit.pop_front() {
            // Get all the neighbors of the current pos
            let all_neighbors = self.get_neighbors(curr_pos);

            // Filter all_neighbors to just those that can reach curr_pos and from the resulting
            // ones get the min dist
            let neighbor_with_min_dist = all_neighbors
                .iter()
                .filter(|n| n.value + 1 >= curr_pos.value)
                .filter_map(|valid| shortest.get(valid))
                .min();

            // If there is no neighbor_with_min_dist, the continue to the next iteration
            if neighbor_with_min_dist.is_none() {
                continue;
            }

            // The new distance to reach curr_pos from that neighbor is calculated adding 1
            let new_dist = neighbor_with_min_dist.unwrap() + 1;

            // The get the dist_of_curr_pos from the shortest map (or make it equal to u32::MAX if
            // not in the map yet)
            let dist_of_curr_pos = shortest.entry(curr_pos).or_insert(u32::MAX);

            if *dist_of_curr_pos > new_dist {
                *dist_of_curr_pos = new_dist;

                // Add all neighbors to the queue to be visited next
                to_visit.extend(all_neighbors);
            }
        }

        // Return the value in shortest associated with the END point
        shortest.get(&self.end).copied()
    }

    pub fn get_neighbors(&self, pos: &Pos) -> Vec<&Pos> {
        let dirs: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        dirs.iter()
            .map(|dir| (pos.point.0 as i32 + dir.0, pos.point.1 as i32 + dir.1))
            .filter(|tupl| {
                tupl.0 >= 0
                    && tupl.0 < self.grid[0].len() as i32
                    && tupl.1 >= 0
                    && tupl.1 < self.grid.len() as i32
            })
            .map(|valid_pos| &self.grid[valid_pos.1 as usize][valid_pos.0 as usize])
            .collect()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;

        let grid = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let new_pos = Pos {
                            point: Point(x, y),
                            value: match char {
                                'S' => b'a',
                                'E' => b'z',
                                _ => char as u8,
                            },
                        };
                        match char {
                            'S' => start = Some(new_pos),
                            'E' => end = Some(new_pos),
                            _ => (),
                        }
                        new_pos
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

struct Challenge {
    map: Map,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);

        let map = input.parse::<Map>().unwrap();
        Self { map }
    }

    pub fn task_1(self) {
        println!("{}", self.map.find_path(&self.map.start).unwrap());
    }

    pub fn task_2(&self) {
        let min = self
            .map
            .grid
            .iter()
            .flatten()
            .filter(|pos| pos.value == b'a')
            .filter_map(|pos| self.map.find_path(pos))
            .min();

        println!("{:?}", min.unwrap());
    }
}

pub fn task_1() {
    let ch = Challenge::new();
    ch.task_1();
}
pub fn task_2() {
    let ch = Challenge::new();
    ch.task_2();
}
