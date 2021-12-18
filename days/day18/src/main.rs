use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::{Add, AddAssign};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct SnailFishPart {
    value: usize,
    depth: usize,
}

impl SnailFishPart {
    fn new(value: usize, depth: usize) -> Self {
        Self { value, depth }
    }
}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    parts: Vec<SnailFishPart>,
}

impl FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = Vec::new();
        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                num if num.is_digit(10) => parts.push(SnailFishPart::new(
                    num.to_digit(10)
                        .ok_or(format!("Unable to convert digit: {}", num))?
                        as usize,
                    depth,
                )),
                _ => (),
            }
        }

        Ok(Self { parts })
    }
}

impl AddAssign for SnailfishNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.parts.extend(rhs.parts);
        for part in self.parts.iter_mut() {
            part.depth += 1;
        }

        self.reduce();
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_num = self;
        new_num += rhs;
        new_num
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first_num = iter.next().unwrap();
        let sum = iter.fold(first_num, |acc, sfn| acc + sfn);

        sum
    }
}

impl SnailfishNumber {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        for (
            i,
            (
                &SnailFishPart {
                    value: value1,
                    depth: depth1,
                },
                &SnailFishPart {
                    value: value2,
                    depth: depth2,
                },
            ),
        ) in self.parts.iter().tuple_windows().enumerate()
        {
            if depth1 == 5 && depth2 == 5 {
                if self.parts.get(i.saturating_sub(1)).is_some() && i.saturating_sub(1) != i {
                    self.parts.get_mut(i - 1).unwrap().value += value1;
                }

                if self.parts.get(i + 2).is_some() {
                    self.parts.get_mut(i + 2).unwrap().value += value2;
                }

                self.parts.drain(i..i + 2);
                self.parts.insert(i, SnailFishPart::new(0, 4));
                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for (i, part) in self.parts.iter().enumerate() {
            if part.value > 9 {
                let (value, depth) = (part.value, part.depth);
                self.parts.remove(i);
                self.parts
                    .insert(i, SnailFishPart::new(value / 2, depth + 1));
                self.parts
                    .insert(i + 1, SnailFishPart::new((value + 1) / 2, depth + 1));

                return true;
            }
        }

        false
    }

    fn magnitude(&self) -> usize {
        let mut mag = self.clone();
        for depth in (1..=4).rev() {
            while mag.magnitude_rec(depth) {}
        }

        mag.parts[0].value
    }

    fn magnitude_rec(&mut self, depth: usize) -> bool {
        for (i, (part1, part2)) in self.parts.iter().tuple_windows().enumerate() {
            if part1.depth == depth && part2.depth == depth {
                self.parts[i] = SnailFishPart::new(3 * part1.value + 2 * part2.value, depth - 1);
                self.parts.remove(i + 1);
                return true;
            }
        }

        false
    }
}

fn parse_input<T>(filename: T) -> io::Result<Vec<SnailfishNumber>>
where
    T: AsRef<Path>,
{
    let mut numbers = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    for line_result in input_buf.lines() {
        let line = line_result?;

        let number = SnailfishNumber::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        numbers.push(number);
    }

    Ok(numbers)
}

fn part1(numbers: Vec<SnailfishNumber>) -> usize {
    let snailfish_sum: SnailfishNumber = numbers.into_iter().sum();

    snailfish_sum.magnitude()
}

fn part2(numbers: Vec<SnailfishNumber>) -> usize {
    numbers
        .into_iter()
        .permutations(2)
        .map(|perm| perm.into_iter().sum::<SnailfishNumber>().magnitude())
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let numbers = parse_input("inputs/day18")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let snailfish_sum_mag = part1(numbers.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let snailfish_perm_max_mag = part2(numbers);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nMagnitude of sum: {}\n",
        part1_time, snailfish_sum_mag
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nMagnitude of max permutation sum: {}\n",
        part2_time, snailfish_perm_max_mag
    );

    Ok(())
}
