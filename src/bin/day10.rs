use std::collections::{HashMap, HashSet};

pub static INPUT: &str = include_str!("../../inputs/day10.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
    Empty,
    Asteroid,
}

impl Location {
    pub fn is_empty(self) -> bool {
        if let Location::Empty = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocationMap(Vec<Vec<Location>>);

impl LocationMap {
    pub fn new(map: Vec<Vec<Location>>) -> LocationMap {
        LocationMap(map)
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Location> {
        self.0.get(y).and_then(|line| line.get(x)).copied()
    }

    /// returns an iterator that yields the coordinates of all the asteroids in the map
    pub fn asteroids<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &loc)| ((x, y), loc)))
            .filter(|(_, loc)| !loc.is_empty())
            .map(|(coords, _)| coords)
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }

    b
}

fn main() {
    let input = LocationMap::new(
        INPUT
            .split('\n')
            .map(|line| {
                line.trim()
                    .chars()
                    .flat_map(|ch| match ch {
                        '.' => Some(Location::Empty),
                        '#' => Some(Location::Asteroid),
                        _ => None,
                    })
                    .collect()
            })
            .collect(),
    );
    // collect all asteroids
    let asteroids: Vec<_> = input.asteroids().collect();

    // find coordinates of the station and the number of asteroids it can see
    let ((x, y), most_seen) = asteroids
        .iter()
        .copied()
        .map(|(x, y)| {
            let seen = asteroids
                .iter()
                .copied()
                .filter(|&other| (x, y) != other)
                .map(|(ox, oy)| ((ox as isize) - (x as isize), (oy as isize) - (y as isize)))
                .fold(HashSet::<(isize, isize)>::new(), |mut seens, (ox, oy)| {
                    let g = gcd(ox, oy).abs();
                    seens.insert((ox / g, oy / g));
                    seens
                })
                .len();
            ((x, y), seen)
        })
        .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .expect("not enough asteroids");

    let part1 = most_seen;
    println!("{1:?} {0}", part1, (x, y));

    let part2 = {
        // get all asteroids other than the station and move to relative coordinates to the station
        let others = asteroids
            .iter()
            .copied()
            .filter(|&(ox, oy)| (ox, oy) != (x, y))
            .map(|(ox, oy)| ((ox as isize) - (x as isize), (oy as isize) - (y as isize)))
            .collect::<Vec<_>>();

        let mut per_axis = {
            let mut per_axis = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();

            // asteroids grouped by directions from station
            others.iter().copied().for_each(|(ox, oy)| {
                let g = gcd(ox, oy).abs();
                per_axis
                    .entry((ox / g, oy / g))
                    .and_modify(|v| v.push((ox, oy)))
                    .or_insert_with(|| vec![(ox, oy)]);
            });

            // sort asteroids by distance from station within each group
            per_axis
                .into_iter()
                .map(|(k, mut v)| {
                    v.sort_by_key(|&(x, y)| (x * x) + (y * y));
                    (k, v)
                })
                .collect::<Vec<_>>()
        };

        // sort group in vaporization order
        per_axis.sort_by(|&((x1, y1), _), &((x2, y2), _)| {
            // to get to the correct base axes, we do two things:
            // - swap x and y in atan2 call
            // - reverse the resulting ordering (radians go counter-clockwise, so we invert it)
            let angle1 = (x1 as f64).atan2(y1 as f64);
            let angle2 = (x2 as f64).atan2(y2 as f64);
            angle1.partial_cmp(&angle2).unwrap().reverse()
        });

        // transform each group to an iterator that will get pulled from once per laser round
        let mut per_axis: Vec<_> = per_axis
            .into_iter()
            .map(|(_, asteroids)| asteroids.into_iter())
            .collect();

        // pull one item from each iterator per loop round, until they are all exhausted
        let mut ordered = Vec::with_capacity(asteroids.len());
        loop {
            let mut still_one = false;
            for vs in per_axis.iter_mut() {
                if let Some(item) = vs.next() {
                    still_one = true;
                    ordered.push(item);
                }
            }
            if !still_one {
                break;
            }
        }

        // get 200th asteroid, go back to absolute coordinates and compute puzzle result
        ordered
            .get(199)
            .map(|(ox, oy)| (ox + (x as isize)) * 100 + (oy + (y as isize)))
            .expect("not enough asteroids")
    };
    println!("{0}", part2);
}
