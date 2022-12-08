use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_8.txt";

enum VisibilityDirection {
    Left,
    Right,
    Up,
    Down,
}

struct Tree {
    value: u8,
    visible: bool,
    view_distance: u32,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {}({}, {}) ",
            self.value, self.visible, self.view_distance
        )
    }
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            visible: self.visible,
            view_distance: 0,
        }
    }
}

impl FromStr for Tree {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse()?,
            visible: false,
            view_distance: 0,
        })
    }
}

struct Map {
    width: usize,
    height: usize,
    layers: Vec<Vec<Tree>>,
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layers: Vec<Vec<Tree>> = s
            .lines()
            .enumerate()
            .map(|(index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(idx, i)| {
                        let mut t = format!("{i}").parse::<Tree>().unwrap();
                        if index == 0 || index + 1 == line.len() {
                            t.visible = true;
                        }
                        if idx == 0 || idx + 1 == line.len() {
                            t.visible = true;
                        }
                        t
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            width: layers[0].len(),
            height: layers[0].len(),
            layers,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map_as_str = self.layers.iter().flatten().enumerate().fold(
            String::new(),
            |mut acc, (index, item)| {
                if index != 0 && index % self.width == 0 {
                    acc.push('\n');
                }
                acc.push_str(&item.to_string());
                acc
            },
        );

        write!(f, "{}", map_as_str)
    }
}

impl Map {
    pub fn update_trees_visibility(&mut self) {
        for i in 1..self.height - 1 {
            for j in 1..self.width - 1 {
                // From the left
                let mut view_distance_left = 0;
                let tree = &self.layers[i][j];
                let any = self.layers[i][..j].iter().rev().all(|other_tree| {
                    let is_shorter = other_tree.value < tree.value;
                    view_distance_left += 1;
                    is_shorter
                });

                if any {
                    self.layers[i][j].visible = any;
                }

                // From the right
                let mut view_distance_right = 0;
                let tree = &self.layers[i][j];
                let any = self.layers[i][j + 1..].iter().all(|other_tree| {
                    let is_shorter = other_tree.value < tree.value;
                    view_distance_right += 1;
                    is_shorter
                });

                if any {
                    self.layers[i][j].visible = any;
                }

                // From top
                let mut view_distance_top = 0;
                let tree = &self.layers[i][j];
                let any = self.layers[..i].iter().rev().all(|other_tree| {
                    let is_shorter = other_tree[j].value < tree.value;
                    view_distance_top += 1;
                    is_shorter
                });

                if any {
                    self.layers[i][j].visible = any;
                }

                // From bottom
                let mut view_distance_bottom = 0;
                let tree = &self.layers[i][j];
                let any = self.layers[i + 1..].iter().all(|other_tree| {
                    let is_shorter = other_tree[j].value < tree.value;
                    view_distance_bottom += 1;
                    is_shorter
                });

                if any {
                    self.layers[i][j].visible = any;
                }

                // if i == 1 && j == 2 {
                //     println!(
                //         "{} {} {} {}",
                //         view_distance_top,
                //         view_distance_left,
                //         view_distance_right,
                //         view_distance_bottom,
                //     );
                // }

                // if i == 3 && j == 2 {
                //     println!(
                //         "{} {} {} {}",
                //         view_distance_top,
                //         view_distance_left,
                //         view_distance_bottom,
                //         view_distance_right,
                //     );
                // }

                let view_distance = [
                    view_distance_top,
                    view_distance_left,
                    view_distance_bottom,
                    view_distance_right,
                ]
                .iter()
                .map(|i| if *i == 0 { 1 } else { *i })
                .product();

                self.layers[i][j].view_distance = view_distance;
            }
        }
    }
}

struct Challenge {
    data: Map,
}

impl Challenge {
    pub fn new() -> Self {
        let s = get_input_content(INPUT_PATH);

        let mut data = s.parse::<Map>().unwrap();
        data.update_trees_visibility();

        Self { data }
    }

    pub fn task_1(&mut self) {
        let visible_trees = self
            .data
            .layers
            .iter()
            .flatten()
            .filter(|tree| tree.visible)
            .count();

        println!("Number of visible trees: {visible_trees}");
    }

    pub fn task_2(&self) {
        // self.print_map();

        let max = self
            .data
            .layers
            .iter()
            .flatten()
            .max_by(|a, b| a.view_distance.cmp(&b.view_distance));

        println!("Best scenic score: {}", max.unwrap().view_distance);
    }

    pub fn print_map(&self) {
        println!("{}", self.data);
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();
    ch.task_1();
}

pub fn task_2() {
    let ch = Challenge::new();
    ch.task_2();
}
