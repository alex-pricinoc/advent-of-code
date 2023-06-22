fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(Assignment::parse)
        .filter(|a| {
            a.left.0 <= a.right.0 && a.left.1 >= a.right.1
                || a.right.0 <= a.left.0 && a.right.1 >= a.left.1
        })
        .count()
        .try_into()
        .unwrap()
}
fn part_2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(Assignment::parse)
        .filter(|a| a.right.0 <= a.left.1)
        .count()
        .try_into()
        .unwrap()
}

#[derive(Debug)]
struct Assignment {
    left: (i32, i32),
    right: (i32, i32),
}

impl Assignment {
    fn parse(s: &str) -> Self {
        let (a, b) = s.split_once(",").unwrap();

        let pair = a.split_once("-").unwrap();
        let first = (pair.0.parse().unwrap(), pair.1.parse().unwrap());

        let pair = b.split_once("-").unwrap();
        let second = (pair.0.parse().unwrap(), pair.1.parse().unwrap());

        let (left, right) = if first.0 < second.0 {
            (first, second)
        } else {
            (second, first)
        };

        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let data = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
          "#;

        assert_eq!(part_1(data), 2);
    }

    #[test]
    fn part_2_test() {
        let data = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
              "#;

        assert_eq!(part_2(data), 4);
    }
}
