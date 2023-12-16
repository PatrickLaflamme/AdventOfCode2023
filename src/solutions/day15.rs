use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day15)]
pub fn generator(raw_input: &str) -> Vec<String> {
    raw_input.split(",").map(|s| s.to_string()).collect()
}

fn hash(seq: &str) -> usize {
    seq.chars()
        .map(|c| c as u8)
        .fold(0 as usize, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
pub fn solve_part1(readings: &[String]) -> usize {
    readings.iter().map(|seq| hash(&seq)).sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(instructions: &[String]) -> usize {
    let mut map = (0..256)
        .map(|_| Vec::<(&str, u8)>::new())
        .collect::<Vec<_>>();
    let matcher = Regex::new(r"([a-zA-Z]+)([=-])([1-9]{0,1})").unwrap();
    instructions.iter().for_each(|i| {
        let (_, label, operation, focal_length) = matcher
            .captures(i)
            .unwrap()
            .iter()
            .map(|m| m.unwrap().as_str())
            .collect_tuple()
            .unwrap();
        let box_num = hash(label);
        match operation {
            "=" => {
                let existing = map[box_num].iter().find_position(|(lab, _)| lab == &label);
                match existing {
                    Some((idx, _)) => map[box_num][idx] = (label, focal_length.parse().unwrap()),
                    None => map[box_num].push((label, focal_length.parse().unwrap())),
                }
            }
            "-" => {
                let existing = map[box_num].iter().find_position(|(lab, _)| lab == &label);
                match existing {
                    Some((idx, _)) => {
                        map[box_num].remove(idx);
                    }
                    None => {}
                }
            }
            _ => panic!("Invalid operation: {}", operation),
        }
    });
    map.iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, l))| (i + 1) * (j + 1) * (*l as usize))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash(&"HASH"), 52);
    }

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 1320);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 145);
    }
}

