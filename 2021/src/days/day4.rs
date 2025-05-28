use std::{collections::HashSet, path::Path};

use crate::helpers::file::read_lines;

#[derive(Debug, Clone)]
struct BingoNum {
    value: i64,
    is_marked: bool,
}

struct Board {
    rows: Vec<Vec<BingoNum>>,
}

impl Board {
    fn new(lines: &[String]) -> Vec<Self> {
        let mut boards: Vec<Board> = vec![Board { rows: vec![] }];
        let mut curr_board = 0;

        for line in lines.iter() {
            if line.is_empty() {
                curr_board += 1;
                boards.push(Board { rows: vec![] });
                continue;
            }

            let nums_row_in_board: Vec<BingoNum> = line
                .split_whitespace()
                .map(|item| BingoNum {
                    value: item.parse::<i64>().unwrap(),
                    is_marked: false,
                })
                .collect();

            boards[curr_board].rows.push(nums_row_in_board);
        }

        return boards;
    }
}

// https://adventofcode.com/2021/day/4
pub fn solve() {
    let path = Path::new("src/data/04.txt");
    let lines = read_lines(path.to_str().unwrap());

    let res1 = part1(&lines);
    println!("Part 1: {:?}", res1);

    let res2 = part2(&lines);
    println!("Part 2: {:?}", res2);
}

fn part1(lines: &Vec<String>) -> Option<i64> {
    return calculate(lines, false);
}

fn part2(lines: &Vec<String>) -> Option<i64> {
    return calculate(lines, true);
}

fn calculate(lines: &Vec<String>, is_part2: bool) -> Option<i64> {
    let winning_nums: Vec<i64> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut boards = Board::new(&lines[2..]);

    let mut last_winning_board = None;
    let mut last_winning_num = 0;
    let mut boards_that_won: HashSet<usize> = HashSet::new();

    for winning_num in winning_nums {
        // mark winning number in all boards
        for (board_idx, board) in &mut boards.iter_mut().enumerate() {
            for (_, row) in board.rows.iter_mut().enumerate() {
                for (_, board_item) in row.iter_mut().enumerate() {
                    if board_item.value == winning_num {
                        board_item.is_marked = true;
                    }
                }

                // check winning row
                if row.iter().all(|board_item| board_item.is_marked) {
                    let res = calculate_unmarked(&mut board.rows) * winning_num;

                    if is_part2 && !boards_that_won.contains(&board_idx) {
                        last_winning_board = Some(board.rows.clone());
                        last_winning_num = winning_num;
                        boards_that_won.insert(board_idx);
                        break;
                    }

                    if !is_part2 {
                        return Some(res);
                    } else {
                        break;
                    }
                }
            }

            // check winning column
            if is_whole_column_marked(&mut board.rows) {
                let res = calculate_unmarked(&mut board.rows) * winning_num;

                if is_part2 && !boards_that_won.contains(&board_idx) {
                    last_winning_board = Some(board.rows.clone());
                    last_winning_num = winning_num;
                    boards_that_won.insert(board_idx);
                }

                if !is_part2 {
                    return Some(res);
                }
            }
        }
    }

    if is_part2 {
        let res = calculate_unmarked(&mut last_winning_board.unwrap()) * last_winning_num;
        return Some(res);
    }

    None
}

fn is_whole_column_marked(board: &mut Vec<Vec<BingoNum>>) -> bool {
    let n = board[0].len();
    let m = board.len();

    for column in 0..n {
        let mut count = 0;

        for row in 0..m {
            if board[row][column].is_marked {
                count += 1;
            }
        }

        if count == n {
            return true;
        }
    }

    false
}

fn calculate_unmarked(board: &mut Vec<Vec<BingoNum>>) -> i64 {
    let mut res = 0;
    let m = board.len();
    let n = board[0].len();

    for row in 0..m {
        for column in 0..n {
            if !board[row][column].is_marked {
                res += board[row][column].value;
            }
        }
    }

    res
}
