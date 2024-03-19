use parse::{GridPos, Instruction};
use std::collections::{HashSet, VecDeque};

mod parse;

const INPUT: &str = include_str!("../input.txt");

struct MyApp {
    instructions: VecDeque<Instruction>,
    head: GridPos,
    tail: GridPos,
    tail_visited: HashSet<GridPos>,
}

impl MyApp {
    fn new() -> Self {
        let instructions = INPUT.lines().map(Instruction::parse).collect();

        Self {
            instructions,
            head: GridPos { x: 0, y: 0 },
            tail: GridPos { x: 0, y: 0 },
            tail_visited: Default::default(),
        }
    }

    fn update_state(&mut self) {
        let Some(instruction) = self.instructions.front_mut() else {
            return;
        };

        self.head += instruction.dir.delta();

        let diff = self.head - self.tail;

        let (dx, dy) = match (diff.x, diff.y) {
            // overlapping
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            _ => panic!("unhandled case: tail - head = {diff:?}"),
        };

        self.tail.x += dx;
        self.tail.y += dy;
        self.tail_visited.insert(self.tail);

        instruction.dist -= 1;

        if instruction.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

fn main() {
    let mut app = MyApp::new();

    while !app.instructions.is_empty() {
        app.update_state();
    }

    println!("{:?}", app.tail_visited.len());
}

fn part_1(input: &str) -> usize {
    unimplemented!()
}

fn part_2(input: &str) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 13);
    }
}
