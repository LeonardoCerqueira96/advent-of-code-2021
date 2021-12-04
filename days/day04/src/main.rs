use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
pub struct BoardSquare {
    number: u8,
    marked: bool,
}

impl BoardSquare {
    fn new(number: u8) -> Self {
        BoardSquare {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Clone)]
pub struct BingoBoard {
    id: usize,
    numbers: Vec<Vec<BoardSquare>>,
    has_won: bool,
}

impl BingoBoard {
    fn new(id: usize, raw_board: Vec<Vec<u8>>) -> Self {
        let mut board = Vec::new();
        for (i, row) in raw_board.into_iter().enumerate() {
            board.push(Vec::new());

            for number in row {
                board[i].push(BoardSquare::new(number))
            }
        }

        BingoBoard {
            id,
            numbers: board,
            has_won: false,
        }
    }

    fn iter_nrow(&self, row_index: usize) -> impl Iterator<Item = &BoardSquare> {
        self.numbers[row_index].iter()
    }

    fn iter_ncolumn(&self, column_index: usize) -> impl Iterator<Item = &BoardSquare> {
        let column: Vec<&BoardSquare> = self.numbers
            .iter()
            .map(|r| &r[column_index])
            .collect();

        column.into_iter()
    }

    fn mark_ball(&mut self, ball: u8) -> Option<(usize, usize)> {
        let mut marked_position = None;
        for (i, row) in self.numbers.iter().enumerate() {
            let drawn_column = row
                .iter()
                .position(|n| n.number == ball);

            if let Some(column) = drawn_column {
                self.numbers[i][column].mark();
                marked_position = Some((i, column));

                break;
            }
        }

        marked_position
    }

    fn check_win_condition(&mut self, row: usize, column: usize) -> bool {
        // Check the row first
        let row_won = self.iter_nrow(row)
            .all(|n| n.marked);

        if row_won {
            self.has_won = true;
        }

        // Row didn't win, check the column
        let column_won = self.iter_ncolumn(column)
            .all(|n| n.marked);

        if column_won {
            self.has_won = true;
        }

        self.has_won
    }
}

pub struct BingoCaller {
    draw_sequence: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoCaller {
    fn new(draw_sequence: Vec<u8>, boards: Vec<BingoBoard>) -> Self {
        BingoCaller {
            draw_sequence,
            boards
        }
    }

    fn draw(&mut self) -> (u8, usize, Option<usize>) {
        let ball = self.draw_sequence.remove(0);

        let mut last_winner_id = None;
        let mut n_winners = 0;
        for board in self.boards.iter_mut() {
            if board.has_won {
                continue;
            }

            let position_option = board.mark_ball(ball);
            if let Some(position) = position_option {
                if board.check_win_condition(position.0, position.1) {
                    last_winner_id = Some(board.id);
                    n_winners += 1;
                }
            }
        }

        (ball, n_winners, last_winner_id)
    }
}

fn parse_input<T>(filename: T) -> io::Result<(Vec<u8>, Vec<Vec<Vec<u8>>>)>
where
    T: AsRef<Path>,
{
    let mut boards = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Setup lines iterator
    let mut lines_iter = input_buf.lines();

    // The first line is the draw sequence
    let draw_sequence = lines_iter.next()
        .expect("File is empty")?
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect();

    // Parse the remaining lines
    for line_result in lines_iter {
        let line = line_result?;

        // If it's an empty line, it means we're going to start reading a new board
        if line.len() == 0 {
            boards.push(Vec::new());
            continue;
        }

        // Get mutable reference to current board
        let current_board = boards.last_mut().unwrap();
        
        // Build and push new row
        let new_row = line.split_ascii_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();
        current_board.push(new_row);
    }

    Ok((draw_sequence, boards))
}

fn part1(draw_sequence: &[u8], raw_boards: &[Vec<Vec<u8>>]) -> (u8, usize, usize) {
    // Build our objects
    let mut boards = Vec::new();
    for (i, raw_board) in raw_boards.into_iter().enumerate() {
        boards.push(BingoBoard::new(i, raw_board.to_vec()));
    }

    let mut bingo_caller = BingoCaller::new(draw_sequence.to_vec(), boards);

    // Run the game until a board wins
    let last_ball: u8;
    let winner_board_id: usize;
    loop {
        if let (drawn_ball, _, Some(winner)) = bingo_caller.draw() {
            // Found our winner!
            last_ball = drawn_ball;
            winner_board_id = winner;
            break;
        }
    }
    
    // Sum the unmarked squares
    let unmarked_sum = bingo_caller.boards[winner_board_id].numbers.iter()
        .flatten()
        .fold(0, |acc, s| acc + if s.marked { 0 } else { s.number as usize });
    
    // Calculate final score
    let final_score = unmarked_sum * last_ball as usize;

    (last_ball, winner_board_id, final_score)
}

fn part2 (draw_sequence: &[u8], raw_boards: &[Vec<Vec<u8>>]) -> (u8, usize, usize) {
        // Build our objects
        let mut boards = Vec::new();
        for (i, raw_board) in raw_boards.into_iter().enumerate() {
            boards.push(BingoBoard::new(i, raw_board.to_vec()));
        }
    
        let mut bingo_caller = BingoCaller::new(draw_sequence.to_vec(), boards);

        // Run the game until all the boards win
        let nboards = raw_boards.len();
        let mut nwins = 0;
        let mut last_ball = 0;
        let mut last_winner_board_id = 0;
        while nwins < nboards {
            if let (drawn_ball, nwinners, Some(winner)) = bingo_caller.draw() {
                // Found our winner!
                last_ball = drawn_ball;
                last_winner_board_id = winner;
                nwins += nwinners;
            }
        }

        // Sum the unmarked squares
        let unmarked_sum = bingo_caller.boards[last_winner_board_id].numbers.iter()
            .flatten()
            .fold(0, |acc, s| acc + if s.marked { 0 } else { s.number as usize });

        // Calculate final score
        let final_score = unmarked_sum * last_ball as usize;

        (last_ball, last_winner_board_id, final_score)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (draw_sequence, raw_boards) = parse_input("inputs/day04")?;
    let parse_time = t0.elapsed();
    
    // Compute part 1 and time it
    let t1 = Instant::now();
    let (last_ball_p1, winner_board_id_p1, final_score_p1) = part1(&draw_sequence, &raw_boards);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let (last_ball_p2, winner_board_id_p2, final_score_p2) = part2(&draw_sequence, &raw_boards);
    let part2_time = t2.elapsed();
    
    // Print results
    let parse_time = parse_time.as_secs() as f64 + parse_time.subsec_nanos() as f64 * 1e-9;
    println!("Parsing the input took {}s\n", parse_time);

    let part1_time = part1_time.as_secs() as f64 + part1_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 1:\nTook {}s\nFirst winner board id: {}\nLast ball: {}\nFinal score: {}\n",
        part1_time,
        winner_board_id_p1,
        last_ball_p1,
        final_score_p1
    );

    let part2_time = part2_time.as_secs() as f64 + part2_time.subsec_nanos() as f64 * 1e-9;
    println!(
        "Part 2:\nTook {}s\nLast winner board id: {}\nLast ball: {}\nFinal score: {}\n",
        part2_time,
        winner_board_id_p2,
        last_ball_p2,
        final_score_p2
    );
    
    Ok(())
}
