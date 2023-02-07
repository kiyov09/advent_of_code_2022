use std::{fmt::Display, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_22.txt";

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!("Not a valid direction"),
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Move(i8),
    Rotate(Dir),
}

impl From<char> for Cmd {
    fn from(value: char) -> Self {
        Cmd::Rotate(value.into())
    }
}

impl From<i8> for Cmd {
    fn from(value: i8) -> Self {
        Cmd::Move(value)
    }
}

impl From<Cmd> for i8 {
    fn from(value: Cmd) -> Self {
        match value {
            Cmd::Move(v) => v,
            Cmd::Rotate(_) => panic!("Can't convert Rotate to i8"),
        }
    }
}

#[derive(Debug, Clone)]
enum Facing {
    Left,
    Right,
    Top,
    Down,
}

impl Facing {
    pub fn rotate(&mut self, direction: Dir) {
        match (&self, direction) {
            (Facing::Left, Dir::Left) => *self = Facing::Down,
            (Facing::Left, Dir::Right) => *self = Facing::Top,
            (Facing::Right, Dir::Left) => *self = Facing::Top,
            (Facing::Right, Dir::Right) => *self = Facing::Down,
            (Facing::Top, Dir::Left) => *self = Facing::Left,
            (Facing::Top, Dir::Right) => *self = Facing::Right,
            (Facing::Down, Dir::Left) => *self = Facing::Right,
            (Facing::Down, Dir::Right) => *self = Facing::Left,
        }
    }
}

impl From<Facing> for usize {
    fn from(value: Facing) -> Self {
        match value {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Top => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Open,
    Wall,
    Void,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Open,
            '#' => Cell::Wall,
            ' ' => Cell::Void,
            _ => unreachable!("Not valid cell char"),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Open => '.',
            Cell::Wall => '#',
            Cell::Void => ' ',
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Cell>>,
    commands: Vec<Cmd>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|row| row.iter().map(char::from).collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((map, commands)) = s.split_once("\n\n") else { todo!() };

        let mut temp = String::new();
        let mut cmds: Vec<Cmd> = Vec::new();

        let map = map
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        commands
            .chars()
            .take_while(|c| !c.is_whitespace())
            .for_each(|c| match c.is_ascii_digit() {
                true => {
                    temp.push(c);
                }
                false => {
                    cmds.push(temp.parse::<i8>().unwrap().into());
                    temp.clear();

                    cmds.push(c.into());
                }
            });

        Ok(Self {
            map,
            commands: cmds,
        })
    }
}

#[derive(Debug)]
struct Actor {
    facing: Facing,
    pos: (usize, usize),
}

impl Actor {
    pub fn rotate(&mut self, dir: Dir) {
        self.facing.rotate(dir);
    }
}

#[derive(Debug)]
struct Challenge {
    map: Map,
    actor: Actor,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        let map: Map = input.parse().unwrap();

        let initial_pos = (
            map.map
                .get(0)
                .unwrap()
                .iter()
                .position(|col| col == &Cell::Open)
                .unwrap(),
            0,
        );

        Self {
            map,
            actor: Actor {
                facing: Facing::Right,
                pos: initial_pos,
            },
        }
    }

    pub fn process_commands(&mut self) {
        for cmd in self.map.commands.iter() {
            let actor_pos = &mut self.actor.pos;
            match cmd {
                Cmd::Move(n) => match self.actor.facing {
                    Facing::Left => {
                        let row = self.map.map.get(actor_pos.1).unwrap();
                        let last = row
                            .iter()
                            .enumerate()
                            .rev()
                            .cycle()
                            .skip(row.len() - actor_pos.0)
                            .filter(|(_, cell)| *cell != &Cell::Void)
                            .take(*n as usize)
                            .take_while(|(_, cell)| *cell == &Cell::Open)
                            .last();

                        if let Some((idx, _)) = last {
                            actor_pos.0 = idx;
                        }
                    }
                    Facing::Right => {
                        let row = self.map.map.get(actor_pos.1).unwrap();
                        let last = row
                            .iter()
                            .enumerate()
                            .cycle()
                            .skip(actor_pos.0 + 1)
                            .filter(|(_, cell)| *cell != &Cell::Void)
                            .take(*n as usize)
                            .take_while(|(_, cell)| *cell == &Cell::Open)
                            .last();

                        if let Some((idx, _)) = last {
                            actor_pos.0 = idx;
                        }
                    }
                    Facing::Top => {
                        let last = self
                            .map
                            .map
                            .iter()
                            .enumerate()
                            .rev()
                            .cycle()
                            .map(|(idx, row)| {
                                (
                                    idx,
                                    match row.get(actor_pos.0) {
                                        Some(cell) => cell,
                                        None => &Cell::Void,
                                    },
                                )
                            })
                            .skip(self.map.map.len() - actor_pos.1)
                            .filter(|(_, cell)| *cell != &Cell::Void)
                            .take(*n as usize)
                            .take_while(|(_, cell)| *cell == &Cell::Open)
                            .last();

                        if let Some((idx, _)) = last {
                            actor_pos.1 = idx;
                        }
                    }
                    Facing::Down => {
                        let last = self
                            .map
                            .map
                            .iter()
                            .enumerate()
                            .cycle()
                            .map(|(idx, row)| {
                                (
                                    idx,
                                    match row.get(actor_pos.0) {
                                        Some(cell) => cell,
                                        None => &Cell::Void,
                                    },
                                )
                            })
                            .skip(actor_pos.1 + 1)
                            .filter(|(_, cell)| *cell != &Cell::Void)
                            .take(*n as usize)
                            .take_while(|(_, cell)| *cell == &Cell::Open)
                            .last();

                        if let Some((idx, _)) = last {
                            actor_pos.1 = idx;
                        }
                    }
                },
                Cmd::Rotate(dir) => self.actor.rotate(*dir),
            };
        }
    }

    pub fn get_password(&self) -> usize {
        1000 * (self.actor.pos.1 + 1)
            + 4 * (self.actor.pos.0 + 1)
            + usize::from(self.actor.facing.clone())
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();
    ch.process_commands();

    println!("The password is: {}", ch.get_password());
}

pub fn task_2() {}
