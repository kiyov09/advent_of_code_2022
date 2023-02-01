use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_20.txt";

type Elem = (i64, bool, usize);

struct Challenge {
    data: Vec<Elem>,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        Self {
            data: input
                .lines()
                .enumerate()
                .map(|(idx, i)| (i.parse().unwrap(), false, idx))
                .collect(),
        }
    }

    pub fn process(&mut self, rounds: usize) -> i64 {
        let original = self.data.clone();
        let iter = original.iter().cycle().take(original.len() * rounds);

        for item in iter {
            let item_pos = self.data.iter().position(|i| i.2 == item.2).unwrap();

            if item.0 != 0 {
                self.move_value_to_pos(item_pos, item.0);
            }
        }

        let pos_of_0 = self.data.iter().position(|item| item.0 == 0).unwrap();
        let result = [1000, 2000, 3000]
            .iter()
            .map(|idx| self.get_at_index(pos_of_0, *idx))
            .sum();

        result
    }

    pub fn move_value_to_pos(&mut self, from: usize, delta: i64) {
        let org_index = self.data[from].2;

        let mut idx = from as i64 + delta;
        idx = idx.rem_euclid(self.data.len() as i64 - 1);

        self.data.remove(from);
        self.data.insert(idx as usize, (delta, true, org_index));
    }

    pub fn get_at_index(&self, from: usize, idx: usize) -> i64 {
        self.data.iter().cycle().skip(from).nth(idx).unwrap().0
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();
    let result = ch.process(1);

    println!("Sum of grove coords: {result}");
}

pub fn task_2() {
    let mut ch = Challenge::new();

    ch.data.iter_mut().for_each(|i| i.0 *= 811589153);
    let result = ch.process(10);

    println!("Sum of grove coords (decryption): {result}");
}
