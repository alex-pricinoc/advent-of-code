use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 1: {}", part_2(INPUT));
}

fn part_1(data: &str) -> usize {
    find_marker(data, 4).unwrap()
}

fn part_2(data: &str) -> usize {
    find_marker(data, 14).unwrap()
}

fn find_marker(input: &str, size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == size)
        .map(|pos| pos + size)
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
