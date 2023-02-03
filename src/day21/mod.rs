use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    str::FromStr,
};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_21.txt";

#[derive(Debug, Clone)]
enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum NodeValue {
    Value(isize),
    Op(Ops),
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    value: NodeValue,
    result: isize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn add_node(parent: Rc<RefCell<Node>>, child: Rc<RefCell<Node>>, left: bool) {
        child.borrow_mut().set_parent(Rc::clone(&parent));
        if left {
            parent.borrow_mut().add_left_child(child);
        } else {
            parent.borrow_mut().add_right_child(child);
        }
    }

    pub fn add_left_child(&mut self, node: Rc<RefCell<Node>>) {
        self.left = Some(node);
    }

    pub fn add_right_child(&mut self, node: Rc<RefCell<Node>>) {
        self.right = Some(node);
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Node>>) {
        self.parent = Some(Rc::clone(&parent));
    }

    pub fn get_sibling(&self) -> Rc<RefCell<Node>> {
        match self
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .id
            == self.id
        {
            true => Rc::clone(
                self.parent
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .right
                    .as_ref()
                    .unwrap(),
            ),
            false => Rc::clone(
                self.parent
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .left
                    .as_ref()
                    .unwrap(),
            ),
        }
    }

    pub fn is_left(&self) -> bool {
        self.parent
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .id
            == self.id
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: Default::default(),
            value: NodeValue::Value(0),
            result: Default::default(),
            left: Default::default(),
            right: Default::default(),
            parent: Default::default(),
        }
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: Rc<RefCell<Node>>,
}

impl BinaryTree {
    pub fn reduce(&self) -> isize {
        Self::process(&self.root)
    }

    fn process(n: &Rc<RefCell<Node>>) -> isize {
        let result = match &n.borrow().value {
            NodeValue::Value(v) => *v,
            NodeValue::Op(op) => match op {
                Ops::Add => {
                    Self::process(n.borrow().left.as_ref().unwrap())
                        + Self::process(n.borrow().right.as_ref().unwrap())
                }
                Ops::Sub => {
                    Self::process(n.borrow().left.as_ref().unwrap())
                        - Self::process(n.borrow().right.as_ref().unwrap())
                }
                Ops::Mul => {
                    Self::process(n.borrow().left.as_ref().unwrap())
                        * Self::process(n.borrow().right.as_ref().unwrap())
                }
                Ops::Div => {
                    Self::process(n.borrow().left.as_ref().unwrap())
                        / Self::process(n.borrow().right.as_ref().unwrap())
                }
            },
        };

        n.borrow_mut().result = result;
        result
    }

    pub fn reduce_to_root(&self, root_val: isize) -> isize {
        let humn = self.find_by_id("humn").unwrap();
        Self::process_to_root(&humn, root_val)
    }

    fn process_to_root(n: &Rc<RefCell<Node>>, root_val: isize) -> isize {
        match &n.borrow().parent.as_ref().is_some() {
            true => {
                if &n.borrow().parent.as_ref().unwrap().borrow().id == "root" {
                    return root_val;
                }

                let parent_v = Self::process_to_root(n.borrow().parent.as_ref().unwrap(), root_val);
                let sibling_v = n.borrow().get_sibling().borrow().result;

                match &n.borrow().parent.as_ref().unwrap().borrow().value {
                    NodeValue::Value(_) => unreachable!("All parents has op"),
                    NodeValue::Op(op) => match op {
                        Ops::Add => parent_v - sibling_v,
                        Ops::Sub => {
                            if n.borrow().is_left() {
                                parent_v + sibling_v
                            } else {
                                -(parent_v - sibling_v)
                            }
                        }
                        Ops::Mul => parent_v / sibling_v,
                        Ops::Div => {
                            if n.borrow().is_left() {
                                parent_v * sibling_v
                            } else {
                                sibling_v / parent_v
                            }
                        }
                    },
                }
            }
            false => root_val,
        }
    }

    pub fn find_by_id(&self, node_id: &str) -> Option<Rc<RefCell<Node>>> {
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&self.root));

        while let Some(node) = queue.pop_front() {
            if node.borrow().id == node_id {
                return Some(Rc::clone(&node));
            }

            if let Some(left) = &node.borrow().left {
                queue.push_back(Rc::clone(left));
            }

            if let Some(right) = &node.borrow().right {
                queue.push_back(Rc::clone(right));
            }
        }

        None
    }
}

impl FromStr for BinaryTree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .collect::<HashMap<_, _>>();

        fn parse_node(node: &(&str, &str), nodes: &HashMap<&str, &str>) -> Rc<RefCell<Node>> {
            match node.1.contains(' ') {
                true => {
                    let mut parts = node.1.split_whitespace();

                    let left_id = parts.next().unwrap();
                    let left = parse_node(&(left_id, *nodes.get(left_id).unwrap()), nodes);

                    let op = match parts.next().unwrap() {
                        "+" => Ops::Add,
                        "-" => Ops::Sub,
                        "*" => Ops::Mul,
                        "/" => Ops::Div,
                        _ => unreachable!("This shouldn't happen"),
                    };

                    let right_id = parts.next().unwrap();
                    let right = parse_node(&(right_id, *nodes.get(right_id).unwrap()), nodes);

                    let node = Node {
                        id: node.0.to_string(),
                        value: NodeValue::Op(op),
                        ..Default::default()
                    };
                    let node_ref = Rc::new(RefCell::new(node));

                    Node::add_node(Rc::clone(&node_ref), left, true);
                    Node::add_node(Rc::clone(&node_ref), right, false);

                    node_ref
                }
                false => Rc::new(RefCell::new(Node {
                    id: node.0.to_string(),
                    value: NodeValue::Value(node.1.parse::<isize>().unwrap()),
                    ..Default::default()
                })),
            }
        }

        Ok(Self {
            root: parse_node(&("root", nodes.get("root").unwrap()), &nodes),
        })
    }
}

struct Challenge {
    tree: BinaryTree,
}

impl Challenge {
    pub fn new() -> Self {
        let input = get_input_content(INPUT_PATH);
        Self {
            tree: input.parse::<BinaryTree>().unwrap(),
        }
    }
}

pub fn task_1() {
    let ch = Challenge::new();
    println!("Root monkey yells {}", ch.tree.reduce());
}

pub fn task_2() {
    let ch = Challenge::new();

    ch.tree.reduce();

    // TODO:
    // This works for my test input but I need to update the code to find the right branch.
    // The branch that doesn't contains "humn"

    let v = ch
        .tree
        .root
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow()
        .result;

    println!("I (humn) should yell: {}", ch.tree.reduce_to_root(v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_test() {
        let ch = Challenge::new();
        assert_eq!(87457751482938, ch.tree.reduce())
    }
}
