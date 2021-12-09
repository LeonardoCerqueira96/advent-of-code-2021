use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

type DigitalPatterns = (Vec<Vec<String>>, Vec<Vec<String>>);
fn parse_input<T>(filename: T) -> io::Result<DigitalPatterns>
where
    T: AsRef<Path>,
{
    let mut all_signal_patterns = Vec::new();
    let mut all_output_digits = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Split by | and take two fields
        let fields: Vec<&str> = line.trim().split('|').take(2).collect();

        if fields.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid number of fields",
            ));
        }

        let (signal_patterns_str, output_digits_str) = (fields[0].trim(), fields[1].trim());
        // Split by whitespace
        let signal_patterns: Vec<String> = signal_patterns_str
            .split_ascii_whitespace()
            .map(|p| p.chars().sorted().collect())
            .collect();
        let output_digits: Vec<String> = output_digits_str
            .split_ascii_whitespace()
            .map(|p| p.chars().sorted().collect())
            .collect();

        all_signal_patterns.push(signal_patterns);
        all_output_digits.push(output_digits);
    }

    Ok((all_signal_patterns, all_output_digits))
}

fn get_pattern_difference(pattern1: &str, pattern2: &str) -> Vec<char> {
    let pattern1_set: HashSet<char> = HashSet::from_iter(pattern1.chars());
    let pattern2_set: HashSet<char> = HashSet::from_iter(pattern2.chars());

    pattern1_set
        .difference(&pattern2_set)
        .map(|p| p.to_owned())
        .collect()
}

type PatternTranslator = Box<dyn Fn(&str) -> char>;
fn get_translator(signal_patterns: &[String]) -> PatternTranslator {
    let mut signal_map = HashMap::new();
    let mut translator_map = HashMap::new();

    // We know which pattern is digit 1 because of the fixed length
    let digit_1_pattern = signal_patterns
        .iter()
        .find(|&s| s.len() == 2)
        .unwrap()
        .to_string();

    // We know which pattern is digit 4 because of the fixed length
    let digit_4_pattern = signal_patterns
        .iter()
        .find(|&s| s.len() == 4)
        .unwrap()
        .to_string();

    // We know which pattern is digit 7 because of the fixed length
    let digit_7_pattern = signal_patterns
        .iter()
        .find(|&s| s.len() == 3)
        .unwrap()
        .to_string();

    // We know which pattern is digit 8 because of the fixed length
    let digit_8_pattern = signal_patterns
        .iter()
        .find(|&s| s.len() == 7)
        .unwrap()
        .to_string();

    // Get all patterns with 6 segments (0, 6 and 9)
    let mut patterns_6_segments: Vec<String> = signal_patterns
        .iter()
        .filter(|&s| s.len() == 6)
        .map(|s| s.to_string())
        .collect();

    // Get all patterns with 5 segments (2, 3 and 5)
    let patterns_5_segments = signal_patterns
        .iter()
        .filter(|&s| s.len() == 5)
        .map(|s| s.to_string());

    // The segment corresponding to signal `a` can be deduced by the difference between digits 1 and 7
    let segment_signal_a = get_pattern_difference(&digit_7_pattern, &digit_1_pattern)[0];
    signal_map.insert(segment_signal_a, 'a');

    // The segment corresponding to signal `a` can be deduced by the difference between digits 1 and 6
    let (digit_6_index, digit_6_pattern) = patterns_6_segments
        .iter()
        .enumerate()
        .filter(|(_i, p)| {
            let diff = get_pattern_difference(&digit_1_pattern, p);

            // Found the segment corresponding to signal `c`!
            if diff.len() == 1 {
                signal_map.insert(diff[0], 'c');
                true
            } else {
                false
            }
        })
        .map(|(i, p)| (i, p.to_string()))
        .collect::<Vec<(usize, String)>>()
        .remove(0);
    patterns_6_segments.remove(digit_6_index);

    // The segment corresponding to signal `a` can be deduced by the difference between digits 4 and 0
    let (digit_0_index, digit_0_pattern) = patterns_6_segments
        .iter()
        .enumerate()
        .filter(|(_i, p)| {
            let diff = get_pattern_difference(&digit_4_pattern, p);

            // Found the segment corresponding to signal `d`!
            if diff.len() == 1 {
                signal_map.insert(diff[0], 'd');
                true
            } else {
                false
            }
        })
        .map(|(i, p)| (i, p.to_string()))
        .collect::<Vec<(usize, String)>>()
        .remove(0);
    patterns_6_segments.remove(digit_0_index);

    // 9 is the only 6 segment digit left
    let digit_9_pattern = patterns_6_segments.pop().unwrap();

    // The segment corresponding to signal `e` can be deduced by the difference between digits 8 and 9
    let segment_signal_e = get_pattern_difference(&digit_8_pattern, &digit_9_pattern)[0];
    signal_map.insert(segment_signal_e, 'e');

    // Determine what the patterns for 2, 3 and 5 are by the numbers of unmmapped signals we ge on the difference with pattern 8
    let mut patterns_5_segments_ord: Vec<String> = patterns_5_segments
        .sorted_by_key(|p| {
            let diff = get_pattern_difference(&digit_8_pattern, p);
            diff.into_iter()
                .filter(|c| !signal_map.contains_key(c))
                .count()
        })
        .collect();

    let digit_2_pattern = patterns_5_segments_ord.pop().unwrap();
    let digit_3_pattern = patterns_5_segments_ord.pop().unwrap();
    let digit_5_pattern = patterns_5_segments_ord.pop().unwrap();

    // The segment corresponding to signal `f` can be deduced by the difference between digits 1 and 2
    let segment_signal_f = get_pattern_difference(&digit_1_pattern, &digit_2_pattern)[0];
    signal_map.insert(segment_signal_f, 'f');

    // The segment corresponding to signal `b` can be deduced by the difference between digits 5 and 3
    let segment_signal_b = get_pattern_difference(&digit_5_pattern, &digit_3_pattern)[0];
    signal_map.insert(segment_signal_b, 'b');

    // The segment corresponding to signal `g` is the only one left
    let segment_signal_g = digit_8_pattern
        .chars()
        .find(|c| !signal_map.contains_key(c))
        .unwrap();
    signal_map.insert(segment_signal_g, 'g');

    translator_map.insert(digit_0_pattern, '0');
    translator_map.insert(digit_1_pattern, '1');
    translator_map.insert(digit_2_pattern, '2');
    translator_map.insert(digit_3_pattern, '3');
    translator_map.insert(digit_4_pattern, '4');
    translator_map.insert(digit_5_pattern, '5');
    translator_map.insert(digit_6_pattern, '6');
    translator_map.insert(digit_7_pattern, '7');
    translator_map.insert(digit_8_pattern, '8');
    translator_map.insert(digit_9_pattern, '9');

    Box::new(move |pattern| *translator_map.get(pattern).unwrap())
}

fn part1(all_output_digits: &[Vec<String>]) -> usize {
    all_output_digits.iter().fold(0, |acc, output_digits| {
        // Calculate number of 1, 4, 7 and 8 digits
        let num_1478_digits = output_digits
            .iter()
            .filter(|&p| {
                p.len() == 2    // Digit 1
                    || p.len() == 4 // Digit 4
                    || p.len() == 3 // Digit 7
                    || p.len() == 7 // Digit 8
            })
            .count();

        acc + num_1478_digits
    })
}

fn part2(all_signal_patterns: &[Vec<String>], all_output_digits: &[Vec<String>]) -> usize {
    all_signal_patterns
        .iter()
        .zip(all_output_digits)
        .fold(0, |acc, (patterns, numbers)| {
            let translate = get_translator(patterns);
            let number = numbers
                .iter()
                .map(|p| translate(p))
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            acc + number
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (all_signal_patterns, all_output_digits) = parse_input("inputs/day08")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let num_1478_digits = part1(&all_output_digits);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let numbers_sum = part2(&all_signal_patterns, &all_output_digits);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nNumber of 1, 4, 7 and 8 digits: {}\n",
        part1_time, num_1478_digits
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nSum of all the numbers: {}\n",
        part2_time, numbers_sum
    );

    Ok(())
}
