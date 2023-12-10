use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;
use regex::Regex;
use std::collections::HashMap;

pub fn gcd<T: Integer + Copy>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T: Integer + Copy>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

pub fn get_path_length(
    node: String,
    directions: &Vec<Direction>,
    nodes: &HashMap<String, (String, String)>,
    end_condition: fn(s: &String) -> bool,
) -> usize {
    let mut cur_node = node.clone();
    let mut i = 0;
    for d in directions.iter().cycle() {
        i += 1;
        cur_node = match d {
            Direction::Left => nodes.get(&cur_node).unwrap().0.clone(),
            Direction::Right => nodes.get(&cur_node).unwrap().1.clone(),
        };
        if end_condition(&cur_node) {
            return i;
        }
    }
    i
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[aoc_generator(day8)]
pub fn generator(raw_input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let parts = raw_input.split("\n\n").collect::<Vec<_>>();
    let directions = parts[0]
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction char {}", c),
        })
        .collect::<Vec<_>>();
    let mut map = HashMap::<String, (String, String)>::new();
    let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    parts[1].split("\n").for_each(|line| {
        let line_parts = re.captures(line).unwrap();
        map.insert(
            line_parts[1].to_string(),
            (line_parts[2].to_string(), line_parts[3].to_string()),
        );
    });
    (directions, map)
}

#[aoc(day8, part1)]
pub fn solve_part1(readings: &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    let (directions, nodes) = readings;
    get_path_length("AAA".to_string(), directions, nodes, |s| s == "ZZZ")
}

#[aoc(day8, part2)]
pub fn solve_part2(readings: &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    let (directions, nodes) = readings;
    nodes
        .keys()
        .into_iter()
        .filter(|n| n.ends_with("A"))
        .map(|n| get_path_length(n.clone(), directions, nodes, |s| s.ends_with("Z")))
        .reduce(lcm)
        .expect("No soluiton exists")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_solve_part1a() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 2);
    }

    #[test]
    fn test_solve_part1b() {
        let example = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 6);
    }
    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE3);
        assert_eq!(solve_part2(&example), 6);
    }
}

