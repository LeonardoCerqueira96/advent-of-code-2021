use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

// Returns a vector, where each position holds the 1's count of that column, and the number of lines
fn parse_input<T>(filename: T) -> io::Result<Vec<String>>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read and collect the lines
    let lines: io::Result<Vec<String>> = input_buf.lines().collect();

    lines
}

fn count_one_bits_by_column(binary_numbers: &[String]) -> Vec<usize> {
    let n_columns = binary_numbers[0].len();

    let n_one_bits = binary_numbers
        .iter()
        .fold(vec![0; n_columns], |mut acc, number_str| {
            for (i, digit) in number_str.chars().enumerate() {
                if digit == '1' {
                    acc[i] += 1;
                }
            }

            acc
        });

    n_one_bits
}

fn compute_gamma_rate(binary_numbers: &[String]) -> u32 {
    // Get 1's count by column
    let n_one_bits = count_one_bits_by_column(binary_numbers);

    // Map vector of 1's counts into a binary string
    let n_lines = binary_numbers.len();
    let gamma_str: String = n_one_bits
        .iter()
        .map(|c| if c * 2 < n_lines { '0' } else { '1' })
        .collect();

    // Convert binary string to u32
    u32::from_str_radix(&gamma_str, 2).unwrap()
}

fn compute_epsilon_rate(binary_numbers: &[String]) -> u32 {
    // Get 1's count by column
    let n_one_bits = count_one_bits_by_column(binary_numbers);

    // Map vector of 1's counts into a binary string
    let n_lines = binary_numbers.len();
    let epsilon_str: String = n_one_bits
        .iter()
        .map(|c| if c * 2 < n_lines { '1' } else { '0' })
        .collect();

    // Convert binary string to u32
    u32::from_str_radix(&epsilon_str, 2).unwrap()
}

fn compute_oxygen_generator_rating(mut binary_numbers: Vec<String>) -> u32 {
    // Get 1's count by column
    let mut n_one_bits = count_one_bits_by_column(&binary_numbers);

    let mut n_lines = binary_numbers.len();
    let mut column_index = 0;
    while binary_numbers.len() > 1 {
        // Check which is the most common bit on column `column_index`
        let most_common_bit = if n_one_bits[column_index] * 2 >= n_lines {
            '1'
        } else {
            '0'
        };

        // Keep only numbers with `most_common_bit` on position `column_index`
        binary_numbers = binary_numbers
            .into_iter()
            .filter(|n| n.chars().nth(column_index).unwrap() == most_common_bit)
            .collect();

        // Update number of lines
        n_lines = binary_numbers.len();

        // Update 1's count
        n_one_bits = count_one_bits_by_column(&binary_numbers);

        column_index += 1;
    }

    u32::from_str_radix(&binary_numbers[0], 2).unwrap()
}

fn compute_co2_scrubber_rating(mut binary_numbers: Vec<String>) -> u32 {
    // Get 1's count by column
    let mut n_one_bits = count_one_bits_by_column(&binary_numbers);

    let mut n_lines = binary_numbers.len();
    let mut column_index = 0;
    while binary_numbers.len() > 1 {
        // Check which is the least common bit on column `column_index`
        let least_common_bit = if n_one_bits[column_index] * 2 >= n_lines {
            '0'
        } else {
            '1'
        };

        // Keep only numbers with `most_common_bit` on position `column_index`
        binary_numbers = binary_numbers
            .into_iter()
            .filter(|n| n.chars().nth(column_index).unwrap() == least_common_bit)
            .collect();

        // Update number of lines
        n_lines = binary_numbers.len();

        // Update 1's count
        n_one_bits = count_one_bits_by_column(&binary_numbers);

        column_index += 1;
    }

    u32::from_str_radix(&binary_numbers[0], 2).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let binary_numbers = parse_input("inputs/day03")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let gamma = compute_gamma_rate(&binary_numbers);
    let epsilon = compute_epsilon_rate(&binary_numbers);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let oxygen_generator_rating = compute_oxygen_generator_rating(binary_numbers.clone());
    let co2_scrubber_rating = compute_co2_scrubber_rating(binary_numbers);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nGamma rate: {}\nEpsilon rate: {}\nMultiplied: {}\n",
        part1_time,
        gamma,
        epsilon,
        gamma * epsilon
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {}s\nOxygen Generator Rating: {}\nCO2 Scrubber Rating: {}\nLife Support Rating: {}\n",
        part2_time,
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );

    Ok(())
}
