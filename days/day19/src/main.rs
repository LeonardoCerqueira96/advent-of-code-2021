use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use fxhash::FxHashSet;

static CHANGE_OF_BASIS_MATRIXES: [Matrix3<isize>; 24] = [
    Matrix3::new(
        1,  0,  0,
        0,  1,  0,
        0,  0,  1,
    ),
    Matrix3::new(
        1,  0,  0,
        0, -1,  0,
        0,  0, -1,
    ),
    Matrix3::new(
        1,  0,  0,
        0,  0, -1,
        0,  1,  0,
    ),
    Matrix3::new(
        1,  0,  0,
        0,  0,  1,
        0, -1,  0,
    ),
    Matrix3::new(
       -1,  0,  0,
        0, -1,  0,
        0,  0,  1,
    ),
    Matrix3::new(
       -1,  0,  0,
        0,  1,  0,
        0,  0, -1,
    ),
    Matrix3::new(
       -1,  0,  0,
        0,  0,  1,
        0,  1,  0,
    ),
    Matrix3::new(
       -1,  0,  0,
        0,  0, -1,
        0, -1,  0,
    ),
    Matrix3::new(
        0, -1,  0,
        1,  0,  0,
        0,  0,  1,
    ),
    Matrix3::new(
        0,  1,  0,
        1,  0,  0,
        0,  0, -1,
    ),
    Matrix3::new(
        0,  0,  1,
        1,  0,  0,
        0,  1,  0,
    ),
    Matrix3::new(
        0,  0, -1,
        1,  0,  0,
        0, -1,  0,
    ),
    Matrix3::new(
        0,  1,  0,
       -1,  0,  0,
        0,  0,  1,
    ),
    Matrix3::new(
        0, -1,  0,
       -1,  0,  0,
        0,  0, -1,
    ),
    Matrix3::new(
        0,  0, -1,
       -1,  0,  0,
        0,  1,  0,
    ),
    Matrix3::new(
        0,  0,  1,
       -1,  0,  0,
        0, -1,  0,
    ),
    Matrix3::new(
        0,  0, -1,
        0,  1,  0,
        1,  0,  0,
    ),
    Matrix3::new(
        0,  0,  1,
        0, -1,  0,
        1,  0,  0,
    ),
    Matrix3::new(
        0,  1,  0,
        0,  0,  1,
        1,  0,  0,
    ),
    Matrix3::new(
        0, -1,  0,
        0,  0, -1,
        1,  0,  0,
    ),
    Matrix3::new(
        0,  0,  1,
        0,  1,  0,
       -1,  0,  0,
    ),
    Matrix3::new(
        0,  0, -1,
        0, -1,  0,
       -1,  0,  0,
    ),
    Matrix3::new(
        0,  1,  0,
        0,  0, -1,
       -1,  0,  0,
    ),
    Matrix3::new(
        0, -1,  0,
        0,  0,  1,
       -1,  0,  0,
    ),
];

#[derive(Debug)]
struct Scan {
    beacons: Vec<Vector3<isize>>,
}

fn parse_input<T>(filename: T) -> io::Result<Vec<Scan>>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Collect scans
    let mut scans = Vec::new();
    let mut curr_scan_vec = Vec::new();
    for line_result in input_buf.lines() {
        let line = line_result?;

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // When reading a new scan, save the previous one (if it has content)
        if line.starts_with("---") {
            if !curr_scan_vec.is_empty() {
                scans.push(Scan { beacons: curr_scan_vec.clone() });
                curr_scan_vec.clear();
            }
            continue;
        }

        // Read the 3D point
        let beacon = Vector3::from_iterator(
            line.split(',')
                .take(3)
                .map(|v| v.parse::<isize>().expect(&format!("Invalid number: {}", v)))
        );
        curr_scan_vec.push(beacon);
    }
    scans.push(Scan { beacons: curr_scan_vec });

    Ok(scans)
}

fn try_update_scan(complete_scan: &mut FxHashSet<Vector3<isize>>, scan: &Scan) -> Option<Vector3<isize>> {
    for base_transform_mtx in &CHANGE_OF_BASIS_MATRIXES {
        let beacons = &scan.beacons;
        
        // Transform all the beacon scans into the new base
        let transformed_beacons = beacons.iter()
            .map(|b| base_transform_mtx * b )
            .collect::<Vec<_>>();
        
        // Build iterator over the distance between all pairs of points    
        let distances_iter = complete_scan.iter()
            .cartesian_product(&transformed_beacons)
            .map(|(orig, dest)| orig - dest); 
     
        for dist in distances_iter {
            // Translate all beacons scans by this distance
            let translated_beacons_iter = transformed_beacons.iter()
                .map(|b| b + dist);

            // Count overlapping beacons
            let overlap_count = translated_beacons_iter.clone()
                .filter(|tv| complete_scan.contains(tv))
                .count();

            // If we have at least 12 overlapping beacons, update the scan set
            if overlap_count >= 12 {
                complete_scan.extend(translated_beacons_iter);
                return Some(dist);
            }
        }
    }

    None
}

// Parts 1 and 2 are computed at the same time
fn part1_2(mut scans: Vec<Scan>) -> (usize, usize) {
    // Build initial scan set from scanner 0
    let mut complete_scan = scans.remove(0)
        .beacons.into_iter()
        .collect::<FxHashSet<Vector3<_>>>();

    let mut distances = Vec::new();

    while !scans.is_empty() {
        for i in (0..scans.len()).rev() {
            if let Some(dist) = try_update_scan(&mut complete_scan, &scans[i]) {
                distances.push(dist);
                scans.swap_remove(i);
            }
        }
    }

    let max_distance = distances.iter()
        .tuple_combinations()
        .map(|(s1, s2)| (s1 - s2).abs().sum() )
        .max()
        .unwrap();

    (complete_scan.len(), max_distance as usize)
}

fn main() ->Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let scans = parse_input("inputs/day19")?;
    let parse_time = t0.elapsed();

    // Compute parts
    let t1 = Instant::now();
    let (nbeacons, max_distance) = part1_2(scans);
    let parts_time = t1.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let parts_time = parts_time.as_secs() as f64 + parts_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1 + Part 2:\nTook {:.9}s\nNumber of beacons: {}\nMax distance between scanners: {}\n",
        parts_time, nbeacons, max_distance
    );

    Ok(())
}
