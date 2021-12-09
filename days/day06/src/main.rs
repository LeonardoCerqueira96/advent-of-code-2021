use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
struct LanternfishShoal(HashMap<u8, usize>);

impl LanternfishShoal {
    fn new(map: HashMap<u8, usize>) -> Self {
        LanternfishShoal(map)
    }

    fn simulate(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let mut next_fish_pop = HashMap::new();
            for (cycle, count) in self.0.iter() {
                if *cycle == 0 {
                    *(next_fish_pop.entry(8).or_insert(0)) += count;
                    *(next_fish_pop.entry(6).or_insert(0)) += count;
                    continue;
                }

                *(next_fish_pop.entry(cycle - 1).or_insert(0)) += count;
            }

            self.0 = next_fish_pop;
        }

        self.0.values().sum()
    }
}

fn parse_input<T>(filename: T) -> io::Result<LanternfishShoal>
where
    T: AsRef<Path>,
{
    let mut lanternfish_map = HashMap::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // There's only one line in the file
    let cycles_str = match input_buf.lines().next() {
        Some(line_result) => line_result?,
        None => return Err(io::Error::new(io::ErrorKind::Other, "File is empty")),
    };

    // Parse cycles
    let cycles: Vec<u8> = cycles_str
        .split(',')
        .map(|a| a.parse::<u8>().unwrap())
        .collect();

    for cycle in cycles {
        *lanternfish_map.entry(cycle).or_insert(0) += 1;
    }

    Ok(LanternfishShoal::new(lanternfish_map))
}

fn part1(mut lanterfish_shoal: LanternfishShoal) -> usize {
    lanterfish_shoal.simulate(80)
}

fn part2(mut lanterfish_shoal: LanternfishShoal) -> usize {
    lanterfish_shoal.simulate(256)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let lanternfish_shoal = parse_input("inputs/day06")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let shoal_size_part1 = part1(lanternfish_shoal.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let shoal_size_part2 = part2(lanternfish_shoal);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nShoal size after 80 days: {}\n",
        part1_time, shoal_size_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nShoal size after 256 days: {}\n",
        part2_time, shoal_size_part2
    );

    Ok(())
}
