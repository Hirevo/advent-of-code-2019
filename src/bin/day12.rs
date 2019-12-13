#[macro_use]
extern crate derive_more;

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use std::fmt;
use std::fmt::Display;

pub static INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign)]
pub struct Vec3(isize, isize, isize);

impl Vec3 {
    pub fn new(x: isize, y: isize, z: isize) -> Vec3 {
        Vec3(x, y, z)
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::new(0, 0, 0)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={0}, y={1}, z={2}>", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Moon {
    pub pos: Vec3,
    pub vel: Vec3,
}

impl Moon {
    pub fn new(x: isize, y: isize, z: isize) -> Moon {
        Moon {
            pos: Vec3::new(x, y, z),
            vel: Vec3::default(),
        }
    }

    pub fn potential_energy(&self) -> isize {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    pub fn kinetic_energy(&self) -> isize {
        self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()
    }

    pub fn total_energy(&self) -> isize {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl Default for Moon {
    fn default() -> Moon {
        Moon::new(0, 0, 0)
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos={0}, vel={1}", self.pos, self.vel)
    }
}

impl FromStr for Moon {
    type Err = ();

    fn from_str(input: &str) -> Result<Moon, Self::Err> {
        let input = input.trim();
        let input = &input[1..(input.len() - 1)];
        let mut moon = Moon::default();
        for unit in input.split(',') {
            let mut iter = unit.split('=');
            let name = iter.next().ok_or(())?.trim();
            let value = {
                let value = iter.next().ok_or(())?;
                let value = value.trim();
                value.parse::<isize>().map_err(|_| ())?
            };
            match name {
                "x" => moon.pos.0 = value,
                "y" => moon.pos.1 = value,
                "z" => moon.pos.2 = value,
                _ => {}
            }
        }
        Ok(moon)
    }
}

fn find_cycle(mut values: Vec<(isize, isize)>) -> isize {
    let mut last = values.clone();
    let mut set = HashSet::new();
    for steps in 0..std::isize::MAX {
        let inserted = set.insert({
            let mut hasher = DefaultHasher::new();
            values.hash(&mut hasher);
            hasher.finish()
        });
        if !inserted {
            return steps;
        }
        for val in values.iter_mut() {
            for other in last.iter().copied() {
                if *val == other {
                    continue;
                }
                val.1 += match val.0.cmp(&other.0) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                };
            }
            val.0 += val.1;
        }
        last.copy_from_slice(values.as_slice());
    }

    0
}

// greatest common divisor
fn gcd(mut a: isize, mut b: isize) -> isize {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }

    b
}

// least common multiple
fn lcm(a: isize, b: isize) -> isize {
    (a * b).abs() / gcd(a, b)
}

fn main() {
    let originals = INPUT
        .trim()
        .split('\n')
        .flat_map(|line| line.parse::<Moon>().ok())
        .collect::<Vec<_>>();

    let mut last = originals.clone();
    let mut current = originals.clone();
    for _ in 0..1000 {
        for moon in current.iter_mut() {
            for other in last.iter().copied() {
                if *moon == other {
                    continue;
                }
                moon.vel.0 += match moon.pos.0.cmp(&other.pos.0) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                };
                moon.vel.1 += match moon.pos.1.cmp(&other.pos.1) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                };
                moon.vel.2 += match moon.pos.2.cmp(&other.pos.2) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                };
            }
            moon.pos += moon.vel;
        }
        last.copy_from_slice(current.as_slice());
    }

    let part1 = current
        .into_iter()
        .map(|moon| moon.total_energy())
        .sum::<isize>();
    println!("{0}", part1);

    let x_cycle = find_cycle(
        originals
            .iter()
            .map(|moon| (moon.pos.0, moon.vel.0))
            .collect(),
    );
    let y_cycle = find_cycle(
        originals
            .iter()
            .map(|moon| (moon.pos.1, moon.vel.1))
            .collect(),
    );
    let z_cycle = find_cycle(
        originals
            .iter()
            .map(|moon| (moon.pos.2, moon.vel.2))
            .collect(),
    );

    let part2 = lcm(lcm(x_cycle, y_cycle), find_cycle(z_cycle));
    println!("{}", part2);
}
