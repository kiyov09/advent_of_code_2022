use std::{cmp, num::ParseIntError, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_19.txt";

type Ore = usize;
type Clay = usize;
type Obsidian = usize;
type Geode = usize;

#[derive(Debug, Clone)]
struct Warehouse {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
}

impl Default for Warehouse {
    fn default() -> Self {
        Self {
            ore: 1,
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
struct Blueprint {
    ore: Ore,
    clay: Ore,
    obsidian: (Ore, Clay),
    geode: (Ore, Obsidian),
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');

        // Ore
        let ore = split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        // Clay
        let clay = split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        // Obsidian
        let mut obsidian_line_split = split.next().unwrap().split(" and ");

        let o_ore = obsidian_line_split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let o_clay = obsidian_line_split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        // Geode
        let mut geode_line_split = split.next().unwrap().split(" and ");

        let g_ore = geode_line_split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let g_obs = geode_line_split
            .next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c != &' ')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            ore,
            clay,
            obsidian: (o_ore, o_clay),
            geode: (g_ore, g_obs),
        })
    }
}

#[derive(Debug, Clone)]
enum Robots {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}

impl From<Robots> for usize {
    fn from(value: Robots) -> Self {
        match value {
            Robots::Ore => 0,
            Robots::Clay => 1,
            Robots::Obsidian => 2,
            Robots::Geode => 3,
        }
    }
}

#[derive(Default, Debug)]
struct Factory {
    id: usize,
    warehouse: Warehouse,
    blueprint: Blueprint,
    robots: [usize; 4],
}

impl Factory {
    fn new(id: usize, blueprint: Blueprint) -> Self {
        Self {
            id,
            blueprint,
            robots: [1, 0, 0, 0],
            ..Default::default()
        }
    }

    fn collect_geodes(&mut self, state: FactoryState) -> usize {
        if state.time == 0 {
            return state.warehouse.geode;
        }

        let mut max_geodes = state.warehouse.geode;

        for kind in [Robots::Geode, Robots::Obsidian, Robots::Clay, Robots::Ore] {
            match kind {
                Robots::Geode => {
                    let mut new_state = state.clone();

                    let ore_robots = *new_state.robots.get(usize::from(Robots::Ore)).unwrap_or(&0);
                    let obs_robots = *new_state
                        .robots
                        .get(usize::from(Robots::Obsidian))
                        .unwrap_or(&0);

                    // Do I need to build another obsidian robot
                    if obs_robots > 0 {
                        let delta_time = cmp::max(
                            time_to(
                                self.blueprint.geode.0 as i32,
                                new_state.warehouse.ore as i32,
                                ore_robots as i32,
                            ),
                            time_to(
                                self.blueprint.geode.1 as i32,
                                new_state.warehouse.obsidian as i32,
                                obs_robots as i32,
                            ),
                        ) as usize;

                        if delta_time < new_state.time {
                            new_state.update_warehouse(delta_time);
                            new_state.add_robot(Robots::Geode);

                            new_state.warehouse.ore -= self.blueprint.geode.0;
                            new_state.warehouse.obsidian -= self.blueprint.geode.1;

                            new_state.time -= delta_time;
                        } else {
                            new_state.update_warehouse(new_state.time);
                            new_state.time -= new_state.time;
                        }

                        max_geodes = max_geodes.max(self.collect_geodes(new_state));
                    }
                }
                Robots::Obsidian => {
                    let mut new_state = state.clone();

                    let ore_robots = *new_state.robots.get(usize::from(Robots::Ore)).unwrap_or(&0);
                    let clay_robots = *new_state
                        .robots
                        .get(usize::from(Robots::Clay))
                        .unwrap_or(&0);
                    let obs_robots = *new_state
                        .robots
                        .get(usize::from(Robots::Obsidian))
                        .unwrap_or(&0);

                    // Do I need to build another obsidian robot
                    if clay_robots > 0
                        && new_state.warehouse.obsidian + (new_state.time * obs_robots)
                            < self.blueprint.geode.1 * new_state.time
                    {
                        let delta_time = cmp::max(
                            time_to(
                                self.blueprint.obsidian.0 as i32,
                                new_state.warehouse.ore as i32,
                                ore_robots as i32,
                            ),
                            time_to(
                                self.blueprint.obsidian.1 as i32,
                                new_state.warehouse.clay as i32,
                                clay_robots as i32,
                            ),
                        ) as usize;

                        if delta_time < new_state.time {
                            new_state.update_warehouse(delta_time);
                            new_state.add_robot(Robots::Obsidian);

                            new_state.warehouse.ore -= self.blueprint.obsidian.0;
                            new_state.warehouse.clay -= self.blueprint.obsidian.1;

                            new_state.time -= delta_time;
                        } else {
                            new_state.update_warehouse(new_state.time);
                            new_state.time -= new_state.time;
                        }

                        max_geodes = max_geodes.max(self.collect_geodes(new_state));
                    }
                }
                Robots::Clay => {
                    let mut new_state = state.clone();

                    let clay_robots = *new_state
                        .robots
                        .get(usize::from(Robots::Clay))
                        .unwrap_or(&0);
                    let ore_robots = *new_state.robots.get(usize::from(Robots::Ore)).unwrap_or(&0);

                    // Do I need to build another clay robot
                    if new_state.warehouse.clay + (new_state.time * clay_robots)
                        < self.blueprint.obsidian.1 * new_state.time
                    {
                        let delta_time = time_to(
                            self.blueprint.clay as i32,
                            new_state.warehouse.ore as i32,
                            ore_robots as i32,
                        ) as usize;

                        if delta_time < new_state.time {
                            new_state.update_warehouse(delta_time);
                            new_state.add_robot(Robots::Clay);

                            new_state.warehouse.ore -= self.blueprint.clay;

                            new_state.time -= delta_time;
                        } else {
                            new_state.update_warehouse(new_state.time);
                            new_state.time -= new_state.time;
                        }

                        max_geodes = max_geodes.max(self.collect_geodes(new_state));
                    }
                }
                Robots::Ore => {
                    let mut new_state = state.clone();

                    let ore_robots = *new_state.robots.get(usize::from(Robots::Ore)).unwrap_or(&0);

                    // Do I need to build another ore robot
                    if new_state.warehouse.ore + (new_state.time * ore_robots)
                        < self.max_necessary_ores() * new_state.time
                    {
                        let delta_time = time_to(
                            self.blueprint.ore as i32,
                            new_state.warehouse.ore as i32,
                            ore_robots as i32,
                        ) as usize;

                        if delta_time < new_state.time {
                            new_state.update_warehouse(delta_time);
                            new_state.add_robot(Robots::Ore);

                            new_state.warehouse.ore -= self.blueprint.ore;

                            new_state.time -= delta_time;
                        } else {
                            new_state.update_warehouse(new_state.time);
                            new_state.time -= new_state.time;
                        }

                        max_geodes = max_geodes.max(self.collect_geodes(new_state));
                    }
                }
            }
        }

        max_geodes
    }

    fn max_necessary_ores(&self) -> usize {
        *[
            self.blueprint.clay,
            self.blueprint.obsidian.0,
            self.blueprint.geode.0,
        ]
        .iter()
        .max()
        .unwrap()
    }
}

fn time_to(cost: i32, current: i32, producer: i32) -> i32 {
    cmp::max((cost - current + producer - 1) / producer, 0) + 1
}

#[derive(Debug, Clone)]
struct FactoryState {
    robots: [usize; 4],
    warehouse: Warehouse,
    time: usize,
}

impl FactoryState {
    fn update_warehouse(&mut self, delta_time: usize) {
        self.robots.iter().enumerate().for_each(|(idx, amount)| {
            match idx {
                0 => self.warehouse.ore += amount * delta_time,
                1 => self.warehouse.clay += amount * delta_time,
                2 => self.warehouse.obsidian += amount * delta_time,
                3 => self.warehouse.geode += amount * delta_time,
                _ => unreachable!("Can't have more than 4 kind of robots"),
            };
        });
    }

    fn add_robot(&mut self, kind: Robots) {
        self.robots[usize::from(kind)] += 1;
    }
}

struct Challenge {
    factories: Vec<Factory>,
}

impl Challenge {
    fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        let factories = input
            .lines()
            .map(|line| {
                let (id_spec, blueprint_spec) = line.split_once(':').unwrap();

                let fac_id = id_spec
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let blueprint = blueprint_spec.parse::<Blueprint>().unwrap();

                Factory::new(fac_id, blueprint)
            })
            .collect();

        Self { factories }
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();

    let result = ch
        .factories
        .iter_mut()
        .map(|fac| {
            fac.collect_geodes(FactoryState {
                robots: fac.robots,
                warehouse: fac.warehouse.clone(),
                time: 23,
            }) * fac.id
        })
        .sum::<usize>();

    println!("Sum of all blueprints: {}", result);
}

pub fn task_2() {
    let mut ch = Challenge::new();

    let result = ch
        .factories
        .iter_mut()
        .take(3)
        .map(|fac| {
            fac.collect_geodes(FactoryState {
                robots: fac.robots,
                warehouse: fac.warehouse.clone(),
                time: 31,
            })
        })
        .product::<usize>();

    println!("Product of first 3 blueprints: {}", result);
}
