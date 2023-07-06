use std::str::FromStr;
use std::time::Instant;

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

fn part_1(data: &str) -> String {
	let mut crates = data.parse::<Crates>().unwrap();

	crates.apply_instructions(false);

	crates.get_word()
}

fn part_2(data: &str) -> String {
	let mut crates = data.parse::<Crates>().unwrap();

	crates.apply_instructions(true);

	crates.get_word()
}

struct Crates {
	ship: Vec<Vec<char>>,
	instructions: Vec<Move>,
}

struct Move {
	amount: usize,
	from: usize,
	to: usize,
}

#[derive(Debug)]
struct ParseCratesError;

impl FromStr for Crates {
	type Err = ParseCratesError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (containers, instructions) = s.split_once("\n\n").unwrap();

		let mut ship: Vec<Vec<char>> = Vec::new();

		for line in containers.lines().rev().skip(1) {
			for (i, c) in line
				.chars()
				.enumerate()
				.filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
			{
				let stack = (i - 1) / 4;

				while ship.len() < stack + 1 {
					ship.push(Vec::new());
				}

				ship[stack].push(c);
			}
		}

		let instructions = instructions
			.lines()
			.map(|l| {
				let [amount, from, to]: [usize; 3] = l
					.split_whitespace()
					.flat_map(|x| x.parse())
					.collect::<Vec<_>>()
					.try_into()
					.unwrap();

				Move { amount, from, to }
			})
			.collect();

		Ok(Self { ship, instructions })
	}
}

impl Crates {
	fn apply_instructions(&mut self, multiple: bool) {
		for Move { amount, from, to } in &self.instructions {
			if !multiple {
				for _ in 0..*amount {
					let value = self.ship[from - 1].pop().expect("crate does not exist");
					self.ship[to - 1].push(value);
				}
			} else {
				let len = self.ship[from - 1].len();

				let mut values = self.ship[from - 1]
					.splice((len - amount)..len, [])
					.collect();

				self.ship[to - 1].append(&mut values);
			}
		}
	}

	fn get_word(&self) -> String {
		self.ship.iter().map(|x| x.iter().last().unwrap()).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &'static str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
	#[test]
	fn part_1_test() {
		assert_eq!(part_1(INPUT), "CMZ");
	}

	#[test]
	fn part_2_test() {
		assert_eq!(part_2(INPUT), "MCD");
	}
}
