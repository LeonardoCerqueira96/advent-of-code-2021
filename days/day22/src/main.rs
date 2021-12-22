use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum CuboidType {
    On,
    Off,
}

impl FromStr for CuboidType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            s => Err(format!("Invalid cuboid type string: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    c_type: CuboidType,
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize),
}

impl FromStr for Cuboid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cuboid_regex = Regex::new(
            r"^(\w+)\s+x=([-]?\d+)..([-]?\d+),y=([-]?\d+)..([-]?\d+),z=([-]?\d+)..([-]?\d+)$",
        )
        .unwrap();
        let captures = cuboid_regex
            .captures(s)
            .ok_or_else(|| "Invalid cuboid input".to_string())?;

        // Parse cuaboid type
        let c_type = if let Some(c_type_match) = captures.get(1) {
            CuboidType::from_str(c_type_match.into())?
        } else {
            return Err("Invalid cuboid input".to_string());
        };

        // Parse x range
        let (x1, x2) = if let (Some(x1_match), Some(x2_match)) = (captures.get(2), captures.get(3))
        {
            let x1: isize = x1_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            let x2: isize = x2_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            (x1, x2)
        } else {
            return Err("Invalid cuboid input".to_string());
        };

        // Parse y range
        let (y1, y2) = if let (Some(y1_match), Some(y2_match)) = (captures.get(4), captures.get(5))
        {
            let y1: isize = y1_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            let y2: isize = y2_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            (y1, y2)
        } else {
            return Err("Invalid cuboid input".to_string());
        };

        // Parse z range
        let (z1, z2) = if let (Some(z1_match), Some(z2_match)) = (captures.get(6), captures.get(7))
        {
            let z1: isize = z1_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            let z2: isize = z2_match
                .as_str()
                .parse()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            (z1, z2)
        } else {
            return Err("Invalid cuboid input".to_string());
        };

        Ok(Self {
            c_type,
            x_range: (x1, x2),
            y_range: (y1, y2),
            z_range: (z1, z2),
        })
    }
}

impl Cuboid {
    fn get_common_range(
        range_a: (isize, isize),
        range_b: (isize, isize),
    ) -> Option<(isize, isize)> {
        if range_a.0 > range_b.1 || range_a.1 < range_b.0 {
            None
        } else {
            Some((range_a.0.max(range_b.0), range_a.1.min(range_b.1)))
        }
    }

    fn get_intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        // Calculate intersection on the x range
        let common_x_range_opt = Self::get_common_range(other.x_range, self.x_range);

        // Calculate intersection on the y range
        let common_y_range_opt = Self::get_common_range(other.y_range, self.y_range);

        // Calculate intersection on the z range
        let common_z_range_opt = Self::get_common_range(other.z_range, self.z_range);

        // If we found intersections in the three ranges, return the intersection cuboid
        if let (Some(x_range), Some(y_range), Some(z_range)) =
            (common_x_range_opt, common_y_range_opt, common_z_range_opt)
        {
            Some(Cuboid {
                c_type: other.c_type,
                x_range,
                y_range,
                z_range,
            })
        } else {
            None
        }
    }

    fn split_from_intersection(mut self, other: &Cuboid) -> Vec<Cuboid> {
        let intersection = self.get_intersection(other);
        if intersection.is_none() {
            return vec![self];
        }
        let intersection = intersection.unwrap();

        let mut new_cubes = Vec::new();

        // See if we have to split the left side of the cube
        if intersection.x_range.0 > self.x_range.0 {
            // Create new cube from split
            let new_x_range = (self.x_range.0, intersection.x_range.0 - 1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: new_x_range,
                y_range: self.y_range,
                z_range: self.z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.x_range.0 = intersection.x_range.0;
        }

        // See if we have to split the right side of the cube
        if intersection.x_range.1 < self.x_range.1 {
            // Create new cube from split
            let new_x_range = (intersection.x_range.1 + 1, self.x_range.1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: new_x_range,
                y_range: self.y_range,
                z_range: self.z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.x_range.1 = intersection.x_range.1;
        }

        // See if we have to slit the back of the cube
        if intersection.z_range.0 > self.z_range.0 {
            // Create new cube from split
            let new_z_range = (self.z_range.0, intersection.z_range.0 - 1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: self.x_range,
                y_range: self.y_range,
                z_range: new_z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.z_range.0 = intersection.z_range.0;
        }

        // See if we have to slit the front of the cube
        if intersection.z_range.1 < self.z_range.1 {
            // Create new cube from split
            let new_z_range = (intersection.z_range.1 + 1, self.z_range.1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: self.x_range,
                y_range: self.y_range,
                z_range: new_z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.z_range.1 = intersection.z_range.1;
        }

        // See if we have to split the bottom of the cube
        if intersection.y_range.0 > self.y_range.0 {
            // Create new cube from split
            let new_y_range = (self.y_range.0, intersection.y_range.0 - 1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: self.x_range,
                y_range: new_y_range,
                z_range: self.z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.y_range.0 = intersection.y_range.0;
        }

        // See if we have to split the top of the cube
        if intersection.y_range.1 < self.y_range.1 {
            // Create new cube from split
            let new_y_range = (intersection.y_range.1 + 1, self.y_range.1);
            let new_cube = Cuboid {
                c_type: self.c_type,
                x_range: self.x_range,
                y_range: new_y_range,
                z_range: self.z_range,
            };
            new_cubes.push(new_cube);

            // Adjust the original cube range
            self.y_range.1 = intersection.y_range.1;
        }

        new_cubes
    }
}

#[derive(Debug)]
struct Reactor {
    on_cuboids: Vec<Cuboid>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            on_cuboids: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.on_cuboids.clear();
    }

    fn execute_instruction(&mut self, new_cuboid: Cuboid) {
        let mut new_on_cuboids = Vec::new();
        for &cuboid in &self.on_cuboids {
            let new_cuboids = cuboid.split_from_intersection(&new_cuboid);
            new_on_cuboids.extend(new_cuboids);
        }

        match new_cuboid.c_type {
            CuboidType::On => new_on_cuboids.push(new_cuboid),
            CuboidType::Off => (),
        };

        self.on_cuboids = new_on_cuboids;
    }

    fn count_on_cubes(&self) -> usize {
        self.on_cuboids.iter().fold(0, |on_count, cbd| {
            let cube_count = (cbd.x_range.1 + 1 - cbd.x_range.0)
                * (cbd.y_range.1 + 1 - cbd.y_range.0)
                * (cbd.z_range.1 + 1 - cbd.z_range.0);

            on_count + cube_count as usize
        })
    }
}

fn parse_input<T>(filename: T) -> io::Result<(Vec<Cuboid>, Vec<Cuboid>)>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);
    let lines_iter = input_buf.lines();

    let mut init_cuboids = Vec::new();
    let mut remaining_cuboids = Vec::new();
    for line_result in lines_iter {
        let line = line_result?;

        let cuboid = Cuboid::from_str(&line).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Failed to parse cuboid: {}", e),
            )
        })?;
        if cuboid.x_range.0 < -50
            || cuboid.x_range.1 > 50
            || cuboid.y_range.0 < -50
            || cuboid.y_range.1 > 50
            || cuboid.z_range.0 < -50
            || cuboid.z_range.1 > 50
        {
            remaining_cuboids.push(cuboid);
        } else {
            init_cuboids.push(cuboid);
        }
    }

    Ok((init_cuboids, remaining_cuboids))
}

fn part1(reactor: &mut Reactor, init_cuboids: Vec<Cuboid>) -> usize {
    for cuboid in init_cuboids {
        reactor.execute_instruction(cuboid);
    }

    reactor.count_on_cubes()
}

fn part2(reactor: &mut Reactor, cuboids: Vec<Cuboid>) -> usize {
    reactor.reset();

    for cuboid in cuboids {
        reactor.execute_instruction(cuboid);
    }

    reactor.count_on_cubes()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (init_cuboids, remaining_cuboids) = parse_input("inputs/day22")?;
    let parse_time = t0.elapsed();

    let mut full_cuboids = init_cuboids.clone();
    full_cuboids.extend(remaining_cuboids);

    let mut reactor = Reactor::new();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let on_cubes_count_part1 = part1(&mut reactor, init_cuboids);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let on_cubes_count_part2 = part2(&mut reactor, full_cuboids);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nOn cubes count: {}\n",
        part1_time, on_cubes_count_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nOn cubes count: {}\n",
        part2_time, on_cubes_count_part2
    );

    Ok(())
}
