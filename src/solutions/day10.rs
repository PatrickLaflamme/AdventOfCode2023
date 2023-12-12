use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PIPES: HashMap<char, (Direction, Direction)> = {
        let mut map = HashMap::with_capacity(6);
        map.insert('|', (Direction::North, Direction::South));
        map.insert('7', (Direction::West, Direction::South));
        map.insert('-', (Direction::West, Direction::East));
        map.insert('L', (Direction::North, Direction::East));
        map.insert('J', (Direction::North, Direction::West));
        map.insert('F', (Direction::South, Direction::East));
        map
    };
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[aoc_generator(day10)]
pub fn generator(raw_input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let map = raw_input
        .split("\n")
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c == &'S' {
                return ((i as isize, j as isize), map);
            }
        }
    }
    panic!("Could not find start point in:\n{}", raw_input)
}

fn get_bounding_box(readings: &((isize, isize), Vec<Vec<char>>)) -> Vec<(usize, usize)> {
    let (start, map) = readings;
    let first_elems = [
        (start.0 + 1, start.1, Direction::South),
        (start.0 - 1, start.1, Direction::North),
        (start.0, start.1 + 1, Direction::East),
        (start.0, start.1 - 1, Direction::West),
    ]
    .iter()
    .filter_map(|(y, x, required_direction)| {
        if x < &0 || y < &0 || y >= &(map.len() as isize) || x >= &(map[0].len() as isize) {
            return None;
        }
        if map[*y as usize][*x as usize] == '.' {
            return None;
        }
        let pipe = PIPES.get(&map[*y as usize][*x as usize]).unwrap();
        if &pipe.0 == &required_direction.opposite() {
            return Some((*y, *x, pipe.0));
        }
        if &pipe.1 == &required_direction.opposite() {
            return Some((*y, *x, pipe.1));
        }
        return None;
    })
    .collect::<Vec<_>>();
    let mut bounding_box = Vec::<(usize, usize)>::new();
    bounding_box.push((start.0 as usize, start.1 as usize));
    let mut elems = first_elems.clone();
    while elems[0].0 != elems[1].0 || elems[0].1 != elems[1].1 {
        for i in 0..2 {
            bounding_box.push((elems[i].0 as usize, elems[i].1 as usize));
            let first_pipe = PIPES
                .get(&map[elems[i].0 as usize][elems[i].1 as usize])
                .unwrap();
            let next_direction = if first_pipe.0 == elems[i].2 {
                first_pipe.1
            } else {
                first_pipe.0
            };
            elems[i] = match next_direction {
                Direction::South => (elems[i].0 + 1, elems[i].1, next_direction.opposite()),
                Direction::North => (elems[i].0 - 1, elems[i].1, next_direction.opposite()),
                Direction::East => (elems[i].0, elems[i].1 + 1, next_direction.opposite()),
                Direction::West => (elems[i].0, elems[i].1 - 1, next_direction.opposite()),
            };
        }
    }
    bounding_box
}

#[aoc(day10, part1)]
pub fn solve_part1(readings: &((isize, isize), Vec<Vec<char>>)) -> usize {
    get_bounding_box(readings).len() / 2
}

#[aoc(day10, part2)]
pub fn solve_part2(readings: &((isize, isize), Vec<Vec<char>>)) -> usize {
    let bounding_box = get_bounding_box(readings)
        .iter()
        .cloned()
        .collect::<HashSet<(usize, usize)>>();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE4: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const EXAMPLE5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_solve_part1a() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 4);
    }

    #[test]
    fn test_solve_part1b() {
        let example = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 8);
    }

    #[test]
    fn test_solve_part2a() {
        let example = generator(&EXAMPLE3);
        assert_eq!(solve_part2(&example), 4);
    }

    #[test]
    fn test_solve_part2b() {
        let example = generator(&EXAMPLE4);
        assert_eq!(solve_part2(&example), 4);
    }

    #[test]
    fn test_solve_part2c() {
        let example = generator(&EXAMPLE5);
        assert_eq!(solve_part2(&example), 8);
    }

    #[test]
    fn test_solve_part2d() {
        let example = generator(&EXAMPLE6);
        assert_eq!(solve_part2(&example), 10);
    }
}
