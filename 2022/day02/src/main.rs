#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Game {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn score(a: Move, b: Move) -> i32 {
    use Game::*;
    use Move::*;

    let outcome = match (a, b) {
        (Rock, Paper) => Win,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Scissors, Paper) => Loss,
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Scissors, Scissors) => Draw,
    };

    outcome as i32 + b as i32
}

fn part1(input: &str) -> i32 {
    use Move::*;

    input
        .lines()
        .map(|g| {
            let (theirs, ours) = g.split_once(" ").unwrap();

            let theirs = match theirs {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("Unknown move"),
            };

            let ours = match ours {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => panic!("Unknown move"),
            };

            score(theirs, ours)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    use Game::*;
    use Move::*;

    input
        .lines()
        .map(|g| {
            let (theirs, ours) = g.split_once(" ").unwrap();

            let theirs = match theirs {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => panic!("Unknown move"),
            };

            let outcome = match ours {
                "X" => Loss,
                "Y" => Draw,
                "Z" => Win,
                _ => panic!("Unknown move"),
            };

            let ours = match (theirs, outcome) {
                (_, Draw) => theirs,
                (Rock, Loss) => Scissors,
                (Rock, Win) => Paper,
                (Paper, Loss) => Rock,
                (Paper, Win) => Scissors,
                (Scissors, Loss) => Paper,
                (Scissors, Win) => Rock,
            };

            score(theirs, ours)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works() {
        assert_eq!(15, part1(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(12, part2(INPUT))
    }
}
