use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
enum FoldInstruction {
    Vertical(usize),
    Horizontal(usize),
}

impl FromStr for FoldInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('=').take(2).collect();
        let pos: usize = match split[1].parse() {
            Ok(n) => n,
            _ => return Err(()),
        };

        match split[0] {
            "x" => Ok(Self::Horizontal(pos)),
            "y" => Ok(Self::Vertical(pos)),
            _ => Err(()),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Dot {
    pos_x: usize,
    pos_y: usize,
}

impl Dot {
    fn new(pos_x: usize, pos_y: usize) -> Self {
        Dot { pos_x, pos_y }
    }
}

struct TransparentPaper {
    dimension_x: usize,
    dimension_y: usize,
    dots: HashSet<Dot>,
}

impl TransparentPaper {
    fn new(dimension_x: usize, dimension_y: usize, dots_pos: Vec<(usize, usize)>) -> Self {
        let dots = dots_pos.into_iter().map(|t| Dot::new(t.0, t.1)).collect();

        TransparentPaper {
            dimension_x,
            dimension_y,
            dots,
        }
    }

    fn fold_horizontally(&mut self, line: usize) {
        let mut new_dot_set = HashSet::new();

        // Mirror the dots to the right of the fold line
        self.dots.iter().for_each(|dot| {
            if dot.pos_x < line {
                new_dot_set.insert(dot.clone());
                return;
            }

            let distance_x = dot.pos_x - line;
            new_dot_set.insert(Dot::new(line - distance_x, dot.pos_y));
        });

        self.dots = new_dot_set;
        self.dimension_x = line;
    }

    fn fold_vertically(&mut self, line: usize) {
        let mut new_dot_set = HashSet::new();

        // Mirror the dots below the fold line
        self.dots.iter().for_each(|dot| {
            if dot.pos_y < line {
                new_dot_set.insert(dot.clone());
                return;
            }

            let distance_y = dot.pos_y - line;
            new_dot_set.insert(Dot::new(dot.pos_x, line - distance_y));
        });

        self.dots = new_dot_set;
        self.dimension_y = line;
    }

    fn fold(&mut self, instruction: &FoldInstruction) {
        match instruction {
            FoldInstruction::Horizontal(line) => self.fold_horizontally(*line),
            FoldInstruction::Vertical(line) => self.fold_vertically(*line),
        }
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut paper = vec![vec![' '; self.dimension_x]; self.dimension_y];
        for dot in &self.dots {
            paper[dot.pos_y][dot.pos_x] = '#';
        }

        let paper_str = paper
            .into_iter()
            .map(|l| l.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", paper_str)
    }
}

fn parse_input<T>(filename: T) -> io::Result<(TransparentPaper, Vec<FoldInstruction>)>
where
    T: AsRef<Path>,
{
    let mut dots_pos = Vec::new();
    let mut instructions = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut lines_iter = input_buf.lines();

    // Read dots
    let mut max_x = 0;
    let mut max_y = 0;
    while let Some(Ok(line)) = lines_iter.next() {
        // If it's an empty line, we finished reading the dots
        if line.is_empty() {
            break;
        }

        let dot_pos: Vec<usize> = line
            .split(',')
            .take(2)
            .map(|s| s.parse().unwrap())
            .collect();

        if dot_pos[0] > max_x {
            max_x = dot_pos[0];
        }

        if dot_pos[1] > max_y {
            max_y = dot_pos[1];
        }

        dots_pos.push((dot_pos[0], dot_pos[1]));
    }

    // Read instructions
    while let Some(Ok(line)) = lines_iter.next() {
        // If it's an empty line, we've nothing more to read
        if line.is_empty() {
            break;
        }

        let instruction_str = line.split_ascii_whitespace().nth(2).unwrap();

        instructions.push(FoldInstruction::from_str(instruction_str).unwrap());
    }

    let paper = TransparentPaper::new(max_x + 1, max_y + 1, dots_pos);
    Ok((paper, instructions))
}

fn part1(paper: &mut TransparentPaper, instruction: &FoldInstruction) -> usize {
    paper.fold(instruction);

    paper.dots.len()
}

fn part2(paper: &mut TransparentPaper, instructions: &[FoldInstruction]) {
    for instruction in instructions {
        paper.fold(instruction);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (mut paper, instructions) = parse_input("inputs/day13")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let ndots = part1(&mut paper, &instructions[0]);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    part2(&mut paper, &instructions[1..]);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nNumber of visible dots: {}\n",
        part1_time, ndots
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nFinal paper:\n{}\n",
        part2_time, paper
    );

    Ok(())
}
