use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;
use std::ops::Range;

fn intersection(a: Range<i32>, b: Range<i32>) -> Option<(Range<i32>, Range<i32>, Range<i32>)> {
    let intersect = cmp::max(a.start, b.start)..cmp::min(a.end, b.end);
    if intersect.is_empty() {
        return None;
    }
    let before = cmp::min(a.start, b.start)..cmp::min(a.end, cmp::min(b.end, intersect.start));
    let after = cmp::max(intersect.end, b.start)..cmp::max(a.end, b.end);
    if a.start < b.start {
        return Some((before, intersect, after));
    }
    return Some((after, intersect, before));
}

#[aoc_generator(day5)]
pub fn generator(raw_input: &str) -> (Vec<i32>, Vec<(Range<i32>, i32)>) {
    let split = raw_input.split("\n\n").collect::<Vec<_>>();
    let seeds = split[0].split(": ")
        .last()
        .expect("invalid input")
        .split(" ")
        .map(|n| n.parse::<i32>().expect("invalid number"))
        .collect();
    let mut maps = split[1..].iter().map(|s| {
        let mut map = s.split("\n")
            .collect::<Vec<_>>()[1..]
            .iter()
            .map(|m| {
                let entry = m.split(" ")
                    .map(|n| {
                        n.parse::<i32>().expect(&format!("invalid number: {:?}", n))
                    })
                    .collect::<Vec<_>>();
                (entry[1]..(entry[1] + entry[2]), entry[0]..(entry[0] + entry[2]))
            })
            .collect::<Vec<_>>();
        map.sort_by(|a, b| a.0.start.partial_cmp(&b.0.start).expect(&format!("invalid ranges {:?}, {:?}", a, b)));
        map
    }).fold(Vec::<(Range<i32>, Range<i32>)>::new(), |acc, m| {
        let mut new = Vec::new();
        println!("acc: {:?}\nmap: {:?}", acc, m);
        let mut a = acc.iter();
        let mut n = m.iter();
        let mut next_a = a.next();
        let mut next_n = n.next();
        let mut tmp = next_n.unwrap().clone();
        loop {
            if next_a.is_none() {
                while next_n.is_some() {
                    new.push(next_n.unwrap().clone());
                    next_n = n.next();
                }
                break;
            }
            if next_n.is_none() {
                while next_a.is_some() {
                    new.push(next_a.unwrap().clone());
                    next_a = a.next();
                }
                break;
            }
            let maybe_intersection = intersection(next_a.unwrap().0.clone(), next_n.unwrap().0.clone());
            if maybe_intersection.is_none() {
                if next_a.unwrap().clone().0.start > next_n.unwrap().clone().0.start {
                    new.push(next_n.unwrap().clone());
                    next_n = n.next();
                } else {
                    new.push(next_a.unwrap().clone());
                    next_a = a.next();
                } 
                continue;
            }

            let Some((a_part, intersect, b_part)) = maybe_intersection else { continue; };
            if !a_part.is_empty() && a_part.start < b_part.start {
                let next_a_start = next_a.unwrap().clone().0.start;
                new.push(((next_a_start..next_a_start + a_part.len() as i32), a_part.clone()));
                if a_part.end == next_a.unwrap().1.end {
                    next_a = a.next();
                    continue;
                }
            }
            let next_a_unwrapped = next_a.unwrap().clone();
            let next_n_unwrapped = next_n.unwrap().clone();
            let start = intersect.start - next_a_unwrapped.1.start + next_a_unwrapped.0.start;
            let m_start = intersect.start - next_n_unwrapped.0.start + next_n_unwrapped.1.start;
            new.push((start..(start + intersect.len() as i32), m_start..(m_start + intersect.len() as i32)));
            if !b_part.is_empty() {
                next_n = n.next();
            } else {
                let next_n_unwrapped = next_n.unwrap().clone();
                let offset = b_part.start - next_n_unwrapped.0.start;
                tmp = (b_part, (next_n_unwrapped.0.end + offset)..next_n_unwrapped.1.end);
                next_n = Some(&tmp)
            }

        }
        new.sort_by(|a, b| a.0.start.partial_cmp(&b.0.start).expect(&format!("invalid ranges {:?}, {:?}", a, b)));
        new
    }).iter().map(|(source_range, dest_range)| {
        (source_range.clone(), dest_range.start - source_range.start)
    }).collect::<Vec<_>>();
    maps.sort_by(|a, b| a.0.start.partial_cmp(&b.0.start).expect(&format!("invalid ranges {:?}, {:?}", a, b)));
    println!("{:?}", maps);
    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn solve_part1(readings: &(Vec<i32>, Vec<(Range<i32>, i32)>)) -> i32 {
    let (seeds, m) = readings;
    seeds.iter().map(|s| {
        let mut ans = *s;
        let mut upper = m.len();
        let mut lower = 0;
        loop {
            let idx = (upper + lower) / 2;
            let (range, translation) = &m[idx];
            if range.start > ans {
                if upper == 0 {
                    break;
                }
                upper = idx;
                if lower >= upper {
                    break;
                }
                continue;
            }
            if range.end <= ans {
                lower = idx + 1;
                if lower >= upper {
                    break;
                }
                continue;
            }
            ans = s + translation;
            break;
        }
        ans
    }).min()
    .expect("Invalid seeds")
}

#[aoc(day5, part2)]
pub fn solve_part2(readings: &(Vec<i32>, Vec<(Range<i32>, i32)>)) -> i32 {
    let (seed_ranges, maps) = readings;
    let mut min_location = i32::MAX;
    for i in 0..seed_ranges.len() / 2 {
        let start = seed_ranges[i * 2];
        let range = seed_ranges[i * 2 + 1];
        for seed in start..(start + range) {
            min_location = cmp::min(min_location, solve_part1(&(vec![seed], maps.clone())));
        }
    }
    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 35);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 46);
    }
}
