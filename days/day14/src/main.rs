use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Polymerizer {
    polymer_pairs: HashMap<String, usize>,
    element_frequency: HashMap<char, usize>,
    rules: HashMap<String, char>,
}

impl Polymerizer {
    fn new(initial_polymer: String, raw_rules: Vec<(String, char)>) -> Self {
        let polymer_pairs = initial_polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(HashMap::new(), |mut hm, p| {
            let pair = String::from_iter(p);
            *hm.entry(pair).or_insert(0) += 1;
            hm
        });
        
        let element_frequency = initial_polymer.chars().fold(HashMap::new(), |mut hm, c| {
            *hm.entry(c).or_insert(0) += 1;
            hm
        });

        let rules = raw_rules.into_iter().fold(HashMap::new(), |mut hm, r| {
            hm.insert(r.0, r.1);
            hm
        });

        Polymerizer { polymer_pairs, element_frequency, rules }
    }

    fn step(&mut self) {
        let mut new_pairs = self.polymer_pairs.clone();

        for (pair, count) in &self.polymer_pairs {
            if *count == 0 {
                continue;
            }

            // Separate the elements of each pair
            let first_elem = pair.chars().nth(0).unwrap();
            let second_elem = pair.chars().nth(1).unwrap();

            // Get the new element from the rule table
            let new_elem = *self.rules.get(pair).unwrap();

            // Update the frequency map
            *self.element_frequency.entry(new_elem).or_insert(0) += count;

            // Empty the count of this pair
            *new_pairs.entry(pair.to_string()).or_insert(*count) -= count;

            // Increase the count of the two new pairs
            let new_pair_left = String::from_iter([first_elem, new_elem]);
            let new_pair_right = String::from_iter([new_elem, second_elem]);
            *new_pairs.entry(new_pair_left).or_insert(0) += count;
            *new_pairs.entry(new_pair_right).or_insert(0) += count;
        }

        self.polymer_pairs = new_pairs;
    }
}

fn parse_input<T>(filename: T) -> io::Result<Polymerizer>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);
    let mut lines_iter = input_buf.lines();

    // First line is the initial polymer
    let polymer = lines_iter.next().unwrap()?;

    // Read the rules
    let raw_rules = lines_iter
        .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
        .map(|r| {
            let r = r.unwrap();
            let fields: Vec<&str> = r.split("->").map(|f| f.trim()).take(2).collect();
            let left_side = fields[0].to_string();
            let right_side = fields[1].chars().nth(0).unwrap();

            (left_side, right_side)
        })
        .collect();

    Ok(Polymerizer::new(polymer, raw_rules))
}

fn part1(mut polymerizer: Polymerizer) -> usize {
    for _ in 0..10 {
        polymerizer.step();
    }

    let max_freq = polymerizer.element_frequency.iter().max_by_key(|f| f.1).unwrap();
    let min_freq = polymerizer.element_frequency.iter().min_by_key(|f| f.1).unwrap();

    max_freq.1 - min_freq.1
}

fn part2(mut polymerizer: Polymerizer) -> usize {
    for _ in 0..40 {
        polymerizer.step();
    }

    let max_freq = polymerizer.element_frequency.iter().max_by_key(|f| f.1).unwrap();
    let min_freq = polymerizer.element_frequency.iter().min_by_key(|f| f.1).unwrap();

    max_freq.1 - min_freq.1
}

fn main() -> Result<(), Box<dyn Error>>{
    // Parse the input and time it
    let t0 = Instant::now();
    let polymerizer = parse_input("inputs/day14")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let part1_result = part1(polymerizer.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let part2_result = part2(polymerizer);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nMost frequent - less frequent after 10 steps: {}\n",
        part1_time, part1_result
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nMost frequent - less frequent after 40 steps: {}\n",
        part2_time, part2_result
    );

    Ok(())
}
