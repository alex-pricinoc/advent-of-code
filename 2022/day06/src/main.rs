use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 1: {}", part_2(INPUT));
}

fn part_1(data: &str) -> i32 {
    let index = data
        .as_bytes()
        .windows(4)
        .position(has_unique_elements)
        .unwrap();

    index as i32 + 4
}

fn part_2(data: &str) -> i32 {
    let index = data
        .as_bytes()
        .windows(14)
        .position(has_unique_elements)
        .unwrap();

    index as i32 + 14
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(|x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
