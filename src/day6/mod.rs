use std::collections::HashSet;

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_6.txt";

struct Marker<'v> {
    value: &'v str,
    end_idx: usize,
}

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

struct Challenge<'a> {
    stream: &'a str,
}

impl<'a> Challenge<'a> {
    fn new(stream: &'a str) -> Self {
        Self { stream }
    }

    fn start_of_packet(&self) -> Marker {
        self.find_marker(PACKET_MARKER_SIZE)
    }

    fn start_of_message(&self) -> Marker {
        self.find_marker(MESSAGE_MARKER_SIZE)
    }

    fn find_marker(&self, marker_size: usize) -> Marker {
        let stream_len = self.stream.len();
        let mut pointer = 0;

        while pointer + marker_size < stream_len {
            let sub = self.stream.get(pointer..pointer + marker_size).unwrap();

            let hs: HashSet<char> = HashSet::from_iter(sub.chars());
            if hs.len() == marker_size {
                return Marker {
                    value: sub,
                    end_idx: pointer + marker_size,
                };
            }

            pointer += 1;

            if pointer + marker_size > stream_len {
                break;
            }
        }

        panic!("Something went wrong")
    }

    fn task_1(&self) {
        let Marker { value, end_idx } = self.start_of_packet();
        println!("The package marker '{value}' end on index: {end_idx}");
    }

    fn task_2(&self) {
        let Marker { value, end_idx } = self.start_of_message();
        println!("The message marker '{value}' end on index: {end_idx}");
    }
}

pub fn task_1() {
    let input = get_input_content(INPUT_PATH);
    Challenge::new(&input).task_1();
}
pub fn task_2() {
    let input = get_input_content(INPUT_PATH);
    Challenge::new(&input).task_2();
}
