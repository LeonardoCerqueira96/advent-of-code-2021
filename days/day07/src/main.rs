use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn parse_input<T>(filename: T) -> io::Result<Vec<usize>>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // There's only one line in the file
    let positions_str = match input_buf.lines().next() {
        Some(line_result) => line_result?,
        None => return Err(io::Error::new(io::ErrorKind::Other, "File is empty")),
    };

    // Parse cycles
    let positions: Vec<usize> = positions_str
        .split(',')
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    Ok(positions)
}

fn calc_fuel_constant(positions: &[usize], final_position: usize) -> usize {
    positions.iter().fold(0, |acc, pos| {
        let diff = (*pos as isize) - (final_position as isize);
        acc + (diff.abs() as usize)
    })
}

fn calc_fuel_variable(positions: &[usize], final_position: usize) -> usize {
    positions.iter().fold(0, |acc, pos| {
        let diff = (*pos as isize) - (final_position as isize);
        let fuel_cost = (1..=(diff.abs() as usize)).sum::<usize>();
        acc + fuel_cost
    })
}

fn part1(positions: &[usize]) -> (usize, usize) {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    (min_pos..=max_pos)
        .map(|pos| (pos, calc_fuel_constant(positions, pos)))
        .min_by_key(|&(_pos, fuel)| fuel)
        .unwrap()
}

fn part2(positions: &[usize]) -> (usize, usize) {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    (min_pos..=max_pos)
        .into_iter()
        .map(|pos| (pos, calc_fuel_variable(positions, pos)))
        .min_by_key(|&(_pos, fuel)| fuel)
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let positions = parse_input("inputs/day07")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let (optimal_position_part1, min_fuel_part1) = part1(&positions);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let (optimal_position_part2, min_fuel_part2) = part2(&positions);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nOptimal position: {}\nFuel used: {}\n",
        part1_time, optimal_position_part1, min_fuel_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {}s\nOptimal position: {}\nFuel used: {}\n",
        part2_time, optimal_position_part2, min_fuel_part2
    );

    Ok(())
}
