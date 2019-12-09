pub static INPUT: (isize, isize) = (240_920, 789_857);

fn is_valid_candidate(mut candidate: isize) -> bool {
    let d6 = candidate % 10;
    candidate /= 10;
    let d5 = candidate % 10;
    candidate /= 10;
    let d4 = candidate % 10;
    candidate /= 10;
    let d3 = candidate % 10;
    candidate /= 10;
    let d2 = candidate % 10;
    candidate /= 10;
    let d1 = candidate % 10;

    let ds = [d1, d2, d3, d4, d5, d6];

    let cond1 = ds.windows(2).all(|w| w[0] <= w[1]);
    let cond2 = ds.windows(2).any(|w| w[0] == w[1]);

    cond1 && cond2
}

fn is_valid_candidate2(mut candidate: isize) -> bool {
    let d6 = candidate % 10;
    candidate /= 10;
    let d5 = candidate % 10;
    candidate /= 10;
    let d4 = candidate % 10;
    candidate /= 10;
    let d3 = candidate % 10;
    candidate /= 10;
    let d2 = candidate % 10;
    candidate /= 10;
    let d1 = candidate % 10;

    let ds = [d1, d2, d3, d4, d5, d6];
    let ds2 = [-1, d1, d2, d3, d4, d5, d6, -1];

    let cond1 = ds.windows(2).all(|w| w[0] <= w[1]);
    let cond2 = ds2
        .windows(4)
        .any(|w| w[0] != w[1] && w[1] == w[2] && w[2] != w[3]);

    cond1 && cond2
}

fn main() {
    let (low, high) = INPUT;

    let part1 = (low..=high)
        .filter(|value| is_valid_candidate(*value))
        .count();
    println!("{0}", part1);

    let part2 = (low..=high)
        .filter(|value| is_valid_candidate2(*value))
        .count();
    println!("{0}", part2);
}
