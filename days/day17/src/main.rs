use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use regex::Regex;

type TargetArea = ((isize, isize), (isize, isize));
fn parse_input<T>(filename: T) -> io::Result<TargetArea>
where
    T: AsRef<Path>,
{
    // Regular expression used to match against the input
    let re = Regex::new(r"^.*x=([-]?\d+)\.\.([-]?\d+).*y=([-]?\d+)\.\.([-]?\d+).*$").unwrap();

    // Open input file
    let input = File::open(filename)?;
    let mut input_buf = BufReader::new(input);

    // The file has only one line
    let mut target_str = String::new();
    input_buf.read_line(&mut target_str)?;

    // Get captures
    let caps = match re.captures(target_str.trim()) {
        Some(caps) => caps,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid input line: {}", target_str),
            ))
        }
    };

    // Extract ranges from captures
    let (x1, x2, y1, y2) = if let (Some(x1_cap), Some(x2_cap), Some(y1_cap), Some(y2_cap)) =
        (caps.get(1), caps.get(2), caps.get(3), caps.get(4))
    {
        let x1 = x1_cap
            .as_str()
            .parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let x2 = x2_cap
            .as_str()
            .parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let y1 = y1_cap
            .as_str()
            .parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let y2 = y2_cap
            .as_str()
            .parse::<isize>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        (x1, x2, y1, y2)
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Invalid input line: {}", target_str),
        ));
    };

    Ok(((x1, x2), (y1, y2)))
}

// Calculates the mininum x velocity needed to reach the target
fn get_min_x_velocity(target_x1: isize) -> isize {
    let mut pos_x = 0;
    let mut min_vx = 0;

    while pos_x < target_x1 {
        min_vx += 1;
        pos_x += min_vx;
    }

    min_vx
}

// Calculates the maximum y velocity that will hit the target and the highest point possible
fn get_max_y_velocity_and_peak(target_y: (isize, isize)) -> (isize, isize) {
    let mut max_vy = 0;
    let mut highest_peak = 0;

    for vy_it in 0..=target_y.0.abs() {
        let mut pos_y = 0;
        let mut curr_highest_peak = 0;

        let mut hit_target = false;
        let mut overshot_target = false;

        let mut vy = vy_it;
        while !hit_target && !overshot_target {
            pos_y += vy;
            vy -= 1;

            if pos_y > curr_highest_peak {
                curr_highest_peak = pos_y;
            }

            hit_target = pos_y >= target_y.0 && pos_y <= target_y.1;
            overshot_target = pos_y < target_y.0;
        }

        if hit_target {
            max_vy = vy_it;
            highest_peak = curr_highest_peak;
        }
    }

    (max_vy, highest_peak)
}

fn get_all_possible_velocities(target: TargetArea) -> Vec<(isize, isize)> {
    let (target_x, target_y) = target;
    let min_vx = get_min_x_velocity(target_x.0);

    let mut hit_velocities = Vec::new();
    for xv_it in min_vx..=target_x.1 {
        for yv_it in target_y.0..=target_y.0.abs() {
            let mut pos_x = 0;
            let mut pos_y = 0;

            let mut hit_target = false;
            let mut overshot_target = false;

            let mut xv = xv_it;
            let mut yv = yv_it;
            while !hit_target && !overshot_target {
                pos_x += xv;
                pos_y += yv;

                if xv.signum() != 0 {
                    xv = (xv.abs() - 1) * xv.signum();
                }
                yv -= 1;

                hit_target = (pos_x >= target_x.0 && pos_x <= target_x.1)
                    && (pos_y >= target_y.0 && pos_y <= target_y.1);
                overshot_target = pos_y < target_y.0 || pos_x > target_x.1;
            }

            if hit_target {
                hit_velocities.push((xv_it, yv_it))
            }
        }
    }

    hit_velocities
}

fn part1(target: TargetArea) -> isize {
    let (_target_x, target_y) = target;
    let (_max_vy, peak) = get_max_y_velocity_and_peak(target_y);

    peak
}

fn part2(target: TargetArea) -> usize {
    let hit_velocities = get_all_possible_velocities(target);

    hit_velocities.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let target = parse_input("inputs/day17")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let max_peak = part1(target);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let nhit_velocities = part2(target);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nMax y peak: {}\n",
        part1_time, max_peak
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nNumber of hit velocities: {}\n",
        part2_time, nhit_velocities
    );

    Ok(())
}
