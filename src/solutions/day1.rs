use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

lazy_static! {
    static ref TRIE: TrieNode = {
        let mut trie: TrieNode = TrieNode::new();
        trie.push("1", '1');
        trie.push("2", '2');
        trie.push("3", '3');
        trie.push("4", '4');
        trie.push("5", '5');
        trie.push("6", '6');
        trie.push("7", '7');
        trie.push("8", '8');
        trie.push("9", '9');
        trie.push("one", '1');
        trie.push("two", '2');
        trie.push("three", '3');
        trie.push("four", '4');
        trie.push("five", '5');
        trie.push("six", '6');
        trie.push("seven", '7');
        trie.push("eight", '8');
        trie.push("nine", '9');
        trie
    };
}

#[derive(Clone, Eq, PartialEq)]
struct TrieNode {
    digit: Option<char>,
    children: HashMap<char, TrieNode>,
}

#[derive(Debug)]
pub enum NoMatch {
    NoMatch,
}

impl TrieNode {
    fn new() -> TrieNode {
        return TrieNode {
            digit: None,
            children: HashMap::new(),
        };
    }

    fn push(&mut self, word: &str, value: char) {
        let mut trie = self;
        for c in word.chars() {
            trie = trie.children.entry(c).or_insert(TrieNode::new());
        }
        trie.digit = Some(value);
    }

    fn digit_of(&self, word: &str) -> Result<Option<char>, NoMatch> {
        let mut trie = self.clone();
        for c in word.chars() {
            match trie.children.entry(c) {
                Entry::Occupied(o) => {
                    trie = o.get().clone();
                }
                Entry::Vacant(_) => return Err(NoMatch::NoMatch),
            }
        }
        return Ok(trie.digit);
    }
}

#[aoc_generator(day1)]
pub fn generator(raw_input: &str) -> Vec<String> {
    return raw_input.split("\n").map(|s| s.to_string()).collect();
}

#[aoc(day1, part1)]
pub fn solve_part1(readings: &[String]) -> i32 {
    return readings
        .iter()
        .map(|s| {
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
            ans.iter()
                .collect::<String>()
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .sum();
}

#[aoc(day1, part2)]
pub fn solve_part2(readings: &[String]) -> i32 {
    return readings
        .iter()
        .map(|s| {
            let mut ans: [char; 2] = [' ', ' '];
            let mut i = 0;
            let mut b = 0;
            let mut e = 1;
            while b < s.len() {
                if e > s.len() {
                    b += 1;
                    e = b + 1;
                    continue;
                }
                let slice = &s[b..e];
                match TRIE.digit_of(slice) {
                    Err(_) => {
                        b += 1;
                        e = b + 1;
                    }
                    Ok(resp) => match resp {
                        Some(digit) => {
                            ans[i] = digit;
                            if i == 0 {
                                i += 1
                            }
                            b += 1;
                            e = b + 1;
                            continue;
                        }
                        None => {
                            e += 1;
                            continue;
                        }
                    },
                }
            }
            if ans[1] == ' ' {
                ans[1] = ans[0];
            }
            ans.iter()
                .collect::<String>()
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_solve_part1() {
        let example: Vec<String> = generator(&EXAMPLE1);
        assert_eq!(solve_part1(&example), 142);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<String> = generator(&EXAMPLE2);
        assert_eq!(solve_part2(&example), 281);
    }
}
