use std::collections::LinkedList;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
struct HeightPoint {
    row: usize,
    col: usize,
    height: u8,
}

impl HeightPoint {
    fn new(row: usize, col: usize, height: u8) -> Self {
        HeightPoint { row, col, height }
    }
}

impl PartialEq for HeightPoint {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

#[derive(Debug)]
struct HeightMap {
    points: Vec<Vec<HeightPoint>>,
    nrows: usize,
    ncols: usize,
}

impl HeightMap {
    fn new(heights: Vec<Vec<u8>>) -> Self {
        let mut points = Vec::new();
        let mut ncols = 0;
        for (i, row) in heights.into_iter().enumerate() {
            points.push(Vec::new());
            for (j, height) in row.into_iter().enumerate() {
                points[i].push(HeightPoint::new(i, j, height));
            }
            ncols = points[i].len();
        }
        let nrows = points.len();

        HeightMap {
            points,
            nrows,
            ncols,
        }
    }

    fn get_low_points(&self) -> Vec<&HeightPoint> {
        let mut low_points = Vec::new();

        for (i, row) in self.points.iter().enumerate() {
            for (j, point) in row.iter().enumerate() {
                if (j > 0 && point.height >= self.points[i][j-1].height)                    // Height to the left is lower or equal
                    || (j < (self.ncols-1) && point.height >= self.points[i][j+1].height)   // Height to the right is lower or equal
                    || (i > 0 && point.height >= self.points[i-1][j].height)                // Height above is lower or equal
                    || (i < (self.nrows-1) && point.height >= self.points[i+1][j].height)
                // Height below is lower or equal
                {
                    continue;
                }

                low_points.push(point);
            }
        }

        low_points
    }

    fn get_basin_sizes(&self) -> Vec<usize> {
        let mut basin_sizes = Vec::new();

        // Each low point has a basin
        for low_point in self.get_low_points() {
            // List the basin points to not repeat lookup
            let mut basin_points = vec![low_point];

            // Setup lookup stack
            let mut lookup_stack = LinkedList::new();
            lookup_stack.push_back(low_point);

            while let Some(point) = lookup_stack.pop_back() {
                // Check point to the left
                if point.col > 0
                    && self.points[point.row][point.col - 1].height < 9
                    && self.points[point.row][point.col - 1].height > point.height
                {
                    if !basin_points.contains(&&self.points[point.row][point.col - 1]) {
                        basin_points.push(&self.points[point.row][point.col - 1]);
                        lookup_stack.push_back(&self.points[point.row][point.col - 1]);
                    }
                }

                // Check point to the right
                if point.col < self.ncols - 1
                    && self.points[point.row][point.col + 1].height < 9
                    && self.points[point.row][point.col + 1].height > point.height
                {
                    if !basin_points.contains(&&self.points[point.row][point.col + 1]) {
                        basin_points.push(&self.points[point.row][point.col + 1]);
                        lookup_stack.push_back(&self.points[point.row][point.col + 1]);
                    }
                }

                // Check point above
                if point.row > 0
                    && self.points[point.row - 1][point.col].height < 9
                    && self.points[point.row - 1][point.col].height > point.height
                {
                    if !basin_points.contains(&&self.points[point.row - 1][point.col]) {
                        basin_points.push(&self.points[point.row - 1][point.col]);
                        lookup_stack.push_back(&self.points[point.row - 1][point.col]);
                    }
                }

                // Check point below
                if point.row < self.nrows - 1
                    && self.points[point.row + 1][point.col].height < 9
                    && self.points[point.row + 1][point.col].height > point.height
                {
                    if !basin_points.contains(&&self.points[point.row + 1][point.col]) {
                        basin_points.push(&self.points[point.row + 1][point.col]);
                        lookup_stack.push_back(&self.points[point.row + 1][point.col]);
                    }
                }
            }

            basin_sizes.push(basin_points.len());
        }

        basin_sizes
    }
}

fn parse_input<T>(filename: T) -> io::Result<HeightMap>
where
    T: AsRef<Path>,
{
    let mut heights = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Each char is a height value
        let height_row: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        heights.push(height_row);
    }

    Ok(HeightMap::new(heights))
}

fn part1(height_map: &HeightMap) -> usize {
    // Sum riks levels for all low points
    height_map
        .get_low_points()
        .into_iter()
        .fold(0, |acc, low_point| acc + (low_point.height as usize) + 1)
}

fn part2(height_map: &HeightMap) -> usize {
    let mut basin_sizes = height_map.get_basin_sizes();
    basin_sizes.sort_by_key(|b| usize::MAX - *b);

    basin_sizes.into_iter().take(3).product()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let height_map = parse_input("inputs/day09")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let risk_level_sum = part1(&height_map);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let basil_size_mult = part2(&height_map);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nRisk level sum: {}\n",
        part1_time, risk_level_sum
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nSize of the three largest basins multiplied: {}\n",
        part2_time, basil_size_mult
    );

    Ok(())
}
