use std::{cell::RefCell, num::ParseIntError, rc::Rc, str::FromStr};

use crate::utils::get_input_content;

const INPUT_PATH: &str = "inputs/day_7.txt";

const FILE_SYSTEM_SPACE: u64 = 70000000;
const UPDATE_REQUIRED_SPACE: u64 = 30000000;

#[derive(Debug)]
enum Filetype {
    File { file_size: u64, filename: String },
    Folder(String),
}

impl FromStr for Filetype {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match split.next() {
            Some("dir") => Ok(Filetype::Folder(split.next().unwrap().to_string())),
            Some(file_size) => Ok(Filetype::File {
                file_size: file_size.parse::<u64>()?,
                filename: split.next().unwrap().to_string(),
            }),
            _ => panic!("Input error."),
        }
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().skip(1);
        match split.next() {
            Some("cd") => Ok(Command::CD(split.next().unwrap().to_string())),
            Some("ls") => Ok(Command::LS),
            _ => panic!("Input error."),
        }
    }
}

fn is_command(line: &str) -> bool {
    line.starts_with('$')
}

#[derive(Debug)]
struct FileSystemNode {
    filename: String,
    size: u64,
    children: Vec<Rc<RefCell<FileSystemNode>>>,
    parent: Option<Rc<RefCell<FileSystemNode>>>,
}

impl FileSystemNode {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: String::from(filename),
            size: 0,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_node(parent: Rc<RefCell<FileSystemNode>>, child: Rc<RefCell<FileSystemNode>>) {
        child.borrow_mut().set_parent(Rc::clone(&parent));
        parent.borrow_mut().add_child(child);
    }

    pub fn add_child(&mut self, node: Rc<RefCell<FileSystemNode>>) {
        self.children.push(node);
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<FileSystemNode>>) {
        self.parent = Some(Rc::clone(&parent));
    }

    pub fn dirs_with_at_most_size(&self, at_most: u64) -> Vec<u64> {
        let mut result = vec![];
        if !self.children.is_empty() && self.size <= at_most {
            result.push(self.size);
        }

        self.children
            .iter()
            .flat_map(|child| child.borrow().dirs_with_at_most_size(at_most))
            .for_each(|size| result.push(size));

        result
    }

    pub fn dirs_with_at_least_size(&self, at_least: u64) -> Vec<u64> {
        let mut result = vec![];
        if !self.children.is_empty() && self.size >= at_least {
            result.push(self.size);
        }

        self.children
            .iter()
            .flat_map(|child| child.borrow().dirs_with_at_least_size(at_least))
            .for_each(|size| result.push(size));

        result
    }

    pub fn update_folder_size(&mut self, size: u64) {
        self.size += size;
        if let Some(parent) = &self.parent {
            parent.borrow_mut().update_folder_size(size);
        }
    }
}

struct Challenge {
    data: String,
}

impl Challenge {
    pub fn new() -> Self {
        let data = get_input_content(INPUT_PATH);
        Self { data }
    }

    pub fn process_data(&self) {
        let _commands = self
            .data
            .lines()
            .filter_map(|line| {
                if is_command(line) {
                    return line.parse::<Command>().ok();
                }
                None
            })
            .collect::<Vec<Command>>();

        let root = Rc::new(RefCell::new(FileSystemNode::new("/")));
        let current = Rc::clone(&root);

        self.data.lines().skip(1).fold(
            Rc::clone(&current),
            |acc: Rc<RefCell<FileSystemNode>>, line| {
                if is_command(line) {
                    match line.parse::<Command>() {
                        Ok(Command::LS) => {}
                        Ok(Command::CD(dir)) => match dir.as_str() {
                            ".." => match &acc.borrow().parent {
                                None => panic!("This shouldn't happens"),
                                Some(parent) => return Rc::clone(parent),
                            },
                            folder => {
                                return Rc::clone(
                                    acc.borrow()
                                        .children
                                        .iter()
                                        .find(|child| child.borrow().filename == folder)
                                        .unwrap(),
                                );
                            }
                        },
                        _ => {
                            panic!("Error");
                        }
                    }
                } else {
                    match line.parse::<Filetype>() {
                        Ok(Filetype::Folder(folder_name)) => {
                            // Found folder, add it to the parent
                            let new_node = Rc::new(RefCell::new(FileSystemNode::new(&folder_name)));
                            FileSystemNode::add_node(Rc::clone(&acc), Rc::clone(&new_node));

                            return acc;
                        }
                        Ok(Filetype::File {
                            filename,
                            file_size,
                        }) => {
                            let new_node = Rc::new(RefCell::new(FileSystemNode::new(&filename)));
                            FileSystemNode::add_node(Rc::clone(&acc), Rc::clone(&new_node));

                            new_node.borrow_mut().size = file_size;
                            acc.borrow_mut().update_folder_size(file_size);

                            return acc;
                        }
                        _ => {
                            panic!("Error");
                        }
                    }
                }

                acc
            },
        );

        Self::task_1(Rc::clone(&root));
        Self::task_2(Rc::clone(&root));
    }

    fn task_1(root: Rc<RefCell<FileSystemNode>>) {
        let sum = root
            .borrow()
            .dirs_with_at_most_size(100000)
            .iter()
            .sum::<u64>();

        println!("Sum of the folder with at least 100000: {}", sum);
    }

    fn task_2(root: Rc<RefCell<FileSystemNode>>) {
        let current_unused_space = FILE_SYSTEM_SPACE - root.borrow().size;

        let mut folders = root
            .borrow()
            .dirs_with_at_least_size(UPDATE_REQUIRED_SPACE - current_unused_space);

        folders.sort();

        println!("Size of the folder to delete: {}", folders[0]);
    }
}

pub fn task_1() {
    Challenge::new().process_data();
}

pub fn task_2() {}
