use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;
use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn generator(raw_input: &str) -> Vec<(i32, Vec<Vec<(String, i8)>>)> {
    raw_input.split("\n")
        .map(|l| {
            let split1: Vec<&str> = l.split(": ").collect();
            let (game, draws_raw_str) = (split1[0], split1[1]);
            let draws = draws_raw_str.split("; ")
                .map(|d| {
                    d.split(", ").map(|c| {
                        let split2: Vec<&str> = c.split(" ").collect();
                        let (count_str, color) = (split2[0], split2[1]);
                        (color.to_string(), count_str.parse::<i8>().unwrap())
                    }).collect()
                }).collect();
            (game.split(" ").last().unwrap().parse::<i32>().unwrap(), draws)
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(readings: &[(i32, Vec<Vec<(String, i8)>>)]) -> i32 {
    let available_stones = {
        let mut map = HashMap::<String, i8>::new();
        map.insert("red".to_string(), 12);
        map.insert("green".to_string(), 13);
        map.insert("blue".to_string(), 14);
        map
    };
    readings.iter()
        .filter(|(_, rounds)| {
            let mut ans = true;
            for draws in rounds {
                for d in draws {
                    match available_stones.get(&d.0) {
                        Some(count) => {
                            if count < &d.1 {
                                ans = false;
                                break;
                            }
                        }
                        None => {
                            panic!("Invalid stone type {:?}", d.0);
                        }
                    }
                }
            }
            ans
        }).map(|(gamenum, _)| { gamenum })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(readings:  &[(i32, Vec<Vec<(String, i8)>>)]) -> usize {
    let mut available_stones = HashMap::<String, usize>::with_capacity(3);
    readings.iter()
        .map(|(_, rounds)| {
            available_stones.clear();
            for draws in rounds {
                for d in draws {
                    available_stones.entry(d.clone().0)
                        .and_modify(|v| *v = cmp::max(*v, d.1 as usize))
                        .or_insert(d.1 as usize);
                }
            }
            let mut ans = 1;
            for v in available_stones.values() {
                ans *= v;
            }
            ans
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 8);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 2286);
    }
}
