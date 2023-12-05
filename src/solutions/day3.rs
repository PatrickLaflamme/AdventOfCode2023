use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(raw_input: &str) -> (Vec<(usize, usize, usize, usize)>, Vec<(usize, usize, char)>) {
    let mut partnos = Vec::<(usize, usize, usize, usize)>::new();
    let mut symbols = Vec::<(usize, usize, char)>::new();
    let mut num = Vec::<char>::new();
    for (i, line) in raw_input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num.push(c);
                continue;
            } 
            if num.len() > 0 {
                partnos.push((i, j - num.len(), j - 1, num.iter().collect::<String>().parse().unwrap()));
                num.clear();
            } 
            if c != '.' {
                symbols.push((i, j, c));
            }

        }
        if num.len() > 0 {
            partnos.push((i, line.len() - num.len() - 1, line.len() - 1, num.iter().collect::<String>().parse().unwrap()));
        }
        num.clear();
    }
    (partnos, symbols)
}

#[aoc(day3, part1)]
pub fn solve_part1(readings: &(Vec<(usize, usize, usize, usize)>, Vec<(usize, usize, char)>)) -> usize {
    let (numbers, symbols) = readings;
    numbers.iter().filter(|(y, x_s, x_e, _)| {
        let relevant_symbols = symbols.iter().filter(|(symbol_y, symbol_x, _)| {
            if symbol_y + 1 < *y || y + 1 < *symbol_y {
                return false;
            }
            if symbol_x + 1 < *x_s || x_e + 1 < *symbol_x {
                return false;
            }
            true
        }).collect::<Vec<_>>();
        relevant_symbols.len() > 0
    }).map(|(_, _, _, n)| { n })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(readings:  &(Vec<(usize, usize, usize, usize)>, Vec<(usize, usize, char)>)) -> usize {
    let (numbers, symbols) = readings;
    symbols.iter().map(|(y, x, symbol)| {
        if symbol != &'*' {
            return 0;
        }
        let relevant_numbers: Vec<usize> = numbers.iter().filter(|(y_n, x_s, x_e, _)| {
            if (y_n.clone() as isize - y.clone() as isize).abs() > 1 {
                return false;
            }
            let mut start = 0;
            if x_s > &0 {
                start = x_s - 1;
            }
            if x < &start || (x_e + 1) < *x {
                return false;
            }
            return true;
        }).map(|(_, _, _, n)| { n.clone() }).collect();
        if relevant_numbers.len() != 2 {
            return 0;
        }
        return relevant_numbers[0] * relevant_numbers[1]
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    // #[test]
    // fn test_generator() {
    //     let expected = vec![0; 1];
    //     assert_eq!(generator(&EXAMPLE), expected);
    // }

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 4361);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 467835);
    }
}
