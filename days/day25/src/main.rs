use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Copy)]
enum Direction {
    East,
    South,
}

#[derive(Clone, Copy)]
struct SeaCucumber {
    move_direction: Direction,
}

impl SeaCucumber {
    fn new(move_direction: Direction) -> Self {
        SeaCucumber { move_direction }
    }
}

struct TrenchSpace {
    occupant: Option<SeaCucumber>,
    position: (usize, usize),
}

impl TrenchSpace {
    fn new(occupant: Option<SeaCucumber>, position: (usize, usize)) -> Self {
        TrenchSpace { occupant, position }
    }
}

struct Trench {
    width: usize,
    height: usize,
    spaces: Vec<Vec<TrenchSpace>>,
}

impl Trench {
    fn new(width: usize, height: usize, spaces: Vec<Vec<TrenchSpace>>) -> Self {
        Trench {
            width,
            height,
            spaces,
        }
    }

    fn move_occupant(&mut self, curr: (usize, usize), dest: (usize, usize)) {
        let occupant = self.spaces[curr.0][curr.1].occupant;
        self.spaces[curr.0][curr.1].occupant = None;
        self.spaces[dest.0][dest.1].occupant = occupant;
    }

    fn run_step(&mut self) -> bool {
        let mut had_movements = false;

        // First try to move east facing cucumbers
        let east_moves: Vec<_> = self
            .spaces
            .iter()
            .flatten()
            .filter(|space| space.occupant.is_some())
            .filter(|space| matches!(space.occupant.unwrap().move_direction, Direction::East))
            .filter(|space| {
                self.spaces[space.position.0][(space.position.1 + 1) % self.width]
                    .occupant
                    .is_none()
            })
            .map(|space| {
                (
                    (space.position.0, space.position.1),
                    (space.position.0, (space.position.1 + 1) % self.width),
                )
            })
            .collect();

        if !east_moves.is_empty() {
            for movement in east_moves.into_iter() {
                self.move_occupant(movement.0, movement.1);
            }
            had_movements = true;
        }

        // Now try to move south facing cucumbers
        let south_moves: Vec<_> = self
            .spaces
            .iter()
            .flatten()
            .filter(|space| space.occupant.is_some())
            .filter(|space| matches!(space.occupant.unwrap().move_direction, Direction::South))
            .filter(|space| {
                self.spaces[(space.position.0 + 1) % self.height][space.position.1]
                    .occupant
                    .is_none()
            })
            .map(|space| {
                (
                    (space.position.0, space.position.1),
                    ((space.position.0 + 1) % self.height, space.position.1),
                )
            })
            .collect();

        if !south_moves.is_empty() {
            for movement in south_moves.into_iter() {
                self.move_occupant(movement.0, movement.1);
            }
            had_movements = true;
        }

        had_movements
    }

    fn run_until_end(&mut self) -> usize {
        let mut step_count = 1;
        while self.run_step() {
            step_count += 1
        }

        step_count
    }
}

impl fmt::Display for Trench {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let trench_str: String = self
            .spaces
            .iter()
            .map(|line| {
                line.iter()
                    .map(|space| match space.occupant {
                        Some(cuc) if matches!(cuc.move_direction, Direction::East) => '>',
                        Some(cuc) if matches!(cuc.move_direction, Direction::South) => 'v',
                        Some(_) => panic!("Unknown direction"),
                        None => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", trench_str)
    }
}

fn parse_input<T>(filename: T) -> io::Result<Trench>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);
    let lines_iter = input_buf.lines();

    let mut spaces = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for line_result in lines_iter {
        let line = line_result?;

        let spaces_line = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '.' => TrenchSpace::new(None, (height, i)),
                '>' => TrenchSpace::new(Some(SeaCucumber::new(Direction::East)), (height, i)),
                'v' => TrenchSpace::new(Some(SeaCucumber::new(Direction::South)), (height, i)),
                _ => panic!("Unknown char {}", c),
            })
            .collect();

        height += 1;
        if width == 0 {
            width = line.len();
        }

        spaces.push(spaces_line);
    }

    Ok(Trench::new(width, height, spaces))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input
    let mut trench = parse_input("inputs/day25")?;

    // Compute part 1
    let steps = trench.run_until_end();
    println!("Took {} steps", steps);

    Ok(())
}
