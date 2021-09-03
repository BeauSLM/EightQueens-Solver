extern crate termion;

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board: [[bool; 8]; 8];

    if args.len() > 1 {
        let filename: String = env::args().nth(1).unwrap();
        board = populate_from_file(filename);
    } else {
        board = [[false; 8]; 8];
    }

    solve_eightqueens(&mut board);
    print_board(board);
}

fn solve_eightqueens(board: &mut [[bool; 8]; 8]) -> [[bool; 8]; 8] {
    if count_queens(*board) == 8 {
        return *board;
    }

    for row in 0..8 {
        for column in 0..8 {
            if check_if_placeable(*board, row, column) {
                board[row][column] = true;

                let temp = solve_eightqueens(board);
                if count_queens(temp) == 0 {
                    return temp;
                }

                board[row][column] = false;
            }
        }
    }

    *board
}

fn check_if_placeable(board: [[bool; 8]; 8], row: usize, column: usize) -> bool {
    if board[row][column] {
        return false;
    }
    
    for i in 0..8 {
        for j in 0..8 {
            // empty squares or the actual square we're checking are uninteresting to this function
            if (i == row && j == column) || !board[i][j] {
                continue;
            }

            // any queens on the same row/diagonal as the coordinates given?
            if i == row || j == column || i - j == row - column || i + j == row + column {
                return false;
            }
        }
    }

    true
}

fn count_queens(board: [[bool; 8]; 8]) -> u8 {
    let mut count = 0;
    for row in board {
        for column in row {
            if column {
                count += 1;
            }
        }
    }

    count
}

fn print_board(board: [[bool; 8]; 8]) {
    for row in board {
        println!("{:?}", row);
    }
}

fn populate_from_file(filename: String) -> [[bool; 8]; 8] {
    let mut board = [[false; 8]; 8];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();

        let left = words.next().unwrap();
        let left = left.parse::<usize>().unwrap();

        let right = words.next().unwrap();
        let right = right.parse::<usize>().unwrap();
        
        board[left][right] = true;
    }
    board
}
