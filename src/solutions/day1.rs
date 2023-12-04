use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(raw_input: &str) -> Vec<String> {
    return raw_input.split("\n").map(|s| { s.to_string() }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(readings: &[String]) -> i32 {
    return readings.iter().map(|s| {
        let mut ans: [char; 2] = [' ', ' '];
        let mut i = 0;
        for c in s.chars() {
            if c.is_digit(10) {
                ans[i] = c;
                if i < 1 {
                    i += 1;
                }
            }
        }
        if ans[1] == ' ' {
            ans[1] = ans[0];
        }
        ans.iter().collect::<String>().trim().parse::<i32>().unwrap()
    }).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(_readings: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_generator() {
        let expected = vec![0; 1];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<usize> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 0);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<usize> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 0);
    }
}
