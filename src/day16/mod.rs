use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_16.txt";

#[derive(Default, Debug, Eq)]
struct Valve {
    id: String,
    idx: usize,
    rate: usize,
    paths: Vec<String>,
    usefull_paths: HashMap<String, usize>,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.rate.hash(state);
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Challenge {
    valves: HashMap<String, Valve>,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);

        Self {
            valves: input
                .lines()
                .enumerate()
                .map(|(idx, line)| Self::parse_line(line, idx))
                .collect(),
        }
    }

    fn parse_line(line: &str, idx: usize) -> (String, Valve) {
        let mut split = line.split_whitespace();
        let id = split.nth(1).unwrap();
        let rate = split.nth(2).unwrap();

        let split = split.skip(4);

        (
            id.to_string(),
            Valve {
                id: id.to_string(),
                idx,
                rate: Self::parse_rate(rate),
                paths: split
                    .collect::<String>()
                    .split(',')
                    .map(|str| str.to_string())
                    .collect(),
                usefull_paths: HashMap::default(),
            },
        )
    }

    fn parse_rate(rate_str: &str) -> usize {
        rate_str
            .split('=')
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c != &';')
            .collect::<String>()
            .parse()
            .unwrap()
    }

    pub fn cost_of_travelling(&self, start: String) -> HashMap<String, usize> {
        let current = self.valves.get(&start).unwrap();

        // To store the costs
        let mut costs = HashMap::new();

        // To iterate
        let mut queue = VecDeque::new();
        // Add starting point to the queu
        queue.push_back((current, 0usize));

        // Visited already
        let mut visited = HashSet::new();
        // Mark starting point as visited
        visited.insert(current);

        while let Some((node, cost)) = queue.pop_front() {
            let paths = &node.paths;

            for p in paths {
                let valve = self.valves.get(p).unwrap();

                if !visited.insert(valve) {
                    // Already visited
                    continue;
                }

                if valve.rate > 0 {
                    // Add to the costs only if vale has rate
                    costs.insert(p.to_string(), cost + 1);
                }
                queue.push_back((valve, cost + 1));
            }
        }

        costs
    }
}

#[derive(Eq, Clone)]
struct SearchState {
    current: String,
    // Each pos in the vec represents the Valve with that idx
    // If that pos is true, that Valve is already on
    on: Vec<bool>,
    time: usize,
    helper: bool,
}

impl Hash for SearchState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        self.on.iter().for_each(|item| item.hash(state));
        self.time.hash(state);
        self.helper.hash(state);
    }
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
            && self.on == other.on
            && self.time == other.time
            && self.helper == other.helper
    }
}

#[derive(Default)]
struct Search {
    seen: HashMap<SearchState, usize>,
    time: usize,
    helper: bool,
}

impl Search {
    pub fn get_initial_state(time: usize, helper: bool, valves_count: usize) -> SearchState {
        SearchState {
            current: "AA".to_string(),
            on: vec![false; valves_count],
            time,
            helper,
        }
    }

    pub fn bfs(&mut self, state: Option<SearchState>, valves: &HashMap<String, Valve>) -> usize {
        let state = match state {
            Some(state) => state,
            None => Self::get_initial_state(self.time, self.helper, valves.len()),
        };

        // If current has a value already, returns it
        if let Some(v) = self.seen.get(&state) {
            return *v;
        }

        let mut max = 0;

        max = if state.helper {
            max.max(self.bfs(
                Some(SearchState {
                    current: "AA".to_string(),
                    on: state.on.clone(),
                    time: 26,
                    helper: false,
                }),
                valves,
            ))
        } else {
            0
        };

        if state.time == 0 {
            return 0;
        }

        let valve = valves.get(&state.current).unwrap();

        // Visit current node (turn it on if it is not already)
        if !state.on[valve.idx] && valve.rate > 0 {
            // Calculate the flow if open this valve
            let flow = valve.rate * (state.time - 1);

            // Create the new "on valves" set
            let mut new_on = state.on.clone();
            new_on[valve.idx] = true;

            // Create the new search state
            let new_state = SearchState {
                current: state.current.clone(),
                on: new_on,
                time: state.time - 1,
                helper: state.helper,
            };

            max = max.max(self.bfs(Some(new_state), valves) + flow);
        }

        // Analyze the nodes current can reach
        max = valve
            .usefull_paths
            .iter()
            .map(|(p, cost)| {
                if *cost > state.time {
                    return max;
                }
                let new_state = SearchState {
                    current: p.clone(),
                    on: state.on.clone(),
                    time: state.time - *cost,
                    helper: state.helper,
                };
                max.max(self.bfs(Some(new_state), valves))
            })
            .max()
            .unwrap();

        self.seen.insert(state.clone(), max);

        // Return the max found
        max
    }
}

pub fn task_1() {
    let mut ch = Challenge::new();

    let valve_names = ch.valves.keys().cloned().collect::<Vec<_>>();

    for valve_name in valve_names {
        ch.valves.get_mut(&valve_name).unwrap().usefull_paths =
            ch.cost_of_travelling(valve_name.to_string());
    }

    let mut search = Search {
        time: 30,
        ..Default::default()
    };
    let max_flow = search.bfs(None, &ch.valves);

    println!("Most possible pressure released: {max_flow}");
}

pub fn task_2() {
    let mut ch = Challenge::new();

    let valve_names = ch.valves.keys().cloned().collect::<Vec<_>>();

    for valve_name in valve_names {
        ch.valves.get_mut(&valve_name).unwrap().usefull_paths =
            ch.cost_of_travelling(valve_name.to_string());
    }

    let mut search = Search {
        time: 26,
        helper: true,
        ..Default::default()
    };
    let max_flow = search.bfs(None, &ch.valves);

    println!("Most possible pressure released: {max_flow}");
}
