use std::collections::HashMap;

pub static INPUT: &str = include_str!("../../inputs/day6.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrbitMap<'a>(HashMap<&'a str, &'a str>);

impl<'a> OrbitMap<'a> {
    pub fn new(map: HashMap<&'a str, &'a str>) -> OrbitMap<'a> {
        OrbitMap(map)
    }

    pub fn inner(&self) -> &HashMap<&'a str, &'a str> {
        &self.0
    }

    pub fn count_orbits(&self, name: &str) -> usize {
        let mut count = 0;
        let mut current = name;
        loop {
            match self.0.get(current) {
                Some(name) => {
                    current = name;
                    count += 1;
                }
                None => break count,
            }
        }
    }

    pub fn get_orbits(&self, name: &str) -> Vec<&str> {
        let mut output = Vec::<&str>::new();
        let mut current = name;
        loop {
            match self.0.get(current) {
                Some(name) => {
                    current = name;
                    output.push(name);
                }
                None => break output,
            }
        }
    }
}

fn main() {
    let orbit_map = OrbitMap::new(
        INPUT
            .split('\n')
            .flat_map(|line| {
                let mut iter = line.split(')');
                let fst = iter.next()?;
                let snd = iter.next()?;
                Some((snd, fst))
            })
            .collect(),
    );

    let part1: usize = orbit_map
        .inner()
        .iter()
        .map(|(k, _)| orbit_map.count_orbits(k))
        .sum();
    println!("{0}", part1);

    let part2: usize = {
        let orbits1 = orbit_map.get_orbits("YOU");
        let orbits2 = orbit_map.get_orbits("SAN");

        let common = orbits1
            .iter()
            .find(|el| orbits2.contains(el))
            .expect("no common node");

        let iter1 = orbits1.iter().position(|el| el == common).into_iter();
        let iter2 = orbits2.iter().position(|el| el == common).into_iter();

        iter1.chain(iter2).sum()
    };
    println!("{0}", part2);
}
