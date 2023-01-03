use std::{cmp::Ordering, collections::VecDeque, fmt::Display, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_13.txt";

#[derive(Debug, PartialEq, Eq)]
enum PacketItem {
    Single(u32),
    Multi(Vec<PacketItem>),
}

impl Display for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketItem::Single(a) => write!(f, "{}", a),
            PacketItem::Multi(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Both are single values
            (PacketItem::Single(a), PacketItem::Single(b)) => a.cmp(b),

            // Both are multi
            (PacketItem::Multi(a), PacketItem::Multi(b)) => {
                for (a_item, b_item) in a.iter().zip(b.iter()) {
                    match a_item.cmp(b_item) {
                        Ordering::Equal => continue,
                        other => return other,
                    };
                }

                a.len().cmp(&b.len())
            }

            // When they are different types
            (a, PacketItem::Single(b)) => {
                let new_b = PacketItem::Multi(vec![PacketItem::Single(*b)]);
                a.cmp(&new_b)
            }

            (PacketItem::Single(a), b) => {
                let new_a = PacketItem::Multi(vec![PacketItem::Single(*a)]);
                new_a.cmp(b)
            }
        }
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    data: PacketItem,
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Packet {
    pub fn process_array(v: &mut VecDeque<char>) -> PacketItem {
        let mut result = Vec::new();
        let mut temp = String::from("");

        while let Some(c) = v.pop_front() {
            match c {
                '[' => result.push(Self::process_array(v)),
                ']' => {
                    if !temp.is_empty() {
                        result.push(PacketItem::Single(temp.parse().unwrap()));
                        temp.clear();
                    }
                    break;
                }
                ',' => {
                    if !temp.is_empty() {
                        result.push(PacketItem::Single(temp.parse().unwrap()));
                        temp.clear();
                    }
                    continue;
                }
                _ => temp.push(c),
            }
        }

        PacketItem::Multi(result)
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: Self::process_array(&mut s.chars().collect::<VecDeque<char>>()),
        })
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    pub fn is_in_order(&self) -> bool {
        self.left.data < self.right.data
    }
}

struct Challenge {
    pairs: Vec<Pair>,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);

        let pairs = input
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|ch| Pair {
                left: ch[0].parse::<Packet>().unwrap(),
                right: ch[1].parse::<Packet>().unwrap(),
            })
            .collect();

        Self { pairs }
    }
}

pub fn task_1() {
    let c = Challenge::new();
    let result = c.pairs.iter().enumerate().fold(0, |mut acc, (idx, p)| {
        if p.is_in_order() {
            acc += idx + 1;
        }
        acc
    });

    println!("Sum of idx: {}", result);
}
pub fn task_2() {
    let c = Challenge::new();

    let mut all_packets = c
        .pairs
        .iter()
        .flat_map(|p| vec![&p.left.data, &p.right.data])
        .collect::<Vec<&PacketItem>>();

    let divider_1 = PacketItem::Multi(vec![PacketItem::Multi(vec![PacketItem::Single(2)])]);
    let divider_2 = PacketItem::Multi(vec![PacketItem::Multi(vec![PacketItem::Single(6)])]);

    all_packets.push(&divider_1);
    all_packets.push(&divider_2);

    all_packets.sort();

    let divider_1_pos = all_packets.iter().position(|p| *p == &divider_1).unwrap();
    let divider_2_pos = all_packets.iter().position(|p| *p == &divider_2).unwrap();

    println!(
        "Mul of dividers: {}",
        (divider_1_pos + 1) * (divider_2_pos + 1)
    )
}
