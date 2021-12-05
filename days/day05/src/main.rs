use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use regex::Regex;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
struct Line {
    point_a: Point,
    point_b: Point,
}

impl Line {
    fn new(point_a: Point, point_b: Point) -> Self {
        Line { point_a, point_b }
    }
}

struct Diagram {
    min: Point,
    values: Vec<Vec<usize>>,
}

impl Diagram {
    fn new(min: Point, max: Point) -> Self {
        let values = vec![vec![0; max.x - min.x + 1]; max.y - min.y + 1];

        Diagram { min, values }
    }

    fn fill(mut self, lines: &[Line]) -> Self {
        for line in lines {
            if line.point_a.x == line.point_b.x {
                // Horizontal line
                let x = line.point_a.x - self.min.x;
                let (y1, y2) = if line.point_a.y > line.point_b.y {
                    (line.point_b.y - self.min.y, line.point_a.y - self.min.y)
                } else {
                    (line.point_a.y - self.min.y, line.point_b.y - self.min.y)
                };

                for y in y1..=y2 {
                    self.values[y][x] += 1;
                }
            } else if line.point_a.y == line.point_b.y {
                // Vertical line
                let y = line.point_a.y - self.min.y;
                let (x1, x2) = if line.point_a.x > line.point_b.x {
                    (line.point_b.x - self.min.x, line.point_a.x - self.min.x)
                } else {
                    (line.point_a.x - self.min.x, line.point_b.x - self.min.x)
                };

                for x in x1..=x2 {
                    self.values[y][x] += 1;
                }
            } else {
                // Diagonal line (always 45 degrees)
                let (x1, y1) = (line.point_a.x - self.min.x, line.point_a.y - self.min.y);
                let (x2, y2) = (line.point_b.x - self.min.x, line.point_b.y - self.min.y);
                let line_iter: Vec<(usize, usize)> = if x1 > x2 {
                    if y1 > y2 {
                        (((y2..=y1).rev()).zip((x2..=x1).rev())).collect()
                    } else {
                        ((y1..=y2).zip((x2..=x1).rev())).collect()
                    }
                } else if y1 > y2 {
                    (((y2..=y1).rev()).zip(x1..=x2)).collect()
                } else {
                    ((y1..=y2).zip(x1..=x2)).collect()
                };

                for (y, x) in line_iter {
                    self.values[y][x] += 1;
                }
            }
        }

        self
    }

    fn overlap_count(&self) -> usize {
        self.values.iter().flatten().filter(|&&v| v > 1).count()
    }
}

fn parse_input<T>(filename: T) -> io::Result<(Vec<Line>, Point, Point)>
where
    T: AsRef<Path>,
{
    // Regular expression used to match against the input lines
    let re = Regex::new(r"^(\d+),(\d+)\s*->\s*(\d+),(\d+)$").unwrap();

    let mut lines = Vec::new();

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Get captures
        let caps = match re.captures(&line) {
            Some(caps) => caps,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid input line: {}", line),
                ))
            }
        };

        // Extract points from captures
        let (x1, y1, x2, y2) = if let (Some(x1_cap), Some(y1_cap), Some(x2_cap), Some(y2_cap)) =
            (caps.get(1), caps.get(2), caps.get(3), caps.get(4))
        {
            let x1 = x1_cap
                .as_str()
                .parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let y1 = y1_cap
                .as_str()
                .parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let x2 = x2_cap
                .as_str()
                .parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let y2 = y2_cap
                .as_str()
                .parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            (x1, y1, x2, y2)
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid input line: {}", line),
            ));
        };

        // Updates max and min
        if x1 > max_x {
            max_x = x1
        }
        if x2 > max_x {
            max_x = x2
        }

        if x1 < min_x {
            min_x = x1
        }
        if x2 < min_x {
            min_x = x2
        }

        if y1 > max_y {
            max_y = y1
        }
        if y2 > max_y {
            max_y = y2
        }

        if y1 < min_y {
            min_y = y1
        }
        if y2 < min_y {
            min_y = y2
        }

        // Build line
        let point_a = Point::new(x1, y1);
        let point_b = Point::new(x2, y2);
        let new_line = Line::new(point_a, point_b);
        lines.push(new_line);
    }

    // Build extreme points
    let min_extreme = Point::new(min_x, min_y);
    let max_extreme = Point::new(max_x, max_y);

    Ok((lines, min_extreme, max_extreme))
}

fn part1(lines: Vec<Line>, min_extreme: Point, max_extreme: Point) -> usize {
    // Keep only horizontal and vertical lines
    let lines: Vec<Line> = lines
        .into_iter()
        .filter(|l| l.point_a.x == l.point_b.x || l.point_a.y == l.point_b.y)
        .collect();

    // Build and fill diagram with overlaps
    let diagram = Diagram::new(min_extreme, max_extreme).fill(&lines);

    // Return the overlap count
    diagram.overlap_count()
}

fn part2(lines: Vec<Line>, min_extreme: Point, max_extreme: Point) -> usize {
    // Build and fill diagram with overlaps
    let diagram = Diagram::new(min_extreme, max_extreme).fill(&lines);

    // Return the overlap count
    diagram.overlap_count()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (lines, min_extreme, max_extreme) = parse_input("inputs/day05")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let overlap_count_part1 = part1(lines.clone(), min_extreme.clone(), max_extreme.clone());
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let overlap_count_part2 = part2(lines, min_extreme, max_extreme);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nOverlap count: {}\n",
        part1_time, overlap_count_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {}s\nOverlap count: {}\n",
        part2_time, overlap_count_part2
    );

    Ok(())
}
