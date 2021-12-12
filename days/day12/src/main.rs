use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
enum Cave {
    Start,
    Small(String),
    Big(String),
    End,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            name => {
                if name.chars().all(|c| c.is_ascii_lowercase()) {
                    // All lowercase means it's a small cave
                    Ok(Self::Small(name.to_string()))
                } else if name.chars().all(|c| c.is_ascii_uppercase()) {
                    // All uppercase means it's a big cave
                    Ok(Self::Big(name.to_string()))
                } else {
                    Err(())
                }
            }
        }
    }
}

#[derive(Debug)]
struct CaveNode {
    cave: Cave,
    connections: Vec<Rc<RefCell<CaveNode>>>,
}

impl CaveNode {
    fn new(cave: Cave) -> Self {
        CaveNode {
            cave,
            connections: Vec::new(),
        }
    }

    fn add_connection(&mut self, other_node: Rc<RefCell<CaveNode>>) {
        self.connections.push(other_node);
    }
}

#[derive(Debug)]
struct CaveSystem {
    start: Rc<RefCell<CaveNode>>,
    end: Rc<RefCell<CaveNode>>,
}

type PathList = Vec<Vec<Rc<RefCell<CaveNode>>>>;
impl CaveSystem {
    fn new(start: Rc<RefCell<CaveNode>>, end: Rc<RefCell<CaveNode>>) -> Self {
        CaveSystem { start, end }
    }

    fn find_duplicate(caves: &[String]) -> Option<String> {
        let mut caves_aux = Vec::new();

        for cave in caves {
            if caves_aux.contains(cave) {
                return Some(cave.clone());
            }

            caves_aux.push(cave.clone());
        }

        None
    }

    fn get_all_paths(&self) -> PathList {
        let mut complete_paths_list = PathList::new();

        let mut path_stack = LinkedList::new();
        path_stack.push_back(vec![Rc::clone(&self.start)]);

        while let Some(path) = path_stack.pop_back() {
            // If the path is complete, add it to the list and get the next one
            if let Cave::End = path.last().unwrap().borrow().cave {
                complete_paths_list.push(path.clone());
                continue;
            }

            // List the small caves that were visited by this path
            let small_caves_visited: Vec<String> = path
                .iter()
                .filter_map(|node| match &node.borrow().cave {
                    Cave::Small(name) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            for connection in &path.last().unwrap().borrow().connections {
                // If this is start, skip
                if let Cave::Start = &connection.borrow().cave {
                    continue;
                }

                // If this is a small cave, check if it was visited in this path before
                if let Cave::Small(name) = &connection.borrow().cave {
                    if small_caves_visited.contains(name) {
                        continue;
                    }
                }

                // Extend the path with this node
                let mut extended_path = path.clone();
                extended_path.push(Rc::clone(connection));

                // Push the extended path to the stack
                path_stack.push_back(extended_path);
            }
        }

        complete_paths_list
    }

    fn get_all_paths_extra_time(&self) -> PathList {
        let mut complete_paths_list = PathList::new();

        let mut path_stack = LinkedList::new();
        path_stack.push_back(vec![Rc::clone(&self.start)]);

        while let Some(path) = path_stack.pop_back() {
            // If the path is complete, add it to the list and get the next one
            if let Cave::End = path.last().unwrap().borrow().cave {
                complete_paths_list.push(path.clone());
                continue;
            }

            // List the small caves that were visited by this path
            let small_caves_visited: Vec<String> = path
                .iter()
                .filter_map(|node| match &node.borrow().cave {
                    Cave::Small(name) => Some(name.clone()),
                    _ => None,
                })
                .collect();

            // Check if we previously visited a small cave twice
            let special_small_cave = Self::find_duplicate(&small_caves_visited);

            for connection in &path.last().unwrap().borrow().connections {
                // If this is start, skip
                if let Cave::Start = &connection.borrow().cave {
                    continue;
                }

                // If this is a small cave, check if it was visited in this path before
                if let Cave::Small(name) = &connection.borrow().cave {
                    if small_caves_visited.contains(name) {
                        // If we alredy visited a small cave twice, skip
                        if special_small_cave.is_some() {
                            continue;
                        }
                    }
                }

                // Extend the path with this node
                let mut extended_path = path.clone();
                extended_path.push(Rc::clone(connection));

                // Push the extended path to the stack
                path_stack.push_back(extended_path);
            }
        }

        complete_paths_list
    }
}

fn parse_input<T>(filename: T) -> io::Result<CaveSystem>
where
    T: AsRef<Path>,
{
    let mut node_map = HashMap::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Split by '-' and take two caves
        let caves: Vec<String> = line
            .trim()
            .split('-')
            .map(|s| s.to_string())
            .take(2)
            .collect();

        if caves.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid number of fields: {}", line),
            ));
        }

        let cave1_str = &caves[0];
        let cave2_str = &caves[1];

        let cave1 = Cave::from_str(cave1_str).unwrap();
        let cave2 = Cave::from_str(cave2_str).unwrap();

        let cave_node1;
        let cave_node2;
        {
            let cave_entry1 = node_map
                .entry(cave1_str.to_string())
                .or_insert_with(|| Rc::new(RefCell::new(CaveNode::new(cave1))));
            cave_node1 = Rc::clone(cave_entry1);
        }
        {
            let cave_entry2 = node_map
                .entry(cave2_str.to_string())
                .or_insert_with(|| Rc::new(RefCell::new(CaveNode::new(cave2))));
            cave_node2 = Rc::clone(cave_entry2);
        }

        cave_node1
            .borrow_mut()
            .add_connection(Rc::clone(&cave_node2));
        cave_node2
            .borrow_mut()
            .add_connection(Rc::clone(&cave_node1));
    }

    let start_node = Rc::clone(node_map.get("start").unwrap());
    let end_node = Rc::clone(node_map.get("end").unwrap());

    Ok(CaveSystem::new(start_node, end_node))
}

fn part1(cave_system: &CaveSystem) -> usize {
    cave_system.get_all_paths().len()
}

fn part2(cave_system: &CaveSystem) -> usize {
    cave_system.get_all_paths_extra_time().len()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let cave_system = parse_input("inputs/day12")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let npaths_part1 = part1(&cave_system);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let npaths_part2 = part2(&cave_system);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nNumber of paths: {}\n",
        part1_time, npaths_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nNumber of paths: {}\n",
        part2_time, npaths_part2
    );

    Ok(())
}
