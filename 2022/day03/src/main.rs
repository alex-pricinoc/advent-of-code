fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(data: &str) -> i32 {
    data.trim()
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);

            let common = a.chars().find(|&c| b.contains(c));

            match common {
                Some(c @ 'a'..='z') => (c as i32) - 96,
                Some(c @ 'A'..='Z') => (c as i32) - 38,
                _ => 0,
            }
        })
        .sum()
}

fn part_2(data: &str) -> i32 {
    data.trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|sets| {
            let [a, b, c]: [_; 3] = sets.try_into().unwrap();

            let common = a
                .chars()
                .filter(|&k| b.contains(k))
                .find(|&k| c.contains(k));

            match common {
                Some(c @ 'a'..='z') => (c as i32) - 96,
                Some(c @ 'A'..='Z') => (c as i32) - 38,
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let data = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        assert_eq!(part_1(data), 157);
    }

    #[test]
    fn part_2_test() {
        let data = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

        assert_eq!(part_2(data), 70);
    }
}
