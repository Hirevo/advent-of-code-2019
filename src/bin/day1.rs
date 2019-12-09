pub static INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let part1: u64 = INPUT
        .split('\n')
        .flat_map(|text| text.parse::<u64>().ok())
        .map(|mass| (mass / 3).saturating_sub(2))
        .sum();

    let part2: u64 = INPUT
        .split('\n')
        .flat_map(|text| text.parse::<u64>().ok())
        .map(|mut mass| {
            let mut total = 0;
            while mass > 0 {
                mass = (mass / 3).saturating_sub(2);
                total += mass;
            }
            total
        })
        .sum();

    println!("{0}", part1);
    println!("{0}", part2);
}
