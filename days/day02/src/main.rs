use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
struct Submarine {
    depth: u64,
    horizontal_position: u64,
}

#[derive(Debug)]
enum SubMovement {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            depth: 0,
            horizontal_position: 0,
        }
    }

    fn maneuver(&mut self, movement: &SubMovement) {
        match movement {
            SubMovement::Forward(d) => self.horizontal_position += d,
            SubMovement::Up(d) => self.depth = self.depth.saturating_sub(*d),
            SubMovement::Down(d) => self.depth += d,
        }
    }
}

fn parse_input<T>(filename: T) -> io::Result<Vec<SubMovement>>
where
    T: AsRef<Path>,
{
    let mut movements = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;
        // Split line into its fields
        let fields: Vec<&str> = line.split(' ').collect();

        if fields.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid number of fields",
            ));
        }

        // Parse the offset part of the movement
        let offset = fields[1]
            .parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Parse the direction part of the movement
        let movement = match fields[0] {
            "forward" => SubMovement::Forward(offset),
            "up" => SubMovement::Up(offset),
            "down" => SubMovement::Down(offset),
            f => panic!("Invalid input {}", f),
        };

        movements.push(movement);
    }

    Ok(movements)
}

fn part1(sub: &mut Submarine, movements: &[SubMovement]) {
    for movement in movements {
        sub.maneuver(movement);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Instantiate our submarine
    let mut sub = Submarine::new();

    // Parse the input and time it
    let t0 = Instant::now();
    let movements = parse_input("inputs/day02")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    part1(&mut sub, &movements);
    let part1_time = t1.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nSub position: {:?}\nMultiplied: {}\n",
        part1_time,
        sub,
        sub.depth * sub.horizontal_position
    );

    Ok(())
}
