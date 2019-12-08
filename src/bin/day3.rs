#[macro_use]
extern crate derive_more;

use std::collections::HashMap;

use aoc_2019::Error;

pub static INPUT: &str = include_str!("../../inputs/day3.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Add, Sub, AddAssign, SubAssign)]
#[display(fmt = "({},{})", _0, _1)]
pub struct Point(isize, isize);

impl Point {
    pub fn origin() -> Point {
        Point::default()
    }

    pub fn manhattan_distance(self, other: Point) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Default for Point {
    fn default() -> Point {
        Point(0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Point {
        match dir {
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
            Direction::Up => Point(0, 1),
            Direction::Down => Point(0, -1),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut iter = INPUT.split('\n').take(2).map(|wire| {
        wire.split(',').flat_map(|instr| {
            let (dir, val) = {
                let mut iter = instr.chars();
                iter.next().map(|fst| (fst, iter.as_str()))
            }?;
            let val = val.parse::<isize>().ok()?;
            let dir = match dir {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!("invalid direction"),
            };
            Some((dir, val))
        })
    });

    let wire1 = iter.next().expect("wrong number of wires");
    let wire2 = iter.next().expect("wrong number of wires");

    // This will contain the smallest amount of steps for each wire to reach a given point.
    // Type of `visited`: `HashMap<(Option<usize>, Option<usize>)>`
    let mut visited = HashMap::new();

    let mut cursor = Point(0, 0);
    let mut steps = 0;
    for (dir, val) in wire1 {
        for _ in 0..val {
            cursor += dir.into();
            steps += 1;
            visited.entry(cursor).or_insert((Some(steps), None));
        }
    }

    let mut cursor = Point(0, 0);
    let mut steps = 0;
    for (dir, val) in wire2 {
        for _ in 0..val {
            cursor += dir.into();
            steps += 1;
            visited
                .entry(cursor)
                .and_modify(|val| {
                    val.1 = match val.1 {
                        Some(val) => Some(val),
                        None => Some(steps),
                    }
                })
                .or_insert((None, Some(steps)));
        }
    }

    let part1 = visited
        .iter()
        .filter(|(_, (v1, v2))| v1.is_some() && v2.is_some())
        .map(|(point, _)| point.manhattan_distance(Point::origin()))
        .min()
        .expect("found no intersections");
    println!("{0}", part1);

    let part2 = visited
        .iter()
        .filter(|(_, (v1, v2))| v1.is_some() && v2.is_some())
        .flat_map(|(_, &(v1, v2))| v1.into_iter().zip(v2.into_iter()))
        .map(|(v1, v2)| v1 + v2)
        .min()
        .expect("found no intersections");
    println!("{0}", part2);

    Ok(())
}
