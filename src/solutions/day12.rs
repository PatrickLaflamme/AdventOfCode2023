use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{EitherOrBoth, Itertools};
use std::ops::Range;

#[aoc_generator(day12)]
pub fn generator(raw_input: &str) -> Vec<(usize, Vec<Range<usize>>, Vec<usize>, Vec<usize>)> {
    raw_input
        .split("\n")
        .map(|l| {
            let Some((map_string, requirements_string)) = l.split(" ").collect_tuple() else {
                panic!("Invalid input {}", l);
            };
            let len = map_string.len();
            let requirements = requirements_string
                .split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            let mut map = Vec::<Range<usize>>::new();
            let mut options = Vec::<usize>::new();
            let mut cur_start: Option<usize> = None;
            for (i, c) in map_string.chars().enumerate() {
                match c {
                    '.' => {
                        if cur_start.is_some() {
                            map.push(cur_start.unwrap()..i);
                            cur_start = None;
                        }
                    }
                    '#' => {
                        if cur_start.is_none() {
                            cur_start = Some(i);
                        }
                    }
                    '?' => {
                        if cur_start.is_some() {
                            map.push(cur_start.unwrap()..i);
                            cur_start = None;
                        }
                        options.push(i);
                    }
                    _ => panic!("Invalid character found: {}", c),
                }
            }
            if cur_start.is_some() {
                map.push(cur_start.unwrap()..map_string.len());
            }
            (len, map, options, requirements)
        })
        .collect::<Vec<_>>()
}

fn solve(readings: &[(usize, Vec<Range<usize>>, Vec<usize>, Vec<usize>)]) -> usize {
    let mut stack = readings.to_vec();
    let mut variations = 0;
    while stack.len() > 0 {
        let (_, ranges, options, requirements) = match stack.pop() {
            Some(e) => e,
            None => break,
        };
        if options.is_empty() {
            let mut complete = 1;
            for zip in ranges.iter().zip_longest(requirements.clone()) {
                match zip {
                    EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => {
                        complete = 0;
                        break;
                    }
                    EitherOrBoth::Both(range, req) => {
                        if range.len() != req {
                            complete = 0;
                            break;
                        }
                    }
                }
            }
            variations += complete;
            continue;
        }
        let next_option = match options.first() {
            Some(next) => next,
            None => panic!("should never get here!"),
        };
        //todo!("The below needs some thinking through. this is to short circuit if the early requirements don't line up.");
        let mut new_range = *next_option..(next_option + 1);
        for range in ranges.iter() {
            if range.start > 0 && (range.start - 1) == *next_option {
                new_range = *next_option..range.end;
                break;
            } else if range.end == *next_option {
                new_range = range.start..(next_option + 1);
                break;
            }
        }
        let new_options = options[1..].to_vec();
        let mut new_ranges = Vec::<Range<usize>>::new();
        if new_range.len() > 1 {
            new_ranges = ranges
                .iter()
                .filter(|r| r.start != new_range.start && r.end != new_range.end)
                .cloned()
                .collect::<Vec<_>>();
        } else {
            new_ranges = ranges.clone();
        }
        new_ranges.push(new_range);
        new_ranges.sort_by(|a, b| a.start.cmp(&b.start));
        new_ranges = new_ranges
            .iter()
            .fold(Vec::<Range<usize>>::new(), |acc, r| match acc.last() {
                Some(e) => {
                    if e.end == r.start {
                        let mut new = acc[0..(acc.len() - 1)].to_vec();
                        new.extend([(e.start..r.end)]);
                        new
                    } else {
                        let mut new = acc.clone();
                        new.extend(vec![r.clone()]);
                        new
                    }
                }
                None => vec![r.clone()],
            });
        stack.push((0, new_ranges, new_options.clone(), requirements.clone()));
        stack.push((0, ranges, new_options, requirements));
    }
    variations
}

#[aoc(day12, part1)]
pub fn solve_part1(readings: &[(usize, Vec<Range<usize>>, Vec<usize>, Vec<usize>)]) -> usize {
    solve(readings)
}

#[aoc(day12, part2)]
pub fn solve_part2(readings: &[(usize, Vec<Range<usize>>, Vec<usize>, Vec<usize>)]) -> usize {
    let expanded_readings = readings
        .iter()
        .map(|(len, ranges, options, reqs)| {
            let expanded_reqs = reqs.repeat(5);
            let expanded_options = (0..5)
                .flat_map(|r| {
                    let mut options_section = options.iter().map(|o| o + r).collect::<Vec<_>>();
                    if r < 4 {
                        options_section.push(len * (r + 1));
                    }
                    options_section
                })
                .collect::<Vec<_>>();
            let expanded_ranges = (0..5)
                .flat_map(|s| {
                    ranges
                        .iter()
                        .map(move |r| (r.start + len * s)..(r.end + len * s))
                })
                .collect::<Vec<_>>();
            (*len, expanded_ranges, expanded_options, expanded_reqs)
        })
        .collect::<Vec<_>>();
    solve(&expanded_readings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 21);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 525152);
    }
}

