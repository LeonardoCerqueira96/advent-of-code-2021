use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

enum SubPackageSize {
    Bits(usize),
    Count(usize),
}

impl FromStr for SubPackageSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length_type_id = s
            .get(0..1)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid length type ID: {}", e)))
            .ok_or(format!("Failed to extract range 0..1 from string"))??;

        match length_type_id {
            0 => {
                let bits = s
                    .get(1..16)
                    .map(|r| {
                        usize::from_str_radix(r, 2).map_err(|e| format!("Invalid length: {}", e))
                    })
                    .ok_or(format!("Failed to extract range 1..16 from string"))??;

                Ok(Self::Bits(bits))
            }
            1 => {
                let count = s
                    .get(1..12)
                    .map(|r| {
                        usize::from_str_radix(r, 2).map_err(|e| format!("Invalid count: {}", e))
                    })
                    .ok_or(format!("Failed to extract range 1..12 from string"))??;

                Ok(Self::Count(count))
            }
            id => Err(format!("Invalid length type ID: {}", id)),
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    package_type: Box<PacketType>,
}

impl Packet {
    // Returns parsed packet and index at which it finished parsing the string
    fn from_str(bin_str: &str) -> Result<(Self, usize), String> {
        // Initialize index
        let mut index = 0;

        // Parse version
        let version = bin_str
            .get(index..index + 3)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid version: {}", e)))
            .ok_or(format!(
                "Failed to extract range {}..{} from binary",
                index,
                index + 3
            ))??;
        index += 3;

        // Parse type ID
        let packet_type_id = bin_str
            .get(index..index + 3)
            .map(|r| u8::from_str_radix(r, 2).map_err(|e| format!("Invalid packet type ID: {}", e)))
            .ok_or(format!(
                "Failed to extract range {}..{} from binary",
                index,
                index + 3
            ))??;
        index += 3;

        let package_type = match packet_type_id {
            4 => {
                // Literal packet
                let mut value_str = String::new();

                // Read groups of 5 bits
                while let Some(group) = bin_str.get(index..index + 5) {
                    value_str.push_str(&group[1..]);
                    index += 5;

                    // If a group starts with 0, it's the last group
                    if group.starts_with('0') {
                        break;
                    }
                }

                let value = usize::from_str_radix(&value_str, 2)
                    .map_err(|e| format!("Invalid literal: {}", e))?;

                Box::new(PacketType::Literal(value))
            }
            _op_type => {
                // Operator packet
                let packet_size = SubPackageSize::from_str(&bin_str[index..])?;
                let packets = match packet_size {
                    SubPackageSize::Bits(bits) => {
                        index += 16;
                        let mut packets = Vec::new();
                        let limit = index + bits;
                        while index < limit {
                            let (packet, diff) = Self::from_str(&bin_str[index..])?;
                            index += diff;
                            packets.push(packet);
                        }
                        packets
                    }
                    SubPackageSize::Count(count) => {
                        index += 12;
                        (0..count).try_fold(Vec::new(), |mut packets, _| {
                            let (packet, diff) = Self::from_str(&bin_str[index..])?;
                            index += diff;
                            packets.push(packet);

                            Result::<_, String>::Ok(packets)
                        })?
                    }
                };

                Box::new(PacketType::Operator(packets))
            }
        };

        Ok((
            Packet {
                version,
                package_type,
            },
            index,
        ))
    }

    fn get_version_sum(&self) -> usize {
        self.version as usize
            + match self.package_type.as_ref() {
                PacketType::Operator(packets) => packets.iter().map(|p| p.get_version_sum()).sum(),
                PacketType::Literal(_) => 0,
            }
    }
}

fn parse_input<T>(filename: T) -> io::Result<Packet>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let mut input_buf = BufReader::new(input);

    // The file has only one line
    let mut hex_string = String::new();
    input_buf.read_line(&mut hex_string)?;

    // Convert hex string to binary
    let binary_string = hex_string
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<Vec<String>>()
        .join("");

    let (packet, _) =
        Packet::from_str(&binary_string).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(packet)
}

fn part1(packet: &Packet) -> usize {
    packet.get_version_sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let packet = parse_input("inputs/day16")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let version_sum = part1(&packet);
    let part1_time = t1.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nVersion sum: {}\n",
        part1_time, version_sum
    );

    Ok(())
}
