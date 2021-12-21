use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn move_pos(&mut self, die: &mut dyn Die) {
        let mut new_pos = self.position + die.roll() + die.roll() + die.roll();
        if new_pos > 10 {
            if new_pos % 10 == 0 {
                new_pos = 10;
            } else {
                new_pos %= 10;
            }
        }

        self.score += new_pos;
        self.position = new_pos;
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }
}

trait Die {
    fn roll(&mut self) -> usize;
}

struct DeterministicDie {
    nsides: usize,
    previous_roll: usize,
    nrolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            nsides: 100,
            previous_roll: 0,
            nrolls: 0,
        }
    }

    fn get_roll_count(&self) -> usize {
        self.nrolls
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let mut new_row = self.previous_roll + 1;
        if new_row == (self.nsides) + 1 {
            new_row = 1;
        }

        self.nrolls += 1;

        self.previous_roll = new_row;
        new_row
    }
}

fn parse_input<T>(filename: T) -> io::Result<(Player, Player)>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);
    let mut lines_iter = input_buf.lines();

    // First line is the first player
    let first_player_str = lines_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Input file is empty"))??;
    let first_pos_index = first_player_str
        .find(':')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid player input"))?
        + 2;
    let first_player_pos = first_player_str[first_pos_index..]
        .parse::<usize>()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to parse position: {}", e),
            )
        })?;
    let player1 = Player::new(first_player_pos);

    // Second line is the second player
    let second_player_str = lines_iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No second player found"))??;
    let second_pos_index = second_player_str
        .find(':')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid player input"))?
        + 2;
    let second_player_pos = second_player_str[second_pos_index..]
        .parse::<usize>()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to parse position: {}", e),
            )
        })?;
    let player2 = Player::new(second_player_pos);

    Ok((player1, player2))
}

fn part1(mut player1: Player, mut player2: Player) -> usize {
    let mut die = DeterministicDie::new();

    let mut _loser = None;
    loop {
        player1.move_pos(&mut die);
        if player1.has_won() {
            _loser = Some(&player2);
            break;
        }

        player2.move_pos(&mut die);
        if player2.has_won() {
            _loser = Some(&player1);
            break;
        }
    }

    let loser = _loser.unwrap();

    loser.score * die.get_roll_count()
}

fn part2(player1: Player, player2: Player) -> usize {
    let mut universes_map = HashMap::new();
    universes_map.insert([player1, player2], 1_usize);

    let mut wins = [0, 0]; // (p1 wins, p2 wins)

    while !universes_map.is_empty() {
        // Two turns, one for each player
        for i in 0..2 {
            // Update universes
            let mut new_universes_map = HashMap::new();
            for (players, &count) in universes_map.iter() {
                let quantum_rolls_iter = itertools::cons_tuples(
                    (1..=3).cartesian_product(1..=3).cartesian_product(1..=3),
                );
                for (r1, r2, r3) in quantum_rolls_iter {
                    let mut players = players.clone();

                    players[i].position += r1 + r2 + r3;
                    if players[i].position > 10 {
                        players[i].position -= 10;
                    }

                    players[i].score += players[i].position;
                    if players[i].score >= 21 {
                        wins[i] += count;
                    } else {
                        *new_universes_map.entry(players).or_insert(0) += count;
                    }
                }
            }
            universes_map = new_universes_map;
        }
    }

    *wins.iter().max().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (player1, player2) = parse_input("inputs/day21")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let part1_result = part1(player1.clone(), player2.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let part2_result = part2(player1, player2);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nLosing score x number of rolls: {}\n",
        part1_time, part1_result
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nPlayer that wins in more universes wins in: {}\n",
        part2_time, part2_result
    );

    Ok(())
}
