use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use std::{io, panic};

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            _ => panic!("Invalid pixel {}", c),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel_str = match self {
            Self::Light => '#',
            Self::Dark => '.',
        };

        write!(f, "{}", pixel_str)
    }
}

#[derive(Debug, Clone)]
struct Image {
    nrows: usize,
    ncols: usize,
    pixels: Vec<Vec<Pixel>>,
    infinity_pixel: Pixel,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .lines()
            .map(|l| l.chars().map(|c| Pixel::from(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let nrows = pixels.len();
        let ncols = pixels[0].len();

        Ok(Self {
            nrows,
            ncols,
            pixels,
            infinity_pixel: Pixel::Dark,
        })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padded_image = ImageEnhancer::pad(self.clone(), 4);

        let image_str = padded_image
            .pixels
            .iter()
            .map(|r| r.iter().map(|p| p.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", image_str)
    }
}

impl Image {
    fn get_pixel(&self, row: isize, col: isize) -> &Pixel {
        if row < 0 || row >= self.nrows as isize || col < 0 || col >= self.ncols as isize {
            &self.infinity_pixel
        } else {
            &self.pixels[row as usize][col as usize]
        }
    }

    fn get_light_pixels_count(&self) -> usize {
        self.pixels
            .iter()
            .flatten()
            .filter(|&&p| matches!(p, Pixel::Light))
            .count()
    }
}

#[derive(Debug)]
struct ImageEnhancer {
    algorithm: Vec<Pixel>,
}

impl FromStr for ImageEnhancer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let algorithm = s.chars().map(|c| Pixel::from(c)).collect::<Vec<_>>();

        Ok(Self { algorithm })
    }
}

impl ImageEnhancer {
    // Pad the image with a layer of dark pixels, 2 pixels wide
    fn pad(image: Image, width: usize) -> Image {
        let padded_nrows = image.nrows + 2 * width;
        let padded_ncols = image.ncols + 2 * width;

        // Top padding = bottom padding
        let top_padding = vec![vec![image.infinity_pixel; padded_ncols]; width];
        let side_padding = vec![image.infinity_pixel; width];

        // Pad the sides
        let padded_sides = image
            .pixels
            .into_iter()
            .map(|r| {
                let mut padded_row = side_padding.clone();

                // Pad left side
                padded_row.extend(r);

                // Pad right side
                padded_row.extend(side_padding.clone());

                padded_row
            })
            .collect::<Vec<_>>();

        let mut padded_pixels = top_padding.clone();

        // Pad the top
        padded_pixels.extend(padded_sides);

        // Pad the bottom
        padded_pixels.extend(top_padding);

        // Build the new image
        Image {
            nrows: padded_nrows,
            ncols: padded_ncols,
            pixels: padded_pixels,
            infinity_pixel: image.infinity_pixel,
        }
    }

    fn enhance(&self, image: Image) -> Image {
        let padded_image = Self::pad(image, 1);
        let mut enchanced_image = padded_image.clone();

        for i in 0..padded_image.nrows {
            for j in 0..padded_image.ncols {
                let index_str = ((i as isize - 1)..=(i as isize + 1))
                    .map(|r| {
                        ((j as isize - 1)..=(j as isize + 1))
                            .map(|c| match padded_image.get_pixel(r, c) {
                                Pixel::Dark => '0',
                                Pixel::Light => '1',
                            })
                            .collect::<String>()
                    })
                    .collect::<String>();

                let index = usize::from_str_radix(&index_str, 2).unwrap();
                enchanced_image.pixels[i][j] = self.algorithm[index];
            }
        }

        // Update the infinity pixel
        enchanced_image.infinity_pixel = match padded_image.infinity_pixel {
            Pixel::Dark => self.algorithm[0b000000000],
            Pixel::Light => self.algorithm[0b111111111],
        };

        enchanced_image
    }
}

fn parse_input<T>(filename: T) -> io::Result<(ImageEnhancer, Image)>
where
    T: AsRef<Path>,
{
    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    let mut lines_iter = input_buf.lines();

    // Build enhancer from first line
    let first_line = lines_iter
        .next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Empty file"))??;
    let enhancer = ImageEnhancer::from_str(&first_line)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // Build image from the rest of the lines
    let image_str = lines_iter
        .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let image =
        Image::from_str(&image_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok((enhancer, image))
}

fn part1(mut image: Image, enhancer: &ImageEnhancer) -> usize {
    for _ in 0..2 {
        image = enhancer.enhance(image);
    }

    image.get_light_pixels_count()
}

fn part2(mut image: Image, enhancer: &ImageEnhancer) -> usize {
    for _ in 0..50 {
        image = enhancer.enhance(image);
    }

    image.get_light_pixels_count()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (enhancer, image) = parse_input("inputs/day20")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let light_pixels_count_part1 = part1(image.clone(), &enhancer);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let light_pixels_count_part2 = part2(image, &enhancer);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nNumber of light pixels after two enhancements: {}\n",
        part1_time, light_pixels_count_part1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nNumber of light pixels after 50 enhancements: {}\n",
        part2_time, light_pixels_count_part2
    );

    Ok(())
}
