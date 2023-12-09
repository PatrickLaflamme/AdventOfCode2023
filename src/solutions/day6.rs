use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day6)]
pub fn generator(raw_input: &str) -> Vec<(usize, usize)> {
    let rows = raw_input.split("\n").collect::<Vec<_>>();
    let re = Regex::new(r" +").unwrap();
    let times = re.split(rows[0]).collect::<Vec<_>>()[1..].into_iter().map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let distances = re.split(rows[1]).collect::<Vec<_>>()[1..].into_iter().map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
    times.into_iter().zip(distances).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(readings: &[(usize, usize)]) -> usize {
    readings.iter().map(|(record, distance)| {
        let mut t = 0;
        while &t <= record {
            if t*(record - t) > *distance {
                break;
            }
            t += 1;
        }
        record - 2*(t - 1) - 1
    }).fold(1, |acc, i| acc * i)
}

#[aoc(day6, part2)]
pub fn solve_part2(readings: &[(usize, usize)]) -> usize {
    let record = readings.iter().fold(0, |acc, i| acc * 10_usize.pow(i.0.ilog10() + 1) + i.0);
    let distance = readings.iter().fold(0, |acc, i| acc * 10_usize.pow(i.1.ilog10() + 1) + i.1);
    let mut t = 0;
    while t <= record {
        if t*(record - t) > distance {
            break;
        }
        t += 1;
    }
    record - 2*(t - 1) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 288);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 71503);
    }
}
