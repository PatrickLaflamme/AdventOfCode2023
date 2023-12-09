use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::cmp::Ordering;

lazy_static!{
    static ref CARD_VALUES: HashMap<char, u8> = {
        let mut m = HashMap::<char, u8>::with_capacity(13);
        (2..10).for_each(|i| {
            m.insert(i.to_string().chars().collect::<Vec<_>>()[0], i);
        });
        m.insert('T', 10);
        m.insert('J', 11);
        m.insert('Q', 12);
        m.insert('K', 13);
        m.insert('A', 14);
        m
    };
}

#[aoc_generator(day7)]
pub fn generator(raw_input: &str) -> Vec<(String, usize)> {
    raw_input.split("\n")
        .map(|h| {
            let details= h.split(" ").collect::<Vec<_>>();
            let hand = details[0];
            let bid = details[1].parse::<usize>().unwrap();
            (hand.to_string(), bid)
        }).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(readings: &[(String, usize)]) -> usize {
    let mut map = HashMap::<char, u8>::with_capacity(13);
    let mut mut_readings: Vec<(usize, &String, &usize)> = readings.iter().map(|(hand, bid)| {
        map.clear();
        hand.chars().for_each(|c| {
            map.entry(c)
               .and_modify(|e| { *e += 1 })
               .or_insert(1);
        });
        let mut counts = map.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        let hand_type = match counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts[1] {
                2 => 5,
                1 => 4,
                _ => panic!("Invalid! {:?}", counts)
            },
            2 => match counts[1] {
                2 => 3,
                1 => 2,
                _ => panic!("Invalid! {:?}", counts)
            },
           1 => 1,
            _ => panic!("Invalid! {:?}", counts)
        };
        (hand_type, hand, bid)
    }).collect::<Vec<_>>();
    mut_readings.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Equal => {
                let mut a_chars = a.1.chars();
                let mut ans: Ordering = Ordering::Equal;
                for b_char in b.1.chars() {
                    let a_char = CARD_VALUES.get(&a_chars.next().unwrap()).unwrap();
                    match a_char.cmp(CARD_VALUES.get(&b_char).unwrap()) {
                        Ordering::Equal => continue,
                        or => {
                            ans = or;
                            break;
                        }
                    }
                }
                ans
            },
            or => or
        }
    });
    mut_readings.iter().enumerate().map(|(i, (_, _, &bid))| {
        (i + 1) * bid
    }).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(readings: &[(String, usize)]) -> usize {
    let mut map = HashMap::<char, u8>::with_capacity(13);
    let mut mut_readings: Vec<(usize, &String, &usize)> = readings.iter().map(|(hand, bid)| {
        map.clear();
        hand.chars().for_each(|c| {
            map.entry(c)
               .and_modify(|e| { *e += 1 })
               .or_insert(1);
        });
        let n_js = *map.get(&'J').unwrap_or(&0) as usize;
        if map.len() > 1 && n_js > 0 {
            map.remove(&'J');
        }
        let mut counts = map.values().map(|i| { *i }).collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        for _ in 0..n_js {
            if counts[0] < 5 {
                let new = counts[0] + 1;
                let _ = std::mem::replace(&mut counts[0], new);
            } else if counts.len() > 1 {
                let new = counts[1] + 1;
                let _ = std::mem::replace(&mut counts[1], new);
            }
        }
        let hand_type = match counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts[1] {
                2 => 5,
                1 => 4,
                _ => panic!("Invalid! {:?}", counts)
            },
            2 => match counts[1] {
                2 => 3,
                1 => 2,
                _ => panic!("Invalid! {:?}", counts)
            },
           1 => 1,
            _ => panic!("Invalid! {:?}", counts)
        };
        (hand_type, hand, bid)
    }).collect::<Vec<_>>();
    let new_card_values = {
        let mut c = CARD_VALUES.clone();
        c.insert('J', 0);
        c
    };
    mut_readings.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Equal => {
                let mut a_chars = a.1.chars();
                let mut ans: Ordering = Ordering::Equal;
                for b_char in b.1.chars() {
                    let a_char = new_card_values.get(&a_chars.next().unwrap()).unwrap();
                    match a_char.cmp(new_card_values.get(&b_char).unwrap()) {
                        Ordering::Equal => continue,
                        or => {
                            ans = or;
                            break;
                        }
                    }
                }
                ans
            },
            or => or
        }
    });
    mut_readings.iter().enumerate().map(|(i, (_, _, &bid))| {
        (i + 1) * bid
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 6440);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 5905);
    }
}
