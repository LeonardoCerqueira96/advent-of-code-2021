use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn parse_input<T>(filename: T) -> io::Result<Vec<u32>>
where
    T: AsRef<Path>,
{
    let mut depths = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Parse string to u32
        // If an error occurs, map the ParseIntError to an IO error and return it
        let depth = line
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        depths.push(depth);
    }

    Ok(depths)
}

fn part1(depths: &[u32]) -> u32 {
    let mut n_increases = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            n_increases += 1
        }
    }

    n_increases
}

fn part2(depths: &[u32]) -> u32 {
    let mut n_increases = 0;
    let window_size = 3;

    let mut window_iter = depths.windows(window_size);
    let mut prev_window = window_iter.next().unwrap();
    while let Some(window) = window_iter.next() {
        let prev_sum: u32 = prev_window.iter().sum();
        let next_sum: u32 = window.iter().sum();

        if next_sum > prev_sum {
            n_increases += 1;
        }

        prev_window = window;
    }

    n_increases
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let depths = parse_input("inputs/day01")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let n_increases_part1 = part1(&depths);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let n_increases_part2 = part2(&depths);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!("Part 1:\nTook {}s\nDepth increased {} times\n", part1_time, n_increases_part1);

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!("Part 2:\nTook {}s\nDepth increased {} times\n", part2_time, n_increases_part2);

    Ok(())
}
