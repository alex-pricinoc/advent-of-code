use day05::{Instruction, Piles};
use std::time::Instant;

fn part_1(data: &str) -> String {
    let (containers, instructions) = data.split_once("\n\n").unwrap();

    let mut piles = containers.parse::<Piles>().unwrap();

    // println!("{piles:?}");

    for ins in instructions
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
    {
        // println!("{ins:?}");
        piles.apply(ins, false);
        // println!("{piles:?}");
    }

    piles.word()
}

fn part_2(data: &str) -> String {
    let (containers, instructions) = data.split_once("\n\n").unwrap();

    let mut piles = containers.parse::<Piles>().unwrap();

    // println!("{piles:?}");

    for ins in instructions
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
    {
        // println!("{ins:?}");
        piles.apply(ins, true);
        // println!("{piles:?}");
    }

    piles.word()
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    let part_1_result = part_1(input);
    let duration = now.elapsed();
    println!("Part 1: {:<20}(took: {:>12?})", part_1_result, duration);

    let now = Instant::now();

    let part_2_result = part_2(input);
    let duration = now.elapsed();
    println!("Part 2: {:<20}(took: {:>12?})", part_2_result, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), "CMZ");
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), "MCD");
    }
}
