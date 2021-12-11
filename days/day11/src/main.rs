use std::collections::LinkedList;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone)]
struct DumboOctopus {
    row: usize,
    col: usize,
    energy_level: usize,
}

impl DumboOctopus {
    fn new(row: usize, col: usize, energy_level: usize) -> Self {
        DumboOctopus {
            row,
            col,
            energy_level,
        }
    }
}

#[derive(Debug, Clone)]
struct Consortium {
    nrows: usize,
    ncols: usize,
    octopi: Vec<Vec<DumboOctopus>>,
}

impl Consortium {
    fn new(energy_levels: Vec<Vec<usize>>) -> Self {
        let nrows = energy_levels.len();
        let ncols = energy_levels.first().unwrap().len();
        let octopi = energy_levels
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(j, energy_level)| DumboOctopus::new(i, j, energy_level))
                    .collect()
            })
            .collect();

        Consortium {
            nrows,
            ncols,
            octopi,
        }
    }

    fn step(&mut self) -> usize {
        let mut flash_stack = LinkedList::new();

        // Increase all energy levels
        for row in &mut self.octopi {
            for octopus in row {
                octopus.energy_level += 1;
                if octopus.energy_level > 9 {
                    flash_stack.push_back((octopus.row, octopus.col));
                }
            }
        }

        // Do flashes
        let mut nflashes = 0;
        while let Some(octopus_pos) = flash_stack.pop_back() {
            // Reset energy level
            self.octopi[octopus_pos.0][octopus_pos.1].energy_level = 0;
            nflashes += 1;

            // Check adjacent octopi
            self.octopi
                .iter_mut()
                .flatten()
                .filter(|oct| {
                    (oct.row != octopus_pos.0 || oct.col != octopus_pos.1)
                        && (oct.row >= octopus_pos.0.saturating_sub(1)
                            && oct.row <= octopus_pos.0 + 1)
                        && (oct.col >= octopus_pos.1.saturating_sub(1)
                            && oct.col <= octopus_pos.1 + 1)
                })
                .for_each(|oct| {
                    // Increase energy levels of adjacent octopi (if they haven't flashed)
                    if oct.energy_level > 0 {
                        oct.energy_level += 1;
                    }

                    // Check if the adjacent octopi should flash
                    if oct.energy_level > 9 && !flash_stack.contains(&(oct.row, oct.col)) {
                        flash_stack.push_back((oct.row, oct.col));
                    }
                });
        }

        nflashes
    }

    fn all_have_flashed(&self) -> bool {
        self.octopi
            .iter()
            .flatten()
            .all(|oct| oct.energy_level == 0)
    }

    fn simulate(&mut self, steps: usize) -> usize {
        let mut nflashes = 0;
        for _i in 1..=steps {
            nflashes += self.step();
        }

        nflashes
    }

    fn simulate_until_all_flash(&mut self) -> usize {
        let mut nsteps = 0;
        while !self.all_have_flashed() {
            self.step();
            nsteps += 1;
        }

        nsteps
    }
}

impl std::fmt::Display for Consortium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let consortium_str = self
            .octopi
            .iter()
            .map(|row| {
                row.iter()
                    .map(|oct| oct.energy_level.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", consortium_str)
    }
}

fn parse_input<T>(filename: T) -> io::Result<Consortium>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let energy_levels: io::Result<Vec<Vec<usize>>> = input_buf
        .lines()
        .map(|row| {
            row?.chars()
                .map(|lvl| {
                    lvl.to_digit(10)
                        .ok_or_else(|| io::Error::new(
                            io::ErrorKind::Other,
                            format!("Invalid digit {}", lvl),
                        ))
                        .map(|d| d as usize)
                })
                .collect()
        })
        .collect();

    Ok(Consortium::new(energy_levels?))
}

fn part1(mut consortium: Consortium) -> usize {
    consortium.simulate(100)
}

fn part2(mut consortium: Consortium) -> usize {
    consortium.simulate_until_all_flash()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let consortium = parse_input("inputs/day11")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let nflashes = part1(consortium.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let steps_until_sync = part2(consortium);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nNumber of flashes after 100 steps: {}\n",
        part1_time, nflashes
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nSteps until all octopi sync: {}\n",
        part2_time, steps_until_sync
    );

    Ok(())
}
