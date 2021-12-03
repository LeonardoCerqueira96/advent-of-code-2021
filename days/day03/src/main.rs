use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;


// Returns a vector, where each position holds the 1's count of that column, and the number of lines
fn parse_input<T>(filename: T) -> io::Result<(Vec<usize>, usize)>
where
    T: AsRef<Path>,
{
    let mut ones_count = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    let mut line_count = 0;
    for line_result in input_buf.lines() {
        line_count += 1;

        let line = line_result?;

        // Allocate the vector if its empty
        if ones_count.is_empty() {
            ones_count = vec![0; line.len()];
        }

        // Count the number of 1's
        for (i, digit) in line.chars().enumerate() {
            match digit {
                '0' => continue,
                '1' => ones_count[i] += 1,
                c => println!("WARNING: Invalid character {}", c)
            }
        }
    }

    Ok((ones_count, line_count))
}

fn part1(ones_count: &[usize], line_count: usize) -> (u64, u64) {
    // Map vector of 1's counts into a binary string
    let gamma_str: String = ones_count
        .iter()
        .map(|c| {
            if c * 2 < line_count {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    // Convert binary string to u64
    let gamma = u64::from_str_radix(&gamma_str, 2).unwrap();
    
    // Epsilon the inverse binary of gamma
    // The mask is there to negate only 12 bits
    let epsilon = (!gamma) & 0x0000000000000FFF;

    (gamma, epsilon)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (ones_count, line_count) = parse_input("inputs/day03")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let (gamma, epsilon) = part1(&ones_count, line_count);
    let part1_time = t1.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nGamma: {}\nEpsilon: {}\nMultiplied: {}\n",
        part1_time,
        gamma,
        epsilon,
        gamma * epsilon
    );

    Ok(())
}
