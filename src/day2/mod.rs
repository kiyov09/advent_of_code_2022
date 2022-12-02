use crate::utils::get_input_content;
use std::ops::Add;

const INPUT_PATH: &str = "inputs/day_2.txt";

enum OponentMoves {
    A, // Rock
    B, // Paper
    C, // Scissors
}

impl From<&str> for OponentMoves {
    fn from(m: &str) -> Self {
        match m {
            "A" => OponentMoves::A,
            "B" => OponentMoves::B,
            "C" => OponentMoves::C,
            _ => panic!("Something wrong happens"),
        }
    }
}

enum MyMoves {
    X, // Rock
    Y, // Paper
    Z, // Scissors
}

impl From<&str> for MyMoves {
    fn from(m: &str) -> Self {
        match m {
            "X" => MyMoves::X,
            "Y" => MyMoves::Y,
            "Z" => MyMoves::Z,
            _ => panic!("Something wrong happens"),
        }
    }
}

impl MyMoves {
    fn get_points(&self) -> u32 {
        match self {
            MyMoves::X => 1,
            MyMoves::Y => 2,
            MyMoves::Z => 3,
        }
    }
}

struct Play {
    their: OponentMoves,
    mine: MyMoves,
}

impl From<&str> for Play {
    fn from(play_line: &str) -> Self {
        let mut split = play_line.split_whitespace();

        let their = split.next().unwrap();
        let mine = split.next().unwrap();

        Play {
            their: OponentMoves::from(their),
            mine: MyMoves::from(mine),
        }
    }
}

enum PointForResult {
    LOSE,
    DRAW,
    WIN,
}

impl Add<u32> for PointForResult {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            PointForResult::LOSE => rhs,
            PointForResult::DRAW => 3 + rhs,
            PointForResult::WIN => 6 + rhs,
        }
    }
}

impl Play {
    fn get_points(&self) -> u32 {
        match self {
            // LOSE
            Play {
                their: OponentMoves::A,
                mine: MyMoves::Z,
            } => PointForResult::LOSE + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::X,
            } => PointForResult::LOSE + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::Y,
            } => PointForResult::LOSE + self.mine.get_points(),

            // DRAW
            Play {
                their: OponentMoves::A,
                mine: MyMoves::X,
            } => PointForResult::DRAW + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::Y,
            } => PointForResult::DRAW + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::Z,
            } => PointForResult::DRAW + self.mine.get_points(),

            // WIN
            Play {
                their: OponentMoves::A,
                mine: MyMoves::Y,
            } => PointForResult::WIN + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::Z,
            } => PointForResult::WIN + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::X,
            } => PointForResult::WIN + self.mine.get_points(),
        }
    }
}

pub fn task_1() {
    let input = get_input_content(INPUT_PATH);

    let points = input.lines().fold(0, |acc, line| {
        let play = Play::from(line);
        acc + play.get_points()
    });

    println!("Points: {}", points);
}

pub fn task_2() {}
