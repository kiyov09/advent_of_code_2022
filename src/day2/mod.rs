use crate::utils::get_input_content;
use std::ops::Add;

const INPUT_PATH: &str = "inputs/day_2.txt";

// OPONENT MOVES
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

// MY MOVES
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

// PLAY
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

impl Play {
    fn get_points(&self) -> u32 {
        match self {
            // LOSE
            Play {
                their: OponentMoves::A,
                mine: MyMoves::Z,
            } => PointForResult::Lose + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::X,
            } => PointForResult::Lose + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::Y,
            } => PointForResult::Lose + self.mine.get_points(),

            // DRAW
            Play {
                their: OponentMoves::A,
                mine: MyMoves::X,
            } => PointForResult::Draw + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::Y,
            } => PointForResult::Draw + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::Z,
            } => PointForResult::Draw + self.mine.get_points(),

            // WIN
            Play {
                their: OponentMoves::A,
                mine: MyMoves::Y,
            } => PointForResult::Win + self.mine.get_points(),
            Play {
                their: OponentMoves::B,
                mine: MyMoves::Z,
            } => PointForResult::Win + self.mine.get_points(),
            Play {
                their: OponentMoves::C,
                mine: MyMoves::X,
            } => PointForResult::Win + self.mine.get_points(),
        }
    }

    fn from_elf_strategy(play_line: &str) -> Self {
        let mut split = play_line.split_whitespace();

        let their = split.next().unwrap();
        let their = OponentMoves::from(their);

        let mine = split.next().unwrap();

        let (their, mine) = match mine {
            // I need to lose
            "X" => match their {
                OponentMoves::A => (their, MyMoves::Z),
                OponentMoves::B => (their, MyMoves::X),
                OponentMoves::C => (their, MyMoves::Y),
            },
            // I need to draw
            "Y" => match their {
                OponentMoves::A => (their, MyMoves::X),
                OponentMoves::B => (their, MyMoves::Y),
                OponentMoves::C => (their, MyMoves::Z),
            },
            // I need to win
            "Z" => match their {
                OponentMoves::A => (their, MyMoves::Y),
                OponentMoves::B => (their, MyMoves::Z),
                OponentMoves::C => (their, MyMoves::X),
            },
            _ => panic!("Wrong input data"),
        };

        Play { their, mine }
    }
}

// POINTS BASED ON PLAY RESULT
enum PointForResult {
    Lose,
    Draw,
    Win,
}

impl Add<u32> for PointForResult {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        match self {
            PointForResult::Lose => rhs,
            PointForResult::Draw => 3 + rhs,
            PointForResult::Win => 6 + rhs,
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

pub fn task_2() {
    let input = get_input_content(INPUT_PATH);

    let points = input.lines().fold(0, |acc, line| {
        let play = Play::from_elf_strategy(line);
        acc + play.get_points()
    });

    println!("Points using elf strategy: {}", points);
}
