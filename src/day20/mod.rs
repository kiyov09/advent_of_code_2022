use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_20.txt";

type Elem = (i32, bool);

struct Challenge {
    data: Vec<Elem>,
    cursor: usize,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        Self {
            data: input.lines().map(|i| (i.parse().unwrap(), false)).collect(),
            cursor: 0,
        }
    }

    pub fn process(&mut self) -> i32 {
        while self.cursor < self.data.len() {
            let item = self.data[self.cursor];

            if !item.1 && item.0 != 0 {
                self.move_value_to_pos(self.cursor, item.0);
            } else {
                self.cursor += 1;
            }
        }

        let pos_of_0 = self.data.iter().position(|item| item.0 == 0).unwrap();
        let result = [1000, 2000, 3000]
            .iter()
            .map(|idx| self.get_at_index(pos_of_0, *idx))
            .sum();

        result
    }

    pub fn move_value_to_pos(&mut self, from: usize, delta: i32) {
        match delta.is_positive() {
            true => {
                let mut idx = from as i32 + delta;

                if idx > self.data.len() as i32 - 1 {
                    idx %= self.data.len() as i32 - 1;
                }

                self.data.remove(from);
                self.data.insert(idx as usize, (delta, true));

                // match idx <= self.cursor as i32 {
                //     true => self.cursor += 1,
                //     false => (),
                // };
            }
            false => {
                let mut idx = from as i32 + delta;

                if idx < 0 {
                    idx = self.data.len() as i32 - 1 - (idx.abs() % self.data.len() as i32);
                }

                self.data.remove(from);

                if idx == 0 {
                    self.data.push((delta, true));
                } else {
                    self.data.insert(idx as usize, (delta, true));
                }

                // match idx <= self.cursor as i32 {
                //     false => (),
                //     true => self.cursor += 1,
                // };
            }
        }
    }

    pub fn get_at_index(&self, from: usize, idx: usize) -> i32 {
        self.data.iter().cycle().skip(from).nth(idx).unwrap().0
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();
    let result = ch.process();

    println!("Sum of grove coordinates: {result}");
}
pub fn task_2() {}
