use aoc_runner_derive::{aoc, aoc_generator};

fn forecast(values: &Vec<isize>, aggregator: fn(isize, &Vec<isize>) -> isize) -> isize {
    let mut derivatives = vec![values.clone()];
    while derivatives
        .last()
        .unwrap()
        .iter()
        .map(|n| n.abs())
        .sum::<isize>()
        > 0
    {
        let new_derivative = derivatives
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        derivatives.push(new_derivative);
    }
    derivatives.reverse();
    derivatives.iter().fold(0, aggregator)
}

#[aoc_generator(day9)]
pub fn generator(raw_input: &str) -> Vec<Vec<isize>> {
    raw_input
        .split("\n")
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(readings: &[Vec<isize>]) -> isize {
    readings
        .iter()
        .map(|n| forecast(n, |acc, vec| acc + *vec.last().unwrap()))
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(readings: &[Vec<isize>]) -> isize {
    readings
        .iter()
        .map(|n| forecast(n, |acc, vec| *vec.first().unwrap() - acc))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 114);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 2);
    }
}

