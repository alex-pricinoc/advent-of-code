use day08::Forest;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let answer = solve(INPUT);

    println!("Part 1: {}", answer.0);
    println!("Part 2: {}", answer.1);
}

fn solve(input: &str) -> (i32, i32) {
    let forest = input.parse::<Forest>().unwrap();
    let (width, height) = (forest.width(), forest.height());

    let mut count = 0;

    let mut highest_score = 0;

    for x in 0..width {
        for y in 0..height {
            let (visible, score) = forest.tree_visible(x, y);

            if visible {
                count += 1;
            }

            highest_score = highest_score.max(score);
        }
    }

    (count, highest_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn part_1_test() {
        assert_eq!(solve(INPUT).0, 21);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(solve(INPUT).1, 8);
    }
}
