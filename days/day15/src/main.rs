use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    position: (usize, usize),        // Position in the cave
    total_risk: usize,               // Total risk cost to this node
    distance: usize,                 // Manhattan distance to the goal
    came_from: Option<Rc<PathNode>>, // Which node this one was measured from
}

impl PathNode {
    fn new(
        position: (usize, usize),
        total_risk: usize,
        distance: usize,
        came_from: Option<Rc<PathNode>>,
    ) -> Self {
        PathNode {
            position,
            total_risk,
            distance,
            came_from,
        }
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_total_cost = self.total_risk + self.distance;
        let other_total_cost = other.total_risk + other.distance;

        // Reverse because a lower cost means higher priority
        self_total_cost.cmp(&other_total_cost).reverse()
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Cave {
    nrows: usize,
    ncols: usize,
    risk_costs: Vec<Vec<u8>>,
}

impl Cave {
    fn new(risk_costs: Vec<Vec<u8>>) -> Self {
        let nrows = risk_costs.len();
        let ncols = risk_costs[0].len();

        Cave {
            nrows,
            ncols,
            risk_costs,
        }
    }

    // Calculates the Manhattan distance between two points
    fn distance_between(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        let y_distance = (pos2.0 as isize - pos1.0 as isize).abs() as usize;
        let x_distance = (pos2.1 as isize - pos1.1 as isize).abs() as usize;

        x_distance + y_distance
    }

    fn extend_tiles(&mut self, x_tiles: usize, y_tiles: usize) {
        // Extend rows first
        for row in &mut self.risk_costs {
            for i in 1..x_tiles {
                let new_tile_row: Vec<u8> = row[0..self.ncols]
                    .iter()
                    .map(|v| {
                        let new_v = *v + i as u8;
                        if new_v > 9 {
                            new_v % 9
                        } else {
                            new_v
                        }
                    })
                    .collect();
                row.extend_from_slice(&new_tile_row);
            }
        }

        // Extend columns
        let mut new_full_rows = Vec::new();
        for i in 1..y_tiles {
            for row in &mut self.risk_costs {
                let new_full_row: Vec<u8> = row
                    .iter()
                    .map(|v| {
                        let new_v = *v + i as u8;
                        if new_v > 9 {
                            new_v % 9
                        } else {
                            new_v
                        }
                    })
                    .collect();
                new_full_rows.push(new_full_row);
            }
        }

        self.risk_costs.extend_from_slice(&new_full_rows);
        self.nrows *= y_tiles;
        self.ncols *= x_tiles;
    }

    fn iter_adjacents(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut adjacent_list = Vec::new();

        // Check position above
        if pos.0 > 0 {
            adjacent_list.push((pos.0 - 1, pos.1));
        }

        // Check position below
        if pos.0 < self.nrows - 1 {
            adjacent_list.push((pos.0 + 1, pos.1));
        }

        // Check position to the left
        if pos.1 > 0 {
            adjacent_list.push((pos.0, pos.1 - 1));
        }

        // Check position to the right
        if pos.1 < self.ncols - 1 {
            adjacent_list.push((pos.0, pos.1 + 1));
        }

        adjacent_list.into_iter()
    }

    // Implements the A* algorithm to find the shortest path
    fn find_shortest_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        let mut node_queue = BinaryHeap::new();

        let start_path_node = PathNode::new(start, 0, Self::distance_between(start, goal), None);
        node_queue.push(Rc::new(start_path_node));

        // Keep track of visited nodes so we don't cycle
        let mut visited_nodes = HashSet::new();

        let mut final_path_node = None;
        while let Some(node) = node_queue.pop() {
            visited_nodes.insert(node.position);

            // Check if we reached our goal
            if node.position == goal {
                final_path_node = Some(node);
                break;
            }

            // Add all adjacent nodes to the queue
            for adj_pos in self.iter_adjacents(node.position) {
                if visited_nodes.contains(&adj_pos) {
                    continue;
                }

                let new_node = PathNode::new(
                    adj_pos,
                    node.total_risk + self.risk_costs[adj_pos.0][adj_pos.1] as usize,
                    Self::distance_between(adj_pos, goal),
                    Some(Rc::clone(&node)),
                );

                node_queue.push(Rc::new(new_node));
            }
        }

        let mut path_node = &final_path_node?;

        // Save total risk cost of the path
        let path_risk_cost = path_node.total_risk;

        // Rebuild path
        let mut shortest_path = VecDeque::new();
        shortest_path.push_front(path_node.position);
        while let Some(previous_node) = &path_node.came_from {
            shortest_path.push_front(previous_node.position);
            path_node = previous_node;
        }

        Some((Vec::from(shortest_path), path_risk_cost))
    }

    fn find_shortest_path_corners(&self) -> Option<(Vec<(usize, usize)>, usize)> {
        let upper_left_corner = (0, 0);
        let lower_right_corner = (self.nrows - 1, self.ncols - 1);

        self.find_shortest_path(upper_left_corner, lower_right_corner)
    }
}

fn parse_input<T>(filename: T) -> io::Result<Cave>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let risk_costs: io::Result<Vec<Vec<u8>>> = input_buf
        .lines()
        .map(|row| {
            row?.chars()
                .map(|lvl| {
                    lvl.to_digit(10)
                        .ok_or_else(|| {
                            io::Error::new(io::ErrorKind::Other, format!("Invalid digit {}", lvl))
                        })
                        .map(|d| d as u8)
                })
                .collect()
        })
        .collect();

    Ok(Cave::new(risk_costs?))
}

fn part1(cave: &Cave) -> usize {
    if let Some((_path, total_risk_cost)) = cave.find_shortest_path_corners() {
        total_risk_cost
    } else {
        // Should never happen
        panic!("No path was found!")
    }
}

fn part2(cave: &mut Cave) -> usize {
    cave.extend_tiles(5, 5);

    if let Some((_path, total_risk_cost)) = cave.find_shortest_path_corners() {
        total_risk_cost
    } else {
        // Should never happen
        panic!("No path was found!")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let mut cave = parse_input("inputs/day15")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let total_risk_par1 = part1(&cave);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let total_risk_par2 = part2(&mut cave);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nPath total risk cost: {}\n",
        part1_time, total_risk_par1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nPath total risk cost: {}\n",
        part2_time, total_risk_par2
    );

    Ok(())
}
