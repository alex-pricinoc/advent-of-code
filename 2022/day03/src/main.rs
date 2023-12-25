use day03::Item;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> day03::Result<()> {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input)?);
    println!("Part 2: {}", part_2(input)?);

    Ok(())
}

fn part_1(data: &str) -> day03::Result<usize> {
    let sum = data
        .lines()
        .map(|line| -> day03::Result<_> {
            let (first, second) = line.split_at(line.len() / 2);

            let first_items = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;

            let second_items = second
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;

            let common = first_items
                .iter()
                .find(|i| second_items.contains(i))
                .map(|i| i.score())
                .ok_or("compartments have no items in common")?;

            Ok(common)
        })
        .sum::<Result<usize, _>>()?;

    Ok(sum)
}

fn part_2(data: &str) -> day03::Result<usize> {
    let rucksacks = data.lines().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let sum = itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .find(|&i| b.contains(i) && c.contains(i))
                    .map(|i| i.score())
                    .unwrap_or_default()
            })
            .sum()
    })?;

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let data = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(part_1(data).unwrap(), 157);
    }

    #[test]
    fn part_2_test() {
        let data = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(part_2(data).unwrap(), 70);
    }
}
