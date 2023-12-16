use std::collections::{hash_map::Entry, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rock {
    Round,
    Square,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[aoc_generator(day14)]
pub fn generator(raw_input: &str) -> Vec<Vec<Option<Rock>>> {
    raw_input
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Square),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn shift(mut board: Vec<Vec<Option<Rock>>>, direction: Direction) -> Vec<Vec<Option<Rock>>> {
    let outer_loop_range: Vec<usize> = match direction {
        Direction::North | Direction::South => (0..board.first().unwrap().len()).collect(),
        Direction::East | Direction::West => (0..board.len()).collect(),
    };
    let inner_loop_range: Vec<usize> = match direction {
        Direction::North => (0..board.len()).collect(),
        Direction::South => (0..board.len()).rev().collect(),
        Direction::West => (0..board.first().unwrap().len()).collect(),
        Direction::East => (0..board.first().unwrap().len()).rev().collect(),
    };

    for i in &outer_loop_range {
        let mut loc = *inner_loop_range.first().unwrap();
        for j in &inner_loop_range {
            let first_ind = match direction {
                Direction::North | Direction::South => *j,
                Direction::East | Direction::West => *i,
            };
            let second_ind = match direction {
                Direction::North | Direction::South => *i,
                Direction::East | Direction::West => *j,
            };
            let swap_first_ind = match direction {
                Direction::North | Direction::South => loc,
                Direction::East | Direction::West => *i,
            };
            let swap_second_ind = match direction {
                Direction::North | Direction::South => *i,
                Direction::East | Direction::West => loc,
            };

            match board[first_ind][second_ind] {
                Some(Rock::Square) => {
                    loc = match direction {
                        Direction::North | Direction::West => j + 1,
                        Direction::South | Direction::East => j.saturating_sub(1),
                    }
                }
                Some(Rock::Round) => {
                    let tmp = *&board[swap_first_ind][swap_second_ind];
                    board[swap_first_ind][swap_second_ind] = *&board[first_ind][second_ind];
                    board[first_ind][second_ind] = tmp;
                    loc = match direction {
                        Direction::North | Direction::West => loc + 1,
                        Direction::South | Direction::East => loc.saturating_sub(1),
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }
    board
}

fn cycle(mut board: Vec<Vec<Option<Rock>>>) -> Vec<Vec<Option<Rock>>> {
    board = shift(board, Direction::North);
    board = shift(board, Direction::West);
    board = shift(board, Direction::South);
    shift(board, Direction::East)
}

fn weight_on_north_beam(board: &[Vec<Option<Rock>>]) -> usize {
    board
        .iter()
        .enumerate()
        .map(|(row_ind, c)| {
            let weight = board.len() - row_ind;
            c.iter()
                .filter(|space| space.is_some() && space.unwrap() == Rock::Round)
                .count()
                * weight
        })
        .sum()
}

fn print(board: &[Vec<Option<Rock>>]) -> String {
    board
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| match c {
                    Some(Rock::Round) => 'O',
                    Some(Rock::Square) => '#',
                    None => '.',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[aoc(day14, part1)]
pub fn solve_part1(board: &[Vec<Option<Rock>>]) -> usize {
    let board_mut = shift(board.to_vec(), Direction::North);
    weight_on_north_beam(&board_mut)
}

#[aoc(day14, part2)]
pub fn solve_part2(board: &[Vec<Option<Rock>>]) -> usize {
    let mut mut_board = board.to_vec();
    let mut boards = HashMap::<String, usize>::new();
    boards.insert(print(&mut_board), 0);
    for i in 1..1_000_000_001 {
        mut_board = cycle(mut_board);
        match boards.entry(print(&mut_board)) {
            Entry::Occupied(idx) => {
                let loop_length = i - idx.get();
                let remaining_cycles = 1_000_000_000 - i;
                for _ in 0..(remaining_cycles % loop_length) {
                    mut_board = cycle(mut_board);
                }
                // println!("{}", print(&mut_board));
                return weight_on_north_beam(&mut_board);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        boards.insert(print(&mut_board), i);
    }
    panic!("should never reach here");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 136);
    }

    #[test]
    fn test_one_shift() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&shift(example, Direction::North)),
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );
    }

    #[test]
    fn test_two_shifts() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&shift(shift(example, Direction::North), Direction::West)),
            "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#...."
        );
    }

    #[test]
    fn test_three_shifts() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&shift(
                shift(shift(example, Direction::North), Direction::West),
                Direction::South
            )),
            ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#...."
        );
    }

    #[test]
    fn test_one_cycle() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&cycle(example)),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );
    }

    #[test]
    fn test_two_cycles() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&cycle(cycle(example))),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
        );
    }

    #[test]
    fn test_three_cycles() {
        let example = generator(&EXAMPLE);
        assert_eq!(
            print(&cycle(cycle(cycle(example)))),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 64);
    }
}

