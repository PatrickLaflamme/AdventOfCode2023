use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn generator(raw_input: &str) -> Vec<(usize, usize)> {
    raw_input
        .split("\n")
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| match c {
                '#' => Some((j, i)),
                _ => None,
            })
        })
        .collect()
}

pub fn expand_and_get_distances(readings: &[(usize, usize)], expansion_factor: usize) -> usize {
    let mut readings_mut = readings.to_vec();
    let galaxy_cols = readings_mut.iter().map(|g| g.0).collect::<HashSet<_>>();
    let cols = (0..readings.iter().map(|it| it.0).max().unwrap())
        .filter(|c| !galaxy_cols.contains(c))
        .collect::<Vec<_>>();
    readings_mut.sort();
    let mut cols_idx = 0;
    let mut readings_mut_expanded_cols: Vec<(usize, usize)> = readings_mut
        .iter()
        .map(|g| {
            if cols_idx < cols.len() {
                let mut col = cols[cols_idx];
                while g.0 > col {
                    cols_idx += 1;
                    if cols_idx >= cols.len() {
                        break;
                    }
                    col = cols[cols_idx]
                }
            }
            (g.0 + (cols_idx * (expansion_factor - 1)), g.1)
        })
        .collect();
    let galaxy_rows = readings_mut.iter().map(|g| g.1).collect::<HashSet<_>>();
    let rows = (0..readings.iter().map(|it| it.1).max().unwrap())
        .filter(|r| !galaxy_rows.contains(r))
        .collect::<Vec<_>>();
    readings_mut_expanded_cols.sort_by(|a, b| a.1.cmp(&b.1));
    let mut rows_idx = 0;
    let readings_expanded: Vec<(usize, usize)> = readings_mut_expanded_cols
        .iter()
        .map(|g| {
            if rows_idx < rows.len() {
                let mut row = rows[rows_idx];
                while g.1 > row {
                    rows_idx += 1;
                    if rows_idx >= rows.len() {
                        break;
                    }
                    row = rows[rows_idx];
                }
            }
            (g.0, g.1 + (rows_idx * (expansion_factor - 1)))
        })
        .collect();
    let mut sum_distance = 0;
    for i in 0..readings_expanded.len() {
        let g_i = readings_expanded[i];
        for j in (i + 1)..readings_expanded.len() {
            let g_j = readings_expanded[j];
            sum_distance += g_i.0.abs_diff(g_j.0) + g_i.1.abs_diff(g_j.1);
        }
    }
    sum_distance
}

#[aoc(day11, part1)]
pub fn solve_part1(readings: &[(usize, usize)]) -> usize {
    expand_and_get_distances(readings, 2)
}

#[aoc(day11, part2)]
pub fn solve_part2(readings: &[(usize, usize)]) -> usize {
    expand_and_get_distances(readings, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(expand_and_get_distances(&example, 2), 374);
    }

    #[test]
    fn test_solve_part2a() {
        let example = generator(&EXAMPLE);
        assert_eq!(expand_and_get_distances(&example, 10), 1030);
    }

    #[test]
    fn test_solve_part2b() {
        let example = generator(&EXAMPLE);
        assert_eq!(expand_and_get_distances(&example, 100), 8410);
    }
}

