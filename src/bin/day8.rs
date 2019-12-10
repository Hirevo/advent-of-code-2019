use itertools::Itertools;

pub static INPUT: &str = include_str!("../../inputs/day8.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    White = 1,
    Transparent = 2,
}

fn main() {
    let input = INPUT
        .split("")
        .flat_map(|digit| match digit.parse::<u8>().ok()? {
            0 => Some(Color::Black),
            1 => Some(Color::White),
            2 => Some(Color::Transparent),
            _ => None,
        })
        .collect::<Vec<_>>();
    let len = 25 * 6;

    let part1 = input
        .chunks_exact(len)
        .min_by(|l1, l2| {
            let c1 = l1.iter().filter(|&&el| el == Color::Black).count();
            let c2 = l2.iter().filter(|&&el| el == Color::Black).count();
            c1.cmp(&c2)
        })
        .map(|layer| {
            layer
                .iter()
                .copied()
                .fold((0usize, 0usize), |(ones, twos), el| match el {
                    Color::White => (ones + 1, twos),
                    Color::Transparent => (ones, twos + 1),
                    _ => (ones, twos),
                })
        })
        .map(|(ones, twos)| ones * twos)
        .expect("no layers at all");
    println!("{0}", part1);

    let _part2 = input
        .chunks_exact(len)
        .rev()
        .fold(vec![Color::Transparent; len], |mut acc, layer| {
            let iter = acc.iter_mut().zip(layer.iter().copied());
            for (out, val) in iter {
                if let Color::Transparent = val {
                } else {
                    *out = val;
                }
            }
            acc
        })
        .into_iter()
        .map(|el| match el {
            Color::White => '1',
            Color::Black | Color::Transparent => ' ',
        })
        .chunks(25)
        .into_iter()
        .map(|it| it.collect::<String>())
        .for_each(|line| println!("{}", line));
}
