use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn generator(raw_input: &str) -> Vec<Vec<Vec<u8>>> {
    raw_input
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .map(|l| l.chars().map(|c| c as u8).collect())
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum ReflectionValue {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflection_value(block: &[Vec<u8>], with_smudge: bool) -> ReflectionValue {
    let mut verticals = Vec::<ReflectionValue>::new();
    let mut horizontals = Vec::<ReflectionValue>::new();
    for i in 1..block.len() {
        let mut t = i - 1;
        let mut b = i;
        let mut smudge_used = !with_smudge;
        loop {
            if block[t] != block[b] {
                if smudge_used {
                    break;
                }
                let num_diffs = block[t]
                    .iter()
                    .zip(block[b].iter())
                    .filter(|(t_v, b_v)| t_v != b_v)
                    .count();
                if num_diffs > 1 {
                    break;
                }
                smudge_used = true;
            }
            if t == 0 || b == (block.len() - 1) {
                if (smudge_used && with_smudge) || !with_smudge {
                    horizontals.push(ReflectionValue::Horizontal(i));
                }
                break;
            }
            t -= 1;
            b += 1;
        }
    }
    for i in 1..block.first().unwrap().len() {
        let mut l = i - 1;
        let mut r = i;
        let mut smudge_used = !with_smudge;
        loop {
            let are_equal = block.iter().all(|row| row[l] == row[r]);
            if !are_equal {
                if smudge_used {
                    break;
                }
                let num_diffs = block.iter().filter(|row| row[l] != row[r]).count();
                if num_diffs > 1 {
                    break;
                }
                smudge_used = true;
            }
            if l == 0 || r == (block.first().unwrap().len() - 1) {
                if (smudge_used && with_smudge) || !with_smudge {
                    verticals.push(ReflectionValue::Vertical(i));
                }
                break;
            }
            l -= 1;
            r += 1;
        }
    }
    *verticals.first().or(horizontals.first()).expect(&format!(
        "No Reflections Found for block: \n {}",
        block
            .iter()
            .map(|r| r.iter().map(|u| *u as char).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    ))
}

#[aoc(day13, part1)]
pub fn solve_part1(blocks: &[Vec<Vec<u8>>]) -> usize {
    blocks
        .iter()
        .map(|b| match find_reflection_value(b, false) {
            ReflectionValue::Horizontal(v) => 100 * v,
            ReflectionValue::Vertical(v) => v,
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(blocks: &[Vec<Vec<u8>>]) -> usize {
    blocks
        .iter()
        .map(|b| match find_reflection_value(b, true) {
            ReflectionValue::Horizontal(v) => 100 * v,
            ReflectionValue::Vertical(v) => v,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 405);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 400);
    }
}

