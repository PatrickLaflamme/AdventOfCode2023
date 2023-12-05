use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn generator(raw_input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    raw_input
        .split("\n")
        .map(|s| {
            let card = s.split(": ").collect::<Vec<_>>()[1];
            let sections = card.split(" | ").collect::<Vec<_>>();
            let my_numbers = sections[0]
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            let winning_numbers = sections[1]
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            (my_numbers, winning_numbers)
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(readings: &[(HashSet<u8>, HashSet<u8>)]) -> usize {
    readings
        .iter()
        .map(|(my_nums, winners)| {
            let intersection_count = my_nums.intersection(winners).count() as u32;
            if intersection_count == 0 {
                return 0;
            }
            2_usize.pow(intersection_count - 1)
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(readings: &[(HashSet<u8>, HashSet<u8>)]) -> usize {
    let mut card_count = vec![1_usize; readings.len()];
    let mut cur = vec![1_usize; readings.len()];
    let mut next = vec![0_usize; readings.len()];
    while cur.iter().sum::<usize>() > 0 {
        for (i, (my_nums, winners)) in readings.iter().enumerate() {
            let times = cur[i];
            for j in 0..my_nums.intersection(winners).count() {
                next[i + j + 1] += times;
                card_count[i + j + 1] += times;
            }
        }
        let mut tmp = cur;
        cur = next;
        for i in 0..tmp.len() {
            tmp[i] = 0;
        }
        next = tmp;
    }
    card_count.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 13);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 30);
    }
}
