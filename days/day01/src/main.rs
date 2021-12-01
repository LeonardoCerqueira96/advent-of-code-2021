use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

fn main() -> Result<(), Box<dyn Error>> {
    let depths = parse_input("inputs/day01")?;

    let mut n_increases = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            n_increases += 1
        }
    }

    println!("Depth increased {} times", n_increases);
    Ok(())
}
