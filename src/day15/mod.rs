use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::{Chars, FromStr},
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_15.txt";

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Pos(i32, i32);

impl Pos {
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        other.1.abs_diff(self.1) + other.0.abs_diff(self.0)
    }

    pub fn line_coverage(&self, distance: i32, line: i32) -> RangeInclusive<i32> {
        let steps_to_line = self.1.abs_diff(line) as i32;

        if steps_to_line > distance {
            return 0..=0;
        }

        let reminder_steps = distance - steps_to_line;

        let start = self.0 - reminder_steps;
        let end = self.0 + reminder_steps;

        if start <= end {
            start..=end
        } else {
            end..=start
        }
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split(", ")
            .map(|s| s.split('=').nth(1).unwrap().parse::<i32>().unwrap());

        Ok(Self(iter.next().unwrap(), iter.next().unwrap()))
    }
}

type Sensor = Pos;
type Beacon = Pos;

struct Challenge {
    pairs: HashMap<Sensor, Beacon>,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);

        Self {
            pairs: input
                .lines()
                .map(|line| {
                    let mut chars_iter = line.chars();
                    Self::get_pair(&mut chars_iter)
                })
                .collect(),
        }
    }

    fn get_pair(iter: &mut Chars) -> (Sensor, Beacon) {
        let sensor = iter
            .skip_while(|c| c != &'x')
            .take_while(|c| c != &':')
            .collect::<String>()
            .parse::<Pos>()
            .unwrap();

        let beacon = iter
            .skip_while(|c| c != &'x')
            .take_while(|c| c != &':')
            .collect::<String>()
            .parse::<Pos>()
            .unwrap();

        (sensor, beacon)
    }

    pub fn get_no_beacons_positions_in_line(&self, line: usize) -> usize {
        let ranges = self
            .pairs
            .iter()
            .map(|p| {
                p.0.line_coverage(p.0.manhattan_distance(p.1) as i32, line as i32)
            })
            .collect::<Vec<_>>();

        let min_start = ranges.iter().min_by_key(|r| r.start()).unwrap().start();
        let max_end = ranges.iter().max_by_key(|r| r.end()).unwrap().end();

        let mut count = 0;
        for x in *min_start..=*max_end {
            if ranges.iter().any(|r| r.contains(&x)) {
                count += 1;
                continue;
            }
        }

        count
    }

    pub fn beacons_in_line(&self, line: usize) -> usize {
        self.pairs
            .values()
            .filter(|p| p.1 == line as i32)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn get_hidden_beacon_pos(&self, from: usize, to: usize) -> Pos {
        for y in from..=to {
            let mut ranges = self
                .pairs
                .iter()
                .map(|pair| {
                    pair.0
                        .line_coverage(pair.0.manhattan_distance(pair.1) as i32, y as i32)
                })
                .filter(|range| range.start() != range.end())
                .collect::<Vec<_>>();

            ranges.sort_by_key(|range| *range.start());

            let mut ranges_iter = ranges.iter();
            let mut current_end = *ranges_iter.next().unwrap().end();

            for range in ranges_iter {
                if range.end() <= &current_end {
                    continue;
                }

                let diff = range.start() - current_end;
                if range.start() > &current_end && diff > 1 {
                    let pos = Pos(current_end + 1, y as i32);
                    return pos;
                }

                current_end = *range.end();
            }
        }

        Pos(0, 0)
    }
}

pub fn task_1() {
    let ch = Challenge::new();

    let line = 2000000;

    let beacons_in_line = ch.beacons_in_line(line);
    let line_coverage = ch.get_no_beacons_positions_in_line(line);

    println!(
        "Number of pos with no beacon: {}",
        line_coverage - beacons_in_line
    );
}

pub fn task_2() {
    let ch = Challenge::new();

    let hidden_beacon = ch.get_hidden_beacon_pos(0, 4000000);
    let tuning_freq: u128 = 4000000 * hidden_beacon.0 as u128 + hidden_beacon.1 as u128;

    println!("Tuning frequency: {}", tuning_freq);
}
