use std::collections::LinkedList;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

// Illegal scores
static ILLEGAL_PARENTHESIS_SCORE: usize = 3;
static ILLEGAL_SQUARE_BRACKET_SCORE: usize = 57;
static ILLEGAL_BRACE_SCORE: usize = 1197;
static ILLEGAL_ANGLED_BRACKET_SCORE: usize = 25137;

// Completion scores
static PARENTHESIS_COMPLETION_POINTS: usize = 1;
static SQUARE_BRACKET_COMPLETION_POINTS: usize = 2;
static BRACE_COMPLETION_POINTS: usize = 3;
static ANGLED_BRACKET_COMPLETION_POINTS: usize = 4;

fn parse_input<T>(filename: T) -> io::Result<Vec<Vec<char>>>
where
    T: AsRef<Path>,
{
    let mut syntax_lines = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    for line_result in input_buf.lines() {
        let line = line_result?;

        let syntax_line: Vec<char> = line.chars().collect();
        syntax_lines.push(syntax_line);
    }

    Ok(syntax_lines)
}

fn part1(syntax_lines: &[Vec<char>]) -> usize {
    // Score for part 1
    let mut syntax_error_score = 0;

    for syntax_line in syntax_lines {
        let mut syntax_stack = LinkedList::new();
        for character in syntax_line {
            match *character {
                // Opening characters
                '(' => syntax_stack.push_back('('),
                '[' => syntax_stack.push_back('['),
                '{' => syntax_stack.push_back('{'),
                '<' => syntax_stack.push_back('<'),

                // Closing characters
                ')' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '(' {
                        syntax_error_score += ILLEGAL_PARENTHESIS_SCORE;
                        break;
                    }
                }
                ']' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '[' {
                        syntax_error_score += ILLEGAL_SQUARE_BRACKET_SCORE;
                        break;
                    }
                }
                '}' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '{' {
                        syntax_error_score += ILLEGAL_BRACE_SCORE;
                        break;
                    }
                }
                '>' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '<' {
                        syntax_error_score += ILLEGAL_ANGLED_BRACKET_SCORE;
                        break;
                    }
                }

                c => panic!("Invalid character '{}'", c),
            };
        }
    }

    syntax_error_score
}

fn part2(syntax_lines: &[Vec<char>]) -> usize {
    // Score for part 1
    let mut completion_scores = Vec::new();

    for syntax_line in syntax_lines {
        let mut syntax_stack = LinkedList::new();
        for character in syntax_line {
            match *character {
                // Opening characters
                '(' => syntax_stack.push_back('('),
                '[' => syntax_stack.push_back('['),
                '{' => syntax_stack.push_back('{'),
                '<' => syntax_stack.push_back('<'),

                // Closing characters
                ')' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '(' {
                        syntax_stack.clear();
                        break;
                    }
                }
                ']' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '[' {
                        syntax_stack.clear();
                        break;
                    }
                }
                '}' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '{' {
                        syntax_stack.clear();
                        break;
                    }
                }
                '>' => {
                    let stack_top = syntax_stack.pop_back().unwrap();
                    if stack_top != '<' {
                        syntax_stack.clear();
                        break;
                    }
                }

                c => panic!("Invalid character '{}'", c),
            };
        }

        if syntax_stack.is_empty() {
            continue;
        }

        let mut completion_score = 0;
        while let Some(character) = syntax_stack.pop_back() {
            match character {
                '(' => completion_score = completion_score * 5 + PARENTHESIS_COMPLETION_POINTS,
                '[' => completion_score = completion_score * 5 + SQUARE_BRACKET_COMPLETION_POINTS,
                '{' => completion_score = completion_score * 5 + BRACE_COMPLETION_POINTS,
                '<' => completion_score = completion_score * 5 + ANGLED_BRACKET_COMPLETION_POINTS,
                // Will never happen
                c => panic!("Invalid character '{}'", c),
            };
        }
        completion_scores.push(completion_score);
    }

    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let syntax_lines = parse_input("inputs/day10")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let syntax_error_score = part1(&syntax_lines);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let middle_completion_score = part2(&syntax_lines);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {:.9}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {:.9}s\nSyntax Error Score: {}\n",
        part1_time, syntax_error_score
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {:.9}s\nMiddle Completion Score: {}\n",
        part2_time, middle_completion_score
    );

    Ok(())
}
